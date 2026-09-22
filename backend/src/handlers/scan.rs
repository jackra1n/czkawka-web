use std::sync::Arc;
use std::sync::Mutex;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tokio::task::{JoinHandle, spawn_blocking};
use uuid::Uuid;

use crate::models::{AppState, ScanResponse, ScanResults, ScanState, ScanStatusResponse};
use crate::scan::run_scan;
use crate::state;

pub async fn start_scan(
    State(state): State<Arc<AppState>>,
    Json(request): Json<crate::models::ScanRequest>,
) -> (StatusCode, Json<ScanResponse>) {
    let scan_id = Uuid::new_v4().to_string();
    let tool_id = request.tool_id.clone();

    let shared_progress = Arc::new(Mutex::new(None));
    let stop_flag = Arc::new(std::sync::atomic::AtomicBool::new(false));

    {
        let mut scans = state.scans.lock().unwrap();
        scans.insert(
            scan_id.clone(),
            ScanState::Running {
                progress: Arc::clone(&shared_progress),
                stop_flag: Arc::clone(&stop_flag),
            },
        );
    }

    {
        let mut persistent = state.persistent.lock().unwrap();
        let tool = persistent.tools.entry(tool_id.clone()).or_default();
        tool.status = "running".to_string();
        tool.scan_id = Some(scan_id.clone());
        tool.results = None;
        tool.error = None;
        if let Err(e) = state::save_state(&state.state_path, &persistent) {
            log::error!("Failed to save state at scan start: {e}");
        }
    }

    let worker_stop_flag = Arc::clone(&stop_flag);
    log::info!("Starting scan {scan_id} for tool {tool_id}");
    let worker = spawn_blocking(move || run_scan(request, shared_progress, worker_stop_flag));
    tokio::spawn(finish_scan(
        state,
        scan_id.clone(),
        tool_id,
        stop_flag,
        worker,
    ));

    (
        StatusCode::CREATED,
        Json(ScanResponse {
            id: scan_id,
            status: "running".to_string(),
        }),
    )
}

async fn finish_scan(
    state: Arc<AppState>,
    scan_id: String,
    tool_id: String,
    stop_flag: Arc<std::sync::atomic::AtomicBool>,
    worker: JoinHandle<Result<ScanResults, String>>,
) {
    let result = match worker.await {
        Ok(result) => result,
        Err(error) => {
            log::error!("Scan {scan_id} worker failed: {error}");
            Err("Scan worker failed unexpectedly. See server logs for details.".to_string())
        }
    };

    if stop_flag.load(std::sync::atomic::Ordering::Relaxed) {
        log::info!("Scan {scan_id} was cancelled");
        {
            let mut scans = state.scans.lock().unwrap();
            scans.insert(scan_id.clone(), ScanState::Cancelled);
        }

        let mut persistent = state.persistent.lock().unwrap();
        let tool = persistent.tools.entry(tool_id).or_default();
        if tool.scan_id.as_ref() == Some(&scan_id) {
            tool.status = "idle".to_string();
            tool.scan_id = None;
            tool.results = None;
            tool.error = None;
            if let Err(e) = state::save_state(&state.state_path, &persistent) {
                log::error!("Failed to save state at scan cancellation: {e}");
            }
        }
        return;
    }

    match result {
        Ok(results) => {
            log::info!(
                "Scan {scan_id} completed with {} groups",
                results.total_groups
            );
            {
                let mut scans = state.scans.lock().unwrap();
                scans.insert(scan_id.clone(), ScanState::Completed(results.clone()));
            }

            let mut persistent = state.persistent.lock().unwrap();
            let tool = persistent.tools.entry(tool_id).or_default();
            if tool.scan_id.as_ref() == Some(&scan_id) {
                tool.status = "completed".to_string();
                tool.results = Some(results);
                tool.scan_id = None;
                tool.error = None;
                if let Err(e) = state::save_state(&state.state_path, &persistent) {
                    log::error!("Failed to save state at scan completion: {e}");
                }
            }
        }
        Err(e) => {
            log::error!("Scan {scan_id} error: {e}");
            {
                let mut scans = state.scans.lock().unwrap();
                scans.insert(scan_id.clone(), ScanState::Error(e.clone()));
            }

            let mut persistent = state.persistent.lock().unwrap();
            let tool = persistent.tools.entry(tool_id).or_default();
            if tool.scan_id.as_ref() == Some(&scan_id) {
                tool.status = "error".to_string();
                tool.error = Some(e);
                tool.scan_id = None;
                tool.results = None;
                if let Err(e) = state::save_state(&state.state_path, &persistent) {
                    log::error!("Failed to save state at scan error: {e}");
                }
            }
        }
    }
}

