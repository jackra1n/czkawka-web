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

use crate::models::{
    AppState, DeleteRequest, DeleteResponse, FailedDeletion, FailedLink, FileQuery, LinkRequest,
    LinkResponse, LinkType,
};
use crate::state;
use czkawka_core::common::{make_file_symlink, make_hard_link};

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

pub async fn link_files(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LinkRequest>,
) -> (StatusCode, Json<LinkResponse>) {
    let mut linked = Vec::new();
    let mut failed = Vec::new();

    let checked_set: std::collections::HashSet<&String> = request.files.iter().collect();

    // 1. Lock the state briefly to find duplicate groups and plan the linking
    let link_plans = {
        let persistent = state.persistent.lock().unwrap();
        let tool = match persistent.tools.get(&request.tool_id) {
            Some(t) => t,
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(LinkResponse {
                        linked,
                        failed: vec![FailedLink {
                            path: String::new(),
                            error: "Tool state not found".to_string(),
                        }],
                    }),
                );
            }
        };
        let results = match &tool.results {
            Some(r) => r,
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(LinkResponse {
                        linked,
                        failed: vec![FailedLink {
                            path: String::new(),
                            error: "No scan results found".to_string(),
                        }],
                    }),
                );
            }
        };

        let mut plans = Vec::new();
        for group in &results.groups {
            let checked_in_group: Vec<&crate::models::ScannedFile> = group
                .files
                .iter()
                .filter(|f| checked_set.contains(&f.path))
                .collect();
            if checked_in_group.is_empty() {
                continue;
            }
            let unchecked_in_group: Vec<&crate::models::ScannedFile> = group
                .files
                .iter()
                .filter(|f| !checked_set.contains(&f.path))
                .collect();
            let original_path = if !unchecked_in_group.is_empty() {
                Some(unchecked_in_group[0].path.clone())
            } else if checked_in_group.len() > 1 {
                Some(checked_in_group[0].path.clone())
            } else {
                None
            };
            if let Some(orig) = original_path {
                for file in checked_in_group {
                    if file.path != orig {
                        plans.push((orig.clone(), file.path.clone()));
                    }
                }
            }
        }
        plans
    };

    // 2. Perform the link operations in spawn_blocking
    let link_type = request.link_type;
    let link_results = match tokio::task::spawn_blocking(move || {
        let mut linked_paths = Vec::new();
        let mut failed_links = Vec::new();
        for (orig, derived) in link_plans {
            let res = match link_type {
                LinkType::Hard => make_hard_link(&orig, &derived),
                LinkType::Soft => make_file_symlink(&orig, &derived),
            };
            match res {
                Ok(()) => {
                    linked_paths.push(derived);
                }
                Err(e) => {
                    log::warn!("Failed to link {} to original {}: {}", derived, orig, e);
                    failed_links.push(FailedLink {
                        path: derived,
                        error: e.to_string(),
                    });
                }
            }
        }
        (linked_paths, failed_links)
    })
    .await
    {
        Ok(res) => res,
        Err(e) => {
            log::error!("Spawn blocking task failed: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(LinkResponse {
                    linked: vec![],
                    failed: vec![FailedLink {
                        path: String::new(),
                        error: format!("Internal task error: {e}"),
                    }],
                }),
            );
        }
    };

    let (successful_linked, mut failed_links) = link_results;
    linked.extend(successful_linked);
    failed.append(&mut failed_links);

    let linked_set: std::collections::HashSet<&String> = linked.iter().collect();

    // 3. Mark files that were checked but had no other file in the group to link to as failed
    for file in &request.files {
        // Check if this file was actually part of any group with an original
        // If not in linked or failed, and it wasn't the chosen original in any group, it failed
        // We can check if it's not in linked and not in failed
        if !linked_set.contains(file) && !failed.iter().any(|f| &f.path == file) {
            // It could be the original file of some group. If so, it shouldn't be marked as failed
            // Let's verify if it's the original.
            let mut is_original = false;
            let mut has_group = false;
            {
                let persistent = state.persistent.lock().unwrap();
                if let Some(results) = persistent
                    .tools
                    .get(&request.tool_id)
                    .and_then(|t| t.results.as_ref())
                {
                    for group in &results.groups {
                        if group.files.iter().any(|f| &f.path == file) {
                            has_group = true;
                            let checked_in_group: Vec<&crate::models::ScannedFile> = group
                                .files
                                .iter()
                                .filter(|f| checked_set.contains(&f.path))
                                .collect();
                            let unchecked_in_group: Vec<&crate::models::ScannedFile> = group
                                .files
                                .iter()
                                .filter(|f| !checked_set.contains(&f.path))
                                .collect();
                            let original_path = if !unchecked_in_group.is_empty() {
                                Some(unchecked_in_group[0].path.clone())
                            } else if checked_in_group.len() > 1 {
                                Some(checked_in_group[0].path.clone())
                            } else {
                                None
                            };
                            if original_path.as_ref() == Some(file) {
                                is_original = true;
                            }
                        }
                    }
                }
            }

            if !is_original && has_group {
                failed.push(FailedLink {
                    path: file.clone(),
                    error: "No other file in the group to link to".to_string(),
                });
            }
        }
    }

    // 4. Update the backend state
    {
        let mut persistent = state.persistent.lock().unwrap();
        if let Some(tool) = persistent.tools.get_mut(&request.tool_id) {
            let failed_paths: std::collections::HashSet<&String> =
                failed.iter().map(|f| &f.path).collect();
            let mut changed = false;
            let old_len = tool.checked_files.len();
            tool.checked_files.retain(|p| {
                if checked_set.contains(p) {
                    failed_paths.contains(p)
                } else {
                    true
                }
            });
            if tool.checked_files.len() != old_len {
                changed = true;
            }

            if !linked.is_empty() {
                if let Some(ref mut results) = tool.results {
                    for group in results.groups.iter_mut() {
                        group.files.retain(|f| !linked_set.contains(&f.path));
                    }

                    let min_group_size = 2;
                    results.groups.retain(|g| g.files.len() >= min_group_size);
                    results.total_groups = results.groups.len();
                    results.total_items = results.groups.iter().map(|g| g.files.len()).sum();
                    results.wasted_bytes = results
                        .groups
                        .iter()
                        .map(|g| g.size * (g.files.len() as u64 - 1))
                        .sum();
                }
                changed = true;
            }

            if changed {
                let save_res = crate::state::save_state(&state.state_path, &persistent);
                if let Err(e) = save_res {
                    log::error!("Failed to save state after linking: {e}");
                }
            }
        }
    }

    (StatusCode::OK, Json(LinkResponse { linked, failed }))
}
