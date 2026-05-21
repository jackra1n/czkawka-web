use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

use axum::{
    Router,
    routing::{get, post},
};
use czkawka_core::common::config_cache_path::set_config_cache_path;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};

mod handlers;
mod models;
mod scan;
mod scanners;
mod state;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
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

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(6198);

    log::info!("Backend starting on 0.0.0.0:{port}");

    let app_state = Arc::new(models::AppState {
        scans: Arc::new(Mutex::new(HashMap::new())),
        persistent: Arc::new(Mutex::new(persistent_state)),
        state_path,
    });

    let frontend_build_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../frontend/build");

    let app = Router::new()
        .route("/api/health", get(handlers::health::health))
        .route("/api/scan", post(handlers::scan::start_scan))
        .route("/api/scan/{id}", get(handlers::scan::get_scan_status))
        .route("/api/scan/{id}/cancel", post(handlers::scan::cancel_scan))
        .route("/api/state", get(handlers::state::get_state))
        .route(
            "/api/state/directories",
            post(handlers::state::update_directories),
        )
        .route(
            "/api/state/tools/{tool_id}",
            post(handlers::state::update_tool_state),
        )
        .route(
            "/api/state/last-browser-directory",
            post(handlers::state::update_last_browser_directory),
        )
        .route("/api/delete", post(handlers::files::delete_files))
        .route("/api/fix", post(handlers::fix::fix_files))
        .route("/api/defaults", get(handlers::state::get_defaults))
        .route(
            "/api/browser-directory",
            get(handlers::state::get_browser_directory),
        )
        .route(
            "/api/directories",
            get(handlers::directories::list_directories),
        )
        .route("/api/file", get(handlers::files::serve_file))
        .fallback_service(
            ServeDir::new(&frontend_build_dir)
                .fallback(ServeFile::new(frontend_build_dir.join("index.html"))),
        )
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
