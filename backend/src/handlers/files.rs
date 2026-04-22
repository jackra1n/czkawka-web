use std::sync::Arc;

use axum::{
    Json,
    body::Body,
    extract::{Query, State},
    http::StatusCode,
    response::Response,
};
use std::path::PathBuf;

use crate::models::{
    AppState, DeleteRequest, DeleteResponse, FailedDeletion, FileQuery,
};
use crate::state;

pub async fn delete_files(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteRequest>,
) -> (StatusCode, Json<DeleteResponse>) {
    let mut deleted = Vec::new();
    let mut failed = Vec::new();

    for path in &request.files {
        match tokio::fs::remove_file(path).await {
            Ok(()) => {
                deleted.push(path.clone());
            }
            Err(e) => {
                log::warn!("Failed to delete file {path}: {e}");
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
                results.groups.retain(|g| g.files.len() >= 2);
                results.total_duplicate_groups = results.groups.len();
                results.total_duplicate_files = results.groups.iter().map(|g| g.files.len()).sum();
                results.wasted_space_bytes = results
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

pub async fn serve_file(Query(query): Query<FileQuery>) -> Result<Response, (StatusCode, String)> {
    let path = PathBuf::from(&query.path);

    if !path.is_file() {
        return Err((StatusCode::NOT_FOUND, "File not found".to_string()));
    }

    let content = match tokio::fs::read(&path).await {
        Ok(data) => data,
        Err(e) => {
            log::warn!("Failed to read file {}: {}", query.path, e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read file".to_string(),
            ));
        }
    };

    let content_type = mime_guess::from_path(&path)
        .first_or_octet_stream()
        .to_string();

    Ok(Response::builder()
        .header("Content-Type", content_type)
        .body(Body::from(content))
        .unwrap())
}
