use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use axum::{
    extract::Query,
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use czkawka_core::common::config_cache_path::set_config_cache_path;
use czkawka_core::common::model::{CheckingMethod, HashType};
use czkawka_core::common::tool_data::{CommonData, DeleteMethod};
use czkawka_core::common::traits::Search;
use czkawka_core::tools::duplicate::{DuplicateFinder, DuplicateFinderParameters};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use tokio::task::spawn_blocking;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    scans: Arc<Mutex<HashMap<String, ScanState>>>,
}

enum ScanState {
    Running,
    Completed(ScanResults),
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScanRequest {
    directories: Vec<String>,
    #[serde(default)]
    exclude_directories: Vec<String>,
    #[serde(default = "default_min_file_size")]
    min_file_size: u64,
}

fn default_min_file_size() -> u64 {
    8192
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScanResponse {
    id: String,
    status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScanStatusResponse {
    id: String,
    status: String,
    results: Option<ScanResults>,
    error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScanResults {
    total_duplicate_groups: usize,
    total_duplicate_files: usize,
    wasted_space_bytes: u64,
    scanning_time_ms: u64,
    groups: Vec<DuplicateGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DuplicateGroup {
    size: u64,
    hash: String,
    files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DirectoryEntry {
    name: String,
    path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DirectoryListingResponse {
    path: String,
    directories: Vec<DirectoryEntry>,
}

async fn start_scan(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ScanRequest>,
) -> (StatusCode, Json<ScanResponse>) {
    let scan_id = Uuid::new_v4().to_string();

    {
        let mut scans = state.scans.lock().unwrap();
        scans.insert(scan_id.clone(), ScanState::Running);
    }

    let state_clone = Arc::clone(&state);
    let scan_id_clone = scan_id.clone();

    spawn_blocking(move || {
        log::info!("Starting scan {}", scan_id_clone);
        let result = run_scan(request);
        let mut scans = state_clone.scans.lock().unwrap();
        match result {
            Ok(results) => {
                log::info!("Scan {} completed with {} groups", scan_id_clone, results.total_duplicate_groups);
                scans.insert(scan_id_clone, ScanState::Completed(results));
            }
            Err(e) => {
                log::info!("Scan {} error: {}", scan_id_clone, e);
                scans.insert(scan_id_clone, ScanState::Error(e));
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

fn run_scan(request: ScanRequest) -> Result<ScanResults, String> {
    set_config_cache_path("Czkawka", "Czkawka");

    let params = DuplicateFinderParameters::new(
        CheckingMethod::Hash,
        HashType::Blake3,
        true,
        8192,
        257144,
        false,
    );

    let mut finder = DuplicateFinder::new(params);

    let included: Vec<PathBuf> = request.directories.into_iter().map(PathBuf::from).collect();
    let excluded: Vec<PathBuf> = request
        .exclude_directories
        .into_iter()
        .map(PathBuf::from)
        .collect();

    finder.set_included_paths(included);
    finder.set_excluded_paths(excluded);
    finder.set_minimal_file_size(request.min_file_size);
    finder.set_recursive_search(true);
    finder.set_delete_method(DeleteMethod::None);
    finder.set_dry_run(true);
    finder.set_hide_hard_links(true);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    let groups = finder.get_files_sorted_by_hash();

    let mut duplicate_groups = Vec::new();
    let mut total_files = 0;

    for group_vec in groups.values() {
        for group in group_vec {
            if group.len() > 1 {
                let mut files = Vec::new();
                let mut hash = String::new();
                for entry in group {
                    files.push(entry.path.to_string_lossy().to_string());
                    if hash.is_empty() {
                        hash = entry.hash.clone();
                    }
                }
                total_files += files.len();
                duplicate_groups.push(DuplicateGroup {
                    size: group[0].size,
                    hash,
                    files,
                });
            }
        }
    }

    Ok(ScanResults {
        total_duplicate_groups: duplicate_groups.len(),
        total_duplicate_files: total_files,
        wasted_space_bytes: info.lost_space_by_hash,
        scanning_time_ms: info.scanning_time.as_millis() as u64,
        groups: duplicate_groups,
    })
}

async fn get_scan_status(
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

async fn health() -> &'static str {
    "ok"
}

#[derive(Debug, Deserialize)]
struct DirectoryQuery {
    path: String,
}

async fn list_directories(
    Query(query): Query<DirectoryQuery>,
) -> (StatusCode, Json<DirectoryListingResponse>) {
    let path = &query.path;

    match std::fs::read_dir(path) {
        Ok(entries) => {
            let mut directories = Vec::new();
            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                    if let Ok(name) = entry.file_name().into_string() {
                        if let Some(full_path) = entry.path().to_str().map(|p| p.to_string()) {
                            directories.push(DirectoryEntry {
                                name,
                                path: full_path,
                            });
                        }
                    }
                }
            }
            directories.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            (StatusCode::OK, Json(DirectoryListingResponse {
                path: path.to_string(),
                directories,
            }))
        }
        Err(e) => {
            log::warn!("Failed to read directory {}: {}", path, e);
            (StatusCode::NOT_FOUND, Json(DirectoryListingResponse {
                path: path.to_string(),
                directories: Vec::new(),
            }))
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    log::info!("Backend starting on 0.0.0.0:3000");

    let state = Arc::new(AppState {
        scans: Arc::new(Mutex::new(HashMap::new())),
    });

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/scan", post(start_scan))
        .route("/api/scan/{id}", get(get_scan_status))
        .route("/api/directories", get(list_directories))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
