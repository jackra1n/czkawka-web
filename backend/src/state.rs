use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::models::ScanResults;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppPersistentState {
    #[serde(default)]
    pub directories: Directories,
    #[serde(default)]
    pub tools: HashMap<String, ToolState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_browser_directory: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Directories {
    #[serde(default)]
    pub included: Vec<String>,
    #[serde(default)]
    pub excluded: Vec<String>,
    #[serde(default)]
    pub excluded_items: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolState {
    #[serde(default = "default_idle")]
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scan_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<ScanResults>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub checked_files: Vec<String>,
}

fn default_idle() -> String {
    "idle".to_string()
}

pub fn load_state(path: &Path) -> AppPersistentState {
    if !path.exists() {
        return AppPersistentState::default();
    }
    match fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|e| {
            log::warn!("Failed to parse state file: {e}, using defaults");
            AppPersistentState::default()
        }),
        Err(e) => {
            log::warn!("Failed to read state file: {e}, using defaults");
            AppPersistentState::default()
        }
    }
}

pub fn save_state(path: &Path, state: &AppPersistentState) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(state)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let temp_path = path.with_extension("tmp");
    let mut file = fs::File::create(&temp_path)?;
    file.write_all(content.as_bytes())?;
    file.sync_all()?;
    drop(file);
    fs::rename(temp_path, path)?;
    Ok(())
}