pub async fn get_scan_status(
    State(state): State<Arc<AppState>>,
    Path(scan_id): Path<String>,
) -> (StatusCode, Json<ScanStatusResponse>) {
    let scans = state.scans.lock().unwrap();

    match scans.get(&scan_id) {
        Some(ScanState::Running { progress, .. }) => {
            let progress_data = progress.lock().unwrap().clone();
            (
                StatusCode::OK,
                Json(ScanStatusResponse {
                    id: scan_id,
                    status: "running".to_string(),
                    progress: progress_data,
                    results: None,
                    error: None,
                }),
            )
        }
        Some(ScanState::Completed(results)) => (
            StatusCode::OK,
            Json(ScanStatusResponse {
                id: scan_id,
                status: "completed".to_string(),
                progress: None,
                results: Some(results.clone()),
                error: None,
            }),
        ),
        Some(ScanState::Cancelled) => (
            StatusCode::OK,
            Json(ScanStatusResponse {
                id: scan_id,
                status: "cancelled".to_string(),
                progress: None,
                results: None,
                error: None,
            }),
        ),
        Some(ScanState::Error(err)) => (
            StatusCode::OK,
            Json(ScanStatusResponse {
                id: scan_id,
                status: "error".to_string(),
                progress: None,
                results: None,
                error: Some(err.clone()),
            }),
        ),
        None => (
            StatusCode::NOT_FOUND,
            Json(ScanStatusResponse {
                id: scan_id,
                status: "not_found".to_string(),
                progress: None,
                results: None,
                error: None,
            }),
        ),
    }
}

pub async fn cancel_scan(
    State(state): State<Arc<AppState>>,
    Path(scan_id): Path<String>,
) -> StatusCode {
    let scans = state.scans.lock().unwrap();
    if let Some(ScanState::Running { stop_flag, .. }) = scans.get(&scan_id) {
        stop_flag.store(true, std::sync::atomic::Ordering::Relaxed);
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;
    use std::time::Duration;

    fn test_state() -> Arc<AppState> {
        let directory = std::env::temp_dir().join(format!("czkawka-scan-test-{}", Uuid::new_v4()));
        std::fs::create_dir(&directory).expect("create test directory");
        Arc::new(AppState {
            scans: Default::default(),
            persistent: Default::default(),
            state_path: directory.join("state.json"),
        })
    }

    #[tokio::test]
    async fn content_scan_completes_without_audio_files() {
        let state = test_state();
        let directory = state.state_path.parent().expect("test directory");
        let request = serde_json::from_value(serde_json::json!({
            "tool_id": "same-music",
            "music_check_type": "content",
            "directories": [directory],
        }))
        .expect("valid scan request");
        let (_, Json(started)) = start_scan(State(Arc::clone(&state)), Json(request)).await;

        let response = tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                let (_, Json(response)) =
                    get_scan_status(State(Arc::clone(&state)), Path(started.id.clone())).await;
                if response.status != "running" {
                    break response;
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .expect("scan must reach a terminal state");

        assert_eq!(response.status, "completed", "{:?}", response.error);
        assert_eq!(response.results.expect("scan results").total_items, 0);
        std::fs::remove_dir_all(directory).expect("remove test directory");
    }

    #[tokio::test]
    async fn panicking_worker_publishes_and_persists_error() {
        let state = test_state();
        let scan_id = Uuid::new_v4().to_string();
        let stop_flag = Arc::new(AtomicBool::new(false));
        state.scans.lock().expect("scan state").insert(
            scan_id.clone(),
            ScanState::Running {
                progress: Default::default(),
                stop_flag: Arc::clone(&stop_flag),
            },
        );
        state
            .persistent
            .lock()
            .expect("persistent state")
            .tools
            .insert(
                "same-music".to_string(),
                crate::state::ToolState {
                    status: "running".to_string(),
                    scan_id: Some(scan_id.clone()),
                    ..Default::default()
                },
            );

        let worker = spawn_blocking(|| panic!("simulated scanner panic"));
        finish_scan(
            Arc::clone(&state),
            scan_id.clone(),
            "same-music".to_string(),
            stop_flag,
            worker,
        )
        .await;

        let (_, Json(response)) = get_scan_status(State(Arc::clone(&state)), Path(scan_id)).await;
        assert_eq!(response.status, "error");
        assert!(response.error.is_some());
        assert!(response.results.is_none());
        let saved: crate::state::AppPersistentState =
            serde_json::from_slice(&std::fs::read(&state.state_path).expect("saved state"))
                .expect("valid saved state");
        let tool = &saved.tools["same-music"];
        assert_eq!(tool.status, "error");
        assert_eq!(tool.error, response.error);
        assert!(tool.scan_id.is_none());
        assert!(tool.results.is_none());
        std::fs::remove_dir_all(state.state_path.parent().expect("test directory"))
            .expect("remove test directory");
    }
}
