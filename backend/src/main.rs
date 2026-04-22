use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use axum::{
    Json, Router,
    body::Body,
    extract::Query,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
    routing::{get, post},
};
use czkawka_core::common::config_cache_path::set_config_cache_path;
use czkawka_core::common::items::{DEFAULT_EXCLUDED_DIRECTORIES, DEFAULT_EXCLUDED_ITEMS};
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

mod state;

#[derive(Clone)]
struct AppState {
    scans: Arc<Mutex<HashMap<String, ScanState>>>,
    persistent: Arc<Mutex<state::AppPersistentState>>,
    state_path: PathBuf,
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
    #[serde(default)]
    excluded_items: String,
    #[serde(default = "default_min_file_size")]
    min_file_size: u64,
    #[serde(default = "default_tool_id")]
    tool_id: String,
}

fn default_min_file_size() -> u64 {
    8192
}

fn default_tool_id() -> String {
    "duplicates".to_string()
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
struct DuplicateFile {
    path: String,
    modified_date: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DuplicateGroup {
    size: u64,
    hash: String,
    files: Vec<DuplicateFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DefaultsResponse {
    excluded_directories: Vec<String>,
    excluded_items: String,
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
                    results.total_duplicate_groups
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

fn run_scan(request: ScanRequest) -> Result<ScanResults, String> {
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

    let excluded_items: Vec<String> = request
        .excluded_items
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    if !excluded_items.is_empty() {
        finder.set_excluded_items(excluded_items);
    }

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
                    let path = entry.path.to_string_lossy().to_string();
                    let modified_date = std::fs::metadata(&entry.path)
                        .and_then(|m| m.modified())
                        .ok()
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                        .map(|d| d.as_millis() as u64);
                    files.push(DuplicateFile {
                        path,
                        modified_date,
                    });
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

async fn get_state(State(state): State<Arc<AppState>>) -> Json<state::AppPersistentState> {
    let persistent = state.persistent.lock().unwrap();
    Json(persistent.clone())
}

async fn get_defaults() -> Json<DefaultsResponse> {
    Json(DefaultsResponse {
        excluded_directories: DEFAULT_EXCLUDED_DIRECTORIES.iter().map(|s| s.to_string()).collect(),
        excluded_items: DEFAULT_EXCLUDED_ITEMS.to_string(),
    })
}

#[derive(Debug, Deserialize)]
struct UpdateDirectoriesRequest {
    #[serde(default)]
    included: Vec<String>,
    #[serde(default)]
    excluded: Vec<String>,
    #[serde(default)]
    excluded_items: String,
}

async fn update_directories(
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

#[derive(Debug, Deserialize)]
struct UpdateToolStateRequest {
    #[serde(default)]
    checked_files: Vec<String>,
}

async fn update_tool_state(
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

#[derive(Debug, Deserialize)]
struct DirectoryQuery {
    path: String,
    #[serde(default)]
    hidden: bool,
}

async fn list_directories(
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

#[derive(Debug, Deserialize)]
struct FileQuery {
    path: String,
}

async fn serve_file(Query(query): Query<FileQuery>) -> Result<Response, (StatusCode, String)> {
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

#[tokio::main]
async fn main() {
    env_logger::init();
    set_config_cache_path("Czkawka", "Czkawka");

    let config_dir = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    let app_config_dir = config_dir.join("czkawka-web");
    let state_path = app_config_dir.join("state.json");

    let mut persistent_state = state::load_state(&state_path);
    for tool in persistent_state.tools.values_mut() {
        if tool.status == "running" {
            tool.status = "idle".to_string();
            tool.scan_id = None;
        }
    }
    if let Err(e) = state::save_state(&state_path, &persistent_state) {
        log::warn!("Failed to save cleaned state on startup: {e}");
    }

    log::info!("Backend starting on 0.0.0.0:3000");

    let state = Arc::new(AppState {
        scans: Arc::new(Mutex::new(HashMap::new())),
        persistent: Arc::new(Mutex::new(persistent_state)),
        state_path,
    });

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/scan", post(start_scan))
        .route("/api/scan/{id}", get(get_scan_status))
        .route("/api/state", get(get_state))
        .route("/api/state/directories", post(update_directories))
        .route("/api/state/tools/{tool_id}", post(update_tool_state))
        .route("/api/defaults", get(get_defaults))
        .route("/api/directories", get(list_directories))
        .route("/api/file", get(serve_file))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
