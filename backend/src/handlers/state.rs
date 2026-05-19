use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::models::{AppState, BrowserDirectoryResponse, DefaultsResponse, UpdateDirectoriesRequest, UpdateLastBrowserDirectoryRequest, UpdateToolStateRequest};
use crate::state;
use czkawka_core::common::items::{DEFAULT_EXCLUDED_DIRECTORIES, DEFAULT_EXCLUDED_ITEMS};

pub async fn get_state(State(state): State<Arc<AppState>>) -> Json<state::AppPersistentState> {
    let persistent = state.persistent.lock().unwrap();
    Json(persistent.clone())
}

pub async fn get_defaults() -> Json<DefaultsResponse> {
    let default_directory = std::env::var("DEFAULT_SCAN_PATH").ok();
    Json(DefaultsResponse {
        excluded_directories: DEFAULT_EXCLUDED_DIRECTORIES
            .iter()
            .map(|s| s.to_string())
            .collect(),
        excluded_items: DEFAULT_EXCLUDED_ITEMS.to_string(),
        default_directory,
    })
}

pub async fn get_browser_directory(State(state): State<Arc<AppState>>) -> Json<BrowserDirectoryResponse> {
    let mut persistent = state.persistent.lock().unwrap();

    if let Some(ref stored) = persistent.last_browser_directory {
        if std::path::Path::new(stored).is_dir() {
            return Json(BrowserDirectoryResponse { path: stored.clone() });
        }
    }

    let resolved = std::env::var("DEFAULT_SCAN_PATH")
        .ok()
        .filter(|p| std::path::Path::new(p).is_dir())
        .or_else(|| dirs::home_dir().map(|p| p.to_string_lossy().to_string()))
        .unwrap_or_else(|| "/".to_string());

    persistent.last_browser_directory = Some(resolved.clone());

    if let Err(e) = state::save_state(&state.state_path, &persistent) {
        log::error!("Failed to save browser directory: {e}");
    }

    Json(BrowserDirectoryResponse { path: resolved })
}

pub async fn update_directories(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateDirectoriesRequest>,
) -> StatusCode {
    let mut persistent = state.persistent.lock().unwrap();
    persistent.directories.included = request.included;
    persistent.directories.excluded = request.excluded;
    persistent.directories.excluded_items = request.excluded_items;
    if let Err(e) = state::save_state(&state.state_path, &persistent) {
        log::error!("Failed to save state: {e}");
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    StatusCode::OK
}

pub async fn update_last_browser_directory(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateLastBrowserDirectoryRequest>,
) -> StatusCode {
    let mut persistent = state.persistent.lock().unwrap();
    persistent.last_browser_directory = request.path;
    if let Err(e) = state::save_state(&state.state_path, &persistent) {
        log::error!("Failed to save last browser directory: {e}");
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    StatusCode::OK
}

pub async fn update_tool_state(
    State(state): State<Arc<AppState>>,
    Path(tool_id): Path<String>,
    Json(request): Json<UpdateToolStateRequest>,
) -> StatusCode {
    let mut persistent = state.persistent.lock().unwrap();
    let tool = persistent.tools.entry(tool_id).or_default();
    tool.checked_files = request.checked_files;
    if let Err(e) = state::save_state(&state.state_path, &persistent) {
        log::error!("Failed to save state: {e}");
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    StatusCode::OK
}
