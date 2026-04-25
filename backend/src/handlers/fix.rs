use std::path::Path;
use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::StatusCode,
};

use czkawka_core::tools::bad_names::core::check_and_generate_new_name;
use czkawka_core::tools::bad_names::NameIssues;
use czkawka_core::tools::exif_remover::core::{clean_exif_tags, extract_exif_tags_public};

use crate::models::{AppState, FixRequest, FixResponse, FailedFix};

pub async fn fix_files(
    State(_state): State<Arc<AppState>>,
    Json(request): Json<FixRequest>,
) -> (StatusCode, Json<FixResponse>) {
    let mut fixed = Vec::new();
    let mut failed = Vec::new();

    match request.tool_id.as_str() {
        "exif-remover" => {
            for path in &request.files {
                match fix_exif(path) {
                    Ok(()) => fixed.push(path.clone()),
                    Err(e) => failed.push(FailedFix { path: path.clone(), error: e }),
                }
            }
        }
        "bad-names" => {
            let checked_issues = NameIssues {
                uppercase_extension: request.bad_name_uppercase_extension.unwrap_or(true),
                emoji_used: request.bad_name_emoji.unwrap_or(true),
                space_at_start_or_end: request.bad_name_spaces.unwrap_or(true),
                non_ascii_graphical: request.bad_name_non_ascii.unwrap_or(true),
                restricted_charset_allowed: if request.bad_name_restricted_charset.unwrap_or(false) {
                    let chars: Vec<char> = request.bad_name_allowed_chars
                        .as_deref()
                        .unwrap_or("_- .")
                        .chars()
                        .collect();
                    Some(chars)
                } else {
                    None
                },
                remove_duplicated_non_alphanumeric: request.bad_name_dedupe_non_alnum.unwrap_or(false),
            };
            for path in &request.files {
                match fix_bad_name(path, &checked_issues) {
                    Ok(()) => fixed.push(path.clone()),
                    Err(e) => failed.push(FailedFix { path: path.clone(), error: e }),
                }
            }
        }
        "bad-extensions" => {
            for path in &request.files {
                match fix_bad_extension(path) {
                    Ok(()) => fixed.push(path.clone()),
                    Err(e) => failed.push(FailedFix { path: path.clone(), error: e }),
                }
            }
        }
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(FixResponse {
                    fixed,
                    failed: vec![FailedFix {
                        path: String::new(),
                        error: format!("Unsupported fix tool_id: {}", request.tool_id),
                    }],
                }),
            );
        }
    }

    (StatusCode::OK, Json(FixResponse { fixed, failed }))
}

fn fix_exif(path: &str) -> Result<(), String> {
    let tags = extract_exif_tags_public(Path::new(path))
        .map_err(|e| format!("Failed to extract EXIF: {e}"))?;
    if tags.is_empty() {
        return Ok(());
    }
    let tags_to_remove: Vec<(u16, String)> = tags.into_iter().map(|(code, group)| (code, group)).collect();
    clean_exif_tags(path, &tags_to_remove, true)
        .map_err(|e| format!("Failed to clean EXIF: {e}"))?;
    Ok(())
}

fn fix_bad_name(path: &str, checked_issues: &NameIssues) -> Result<(), String> {
    let new_name = check_and_generate_new_name(Path::new(path), checked_issues)
        .ok_or_else(|| "No name change needed".to_string())?;
    let new_path = Path::new(path).with_file_name(&new_name);
    if new_path.exists() {
        return Err(format!("Target file already exists: {}", new_path.display()));
    }
    std::fs::rename(path, &new_path).map_err(|e| format!("Rename failed: {e}"))?;
    Ok(())
}

fn fix_bad_extension(path: &str) -> Result<(), String> {
    let kind = infer::get_from_path(path)
        .map_err(|e| format!("Failed to read file: {e}"))?
        .ok_or_else(|| "Could not infer file type".to_string())?;
    let proper_ext = kind.extension();
    let current_ext = Path::new(path).extension().and_then(|e| e.to_str()).unwrap_or("");
    if current_ext.eq_ignore_ascii_case(proper_ext) {
        return Ok(());
    }
    let new_path = Path::new(path).with_extension(proper_ext);
    if new_path.exists() {
        return Err(format!("Target file already exists: {}", new_path.display()));
    }
    std::fs::rename(path, &new_path).map_err(|e| format!("Rename failed: {e}"))?;
    Ok(())
}
