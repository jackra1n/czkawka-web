use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, Request, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::path::PathBuf;
use tower::ServiceExt;
use tower_http::services::ServeFile;

use crate::models::{AppState, DeleteRequest, DeleteResponse, FailedDeletion, FileQuery};
use crate::state;

pub async fn delete_files(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteRequest>,
) -> (StatusCode, Json<DeleteResponse>) {
    let mut deleted = Vec::new();
    let mut failed = Vec::new();

    for path in &request.files {
        let path_buf = std::path::PathBuf::from(path);
        let res = if path_buf.is_dir() {
            tokio::fs::remove_dir(path).await
        } else {
            tokio::fs::remove_file(path).await
        };
        match res {
            Ok(()) => {
                deleted.push(path.clone());
            }
            Err(e) => {
                log::warn!("Failed to delete {path}: {e}");
                failed.push(FailedDeletion {
                    path: path.clone(),
                    error: e.to_string(),
                });
            }
        }
    }

    if !deleted.is_empty() {
        let mut persistent = state.persistent.lock().unwrap();
        if let Some(tool) = persistent.tools.get_mut(&request.tool_id) {
            tool.checked_files.retain(|p| !deleted.contains(p));

            if let Some(ref mut results) = tool.results {
                for group in results.groups.iter_mut() {
                    group.files.retain(|f| !deleted.contains(&f.path));
                }
                let min_group_size = if request.tool_id == "empty-folders"
                    || request.tool_id == "big-files"
                    || request.tool_id == "empty-files"
                    || request.tool_id == "temporary"
                {
                    1
                } else {
                    2
                };
                results.groups.retain(|g| g.files.len() >= min_group_size);
                results.total_groups = results.groups.len();
                results.total_items = results.groups.iter().map(|g| g.files.len()).sum();
                results.wasted_bytes = results
                    .groups
                    .iter()
                    .map(|g| g.size * (g.files.len() as u64 - 1))
                    .sum();
            }

            if let Err(e) = state::save_state(&state.state_path, &persistent) {
                log::error!("Failed to save state after deletion: {e}");
            }
        }
    }

    (StatusCode::OK, Json(DeleteResponse { deleted, failed }))
}

pub async fn serve_file(
    Query(query): Query<FileQuery>,
    req: Request,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let path = PathBuf::from(&query.path);

    if !path.is_file() {
        return Err((StatusCode::NOT_FOUND, "File not found".to_string()));
    }

    let service = ServeFile::new(path);
    match service.oneshot(req).await {
        Ok(res) => Ok(res.into_response()),
        Err(err) => {
            log::error!("Failed to serve file: {:?}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to serve file".to_string(),
            ))
        }
    }
}
