use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tokio::task::spawn_blocking;
use uuid::Uuid;

use crate::models::{
    AppState, ScanResponse, ScanState, ScanStatusResponse,
};
use crate::scan::run_scan;
use crate::state;

pub async fn start_scan(
    State(state): State<Arc<AppState>>,
    Json(request): Json<crate::models::ScanRequest>,
) -> (StatusCode, Json<ScanResponse>) {
    let scan_id = Uuid::new_v4().to_string();
    let tool_id = request.tool_id.clone();

    {
        let mut scans = state.scans.lock().unwrap();
        scans.insert(scan_id.clone(), ScanState::Running);
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

    spawn_blocking(move || {
        log::info!("Starting scan {scan_id_clone} for tool {tool_id}");
        let result = run_scan(request);
        let mut scans = state_clone.scans.lock().unwrap();
        match result {
            Ok(results) => {
                log::info!(
                    "Scan {scan_id_clone} completed with {} groups",
                    results.total_groups
                );
                scans.insert(scan_id_clone.clone(), ScanState::Completed(results.clone()));

                let mut persistent = state_clone.persistent.lock().unwrap();
                let tool = persistent.tools.entry(tool_id.clone()).or_default();
                tool.status = "completed".to_string();
                tool.results = Some(results);
                tool.scan_id = None;
                tool.error = None;
                if let Err(e) = state::save_state(&state_clone.state_path, &persistent) {
                    log::error!("Failed to save state at scan completion: {e}");
                }
            }
            Err(e) => {
                log::info!("Scan {scan_id_clone} error: {e}");
                scans.insert(scan_id_clone.clone(), ScanState::Error(e.clone()));

                let mut persistent = state_clone.persistent.lock().unwrap();
                let tool = persistent.tools.entry(tool_id).or_default();
                tool.status = "error".to_string();
                tool.error = Some(e);
                tool.scan_id = None;
                tool.results = None;
                if let Err(e) = state::save_state(&state_clone.state_path, &persistent) {
                    log::error!("Failed to save state at scan error: {e}");
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
        Some(ScanState::Running) => (
            StatusCode::OK,
            Json(ScanStatusResponse {
                id: scan_id,
                status: "running".to_string(),
                results: None,
                error: None,
            }),
        ),
        Some(ScanState::Completed(results)) => (
            StatusCode::OK,
            Json(ScanStatusResponse {
                id: scan_id,
                status: "completed".to_string(),
                results: Some(results.clone()),
                error: None,
            }),
        ),
        Some(ScanState::Error(err)) => (
            StatusCode::OK,
            Json(ScanStatusResponse {
                id: scan_id,
                status: "error".to_string(),
                results: None,
                error: Some(err.clone()),
            }),
        ),
        None => (
            StatusCode::NOT_FOUND,
            Json(ScanStatusResponse {
                id: scan_id,
                status: "not_found".to_string(),
                results: None,
                error: None,
            }),
        ),
    }
}
