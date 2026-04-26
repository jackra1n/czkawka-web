use axum::{Json, extract::Query, http::StatusCode};

use crate::models::{DirectoryEntry, DirectoryListingResponse, DirectoryQuery};

pub async fn list_directories(
    Query(query): Query<DirectoryQuery>,
) -> (StatusCode, Json<DirectoryListingResponse>) {
    let mut path = query.path;
    let show_hidden = query.hidden;

    if (path == "~" || path == "~/")
        && let Some(home) = dirs::home_dir()
    {
        path = home.to_string_lossy().to_string();
    }

    match std::fs::read_dir(&path) {
        Ok(entries) => {
            let mut directories = Vec::new();
            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false)
                    && let Ok(name) = entry.file_name().into_string()
                {
                    if !show_hidden && name.starts_with('.') {
                        continue;
                    }
                    if let Some(full_path) = entry.path().to_str().map(|p| p.to_string()) {
                        directories.push(DirectoryEntry {
                            name,
                            path: full_path,
                        });
                    }
                }
            }
            directories.sort_by_key(|a| a.name.to_lowercase());
            (
                StatusCode::OK,
                Json(DirectoryListingResponse {
                    path: path.to_string(),
                    directories,
                }),
            )
        }
        Err(e) => {
            log::warn!("Failed to read directory {path}: {e}");
            (
                StatusCode::NOT_FOUND,
                Json(DirectoryListingResponse {
                    path: path.to_string(),
                    directories: Vec::new(),
                }),
            )
        }
    }
}
