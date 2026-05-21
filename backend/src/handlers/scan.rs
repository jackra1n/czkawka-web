use std::sync::Arc;
use std::sync::Mutex;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tokio::task::spawn_blocking;
use uuid::Uuid;

use crate::models::{AppState, ScanResponse, ScanState, ScanStatusResponse};
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

    let state_clone = Arc::clone(&state);
    let scan_id_clone = scan_id.clone();
    let stop_flag_clone = Arc::clone(&stop_flag);

    spawn_blocking(move || {
        log::info!("Starting scan {scan_id_clone} for tool {tool_id}");
        let result = run_scan(request, shared_progress, stop_flag_clone.clone());

        if stop_flag_clone.load(std::sync::atomic::Ordering::Relaxed) {
            log::info!("Scan {scan_id_clone} was cancelled");
            {
                let mut scans = state_clone.scans.lock().unwrap();
                scans.insert(scan_id_clone.clone(), ScanState::Cancelled);
            }

            let mut persistent = state_clone.persistent.lock().unwrap();
            let tool = persistent.tools.entry(tool_id).or_default();
            if tool.scan_id.as_ref() == Some(&scan_id_clone) {
                tool.status = "idle".to_string();
                tool.scan_id = None;
                tool.results = None;
                tool.error = None;
                if let Err(e) = state::save_state(&state_clone.state_path, &persistent) {
                    log::error!("Failed to save state at scan cancellation: {e}");
                }
            }
            return;
        }

        match result {
            Ok(results) => {
                log::info!(
                    "Scan {scan_id_clone} completed with {} groups",
                    results.total_groups
                );
                {
                    let mut scans = state_clone.scans.lock().unwrap();
                    scans.insert(scan_id_clone.clone(), ScanState::Completed(results.clone()));
                }

                let mut persistent = state_clone.persistent.lock().unwrap();
                let tool = persistent.tools.entry(tool_id.clone()).or_default();
                if tool.scan_id.as_ref() == Some(&scan_id_clone) {
                    tool.status = "completed".to_string();
                    tool.results = Some(results);
                    tool.scan_id = None;
                    tool.error = None;
                    if let Err(e) = state::save_state(&state_clone.state_path, &persistent) {
                        log::error!("Failed to save state at scan completion: {e}");
                    }
                }
            }
            Err(e) => {
                log::info!("Scan {scan_id_clone} error: {e}");
                {
                    let mut scans = state_clone.scans.lock().unwrap();
                    scans.insert(scan_id_clone.clone(), ScanState::Error(e.clone()));
                }

                let mut persistent = state_clone.persistent.lock().unwrap();
                let tool = persistent.tools.entry(tool_id).or_default();
                if tool.scan_id.as_ref() == Some(&scan_id_clone) {
                    tool.status = "error".to_string();
                    tool.error = Some(e);
                    tool.scan_id = None;
                    tool.results = None;
                    if let Err(e) = state::save_state(&state_clone.state_path, &persistent) {
                        log::error!("Failed to save state at scan error: {e}");
                    }
                }
            }
        }
    });

    (
        StatusCode::CREATED,
        Json(ScanResponse {
            id: scan_id,
            status: "running".to_string(),
        }),
    )
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
