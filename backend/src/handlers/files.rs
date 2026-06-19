use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, Request, State},
    http::StatusCode,
    response::IntoResponse,
};
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
    let mut missing = Vec::new();
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
            // The file is already gone, which is the outcome we wanted anyway.
            // Treat it as success so the stale entry gets cleared from results.
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                log::info!("Already gone, clearing from results: {path}");
                missing.push(path.clone());
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

    // Both genuinely deleted and already-missing files should be pruned from state.
    let removed: Vec<String> = deleted.iter().chain(missing.iter()).cloned().collect();

    if !removed.is_empty() {
        let mut persistent = state.persistent.lock().unwrap();
        if let Some(tool) = persistent.tools.get_mut(&request.tool_id) {
            tool.checked_files.retain(|p| !removed.contains(p));

            if let Some(ref mut results) = tool.results {
                for group in results.groups.iter_mut() {
                    group.files.retain(|f| !removed.contains(&f.path));
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

    (
        StatusCode::OK,
        Json(DeleteResponse {
            deleted,
            missing,
            failed,
        }),
    )
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

    // source of truth lookup map for requested files
    let checked_set: HashSet<&String> = request.files.iter().collect();

    let mut grouped_checked_files = HashSet::new();
    let mut link_plans = Vec::new();

    // 1. lock the state briefly, plan links, and immediately capture immediate failures
    {
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

        for group in &results.groups {
            let mut checked_in_group = Vec::new();

            for file in &group.files {
                if let Some(&req_file_ref) = checked_set.get(&file.path) {
                    checked_in_group.push(req_file_ref);
                    grouped_checked_files.insert(req_file_ref);
                }
            }

            // need at least 2 checked files in the group to link
            if checked_in_group.len() < 2 {
                continue;
            }

            // first checked file is the original, rest get linked to it
            let original = checked_in_group[0];

            for &file_path in &checked_in_group[1..] {
                link_plans.push((original.clone(), file_path.clone()));
            }
        }
    };

    // 2. identify requested files that weren't even in the scan results
    for file in &request.files {
        if !grouped_checked_files.contains(file) {
            failed.push(FailedLink {
                path: file.clone(),
                error: "File not found in scan results".to_string(),
            });
        }
    }

    // 3. perform the blocking linking operations
    if !link_plans.is_empty() {
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
                    Ok(()) => linked_paths.push(derived),
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
    }

    // 4. update the backend state
    let mut state_to_save = None;

    if !linked.is_empty() || !failed.is_empty() {
        let mut persistent = state.persistent.lock().unwrap();
        if let Some(tool) = persistent.tools.get_mut(&request.tool_id) {
            let old_checked_len = tool.checked_files.len();
            tool.checked_files.retain(|p| !checked_set.contains(p));
            let mut changed = tool.checked_files.len() != old_checked_len;

            if !linked.is_empty()
                && let Some(ref mut results) = tool.results
            {
                let linked_set: HashSet<&String> = linked.iter().collect();

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

                changed = true;
            }

            if changed {
                state_to_save = Some(persistent.clone());
            }
        }
    }

    // 5. perform disk writing completely detached from mutex window
    if let Some(persistent_state) = state_to_save {
        let path = state.state_path.clone();
        tokio::task::spawn_blocking(move || {
            if let Err(e) = crate::state::save_state(&path, &persistent_state) {
                log::error!("Failed to save state to disk after linking files: {}", e);
            }
        });
    }

    (StatusCode::OK, Json(LinkResponse { linked, failed }))
}
