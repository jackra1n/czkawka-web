use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::models::{AppState, DefaultsResponse, UpdateDirectoriesRequest, UpdateToolStateRequest};
use crate::state;
use czkawka_core::common::items::{DEFAULT_EXCLUDED_DIRECTORIES, DEFAULT_EXCLUDED_ITEMS};

pub async fn get_state(State(state): State<Arc<AppState>>) -> Json<state::AppPersistentState> {
    let persistent = state.persistent.lock().unwrap();
    Json(persistent.clone())
}

pub async fn get_defaults() -> Json<DefaultsResponse> {
    Json(DefaultsResponse {
        excluded_directories: DEFAULT_EXCLUDED_DIRECTORIES
            .iter()
            .map(|s| s.to_string())
            .collect(),
        excluded_items: DEFAULT_EXCLUDED_ITEMS.to_string(),
    })
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
