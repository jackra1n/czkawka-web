use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    pub scans: Arc<Mutex<HashMap<String, ScanState>>>,
    pub persistent: Arc<Mutex<crate::state::AppPersistentState>>,
    pub state_path: PathBuf,
}

pub type SharedProgress = Arc<Mutex<Option<ScanProgress>>>;

pub enum ScanState {
    Running {
        progress: SharedProgress,
        stop_flag: Arc<std::sync::atomic::AtomicBool>,
    },
    Completed(ScanResults),
    Cancelled,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub stage_label: String,
    pub current_stage_idx: u8,
    pub max_stage_idx: u8,
    pub entries_checked: usize,
    pub entries_to_check: usize,
    pub bytes_checked: u64,
    pub bytes_to_check: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanRequest {
    pub directories: Vec<String>,
    #[serde(default)]
    pub exclude_directories: Vec<String>,
    #[serde(default)]
    pub excluded_items: String,
    #[serde(default = "default_true")]
    pub hide_hard_links: bool,
    #[serde(default = "default_min_file_size")]
    pub min_file_size: u64,
    #[serde(default = "default_tool_id")]
    pub tool_id: String,
    // Big files configuration
    #[serde(default)]
    pub number_of_files: Option<u32>,
    #[serde(default)]
    pub search_mode: Option<String>,
    // Similar videos configuration
    #[serde(default)]
    pub tolerance: Option<i32>,
    #[serde(default)]
    pub vid_hash_duration: Option<u32>,
    #[serde(default)]
    pub crop_detect: Option<String>,
    // Similar images configuration
    #[serde(default)]
    pub hash_alg: Option<String>,
    #[serde(default)]
    pub hash_size: Option<u8>,
    #[serde(default)]
    pub resize_filter: Option<String>,
    #[serde(default)]
    pub similarity: Option<u32>,
    // Same music configuration
    #[serde(default)]
    pub music_check_type: Option<String>,
    // Broken files configuration
    #[serde(default)]
    pub broken_file_types: Option<String>,
    // Bad extensions configuration
    #[serde(default)]
    pub include_files_without_extension: Option<bool>,
    // Bad names configuration
    #[serde(default)]
    pub bad_name_uppercase_extension: Option<bool>,
    #[serde(default)]
    pub bad_name_emoji: Option<bool>,
    #[serde(default)]
    pub bad_name_spaces: Option<bool>,
    #[serde(default)]
    pub bad_name_non_ascii: Option<bool>,
    #[serde(default)]
    pub bad_name_restricted_charset: Option<bool>,
    #[serde(default)]
    pub bad_name_allowed_chars: Option<String>,
    #[serde(default)]
    pub bad_name_dedupe_non_alnum: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct FixRequest {
    pub tool_id: String,
    pub files: Vec<String>,
    // Bad names fix params
    #[serde(default)]
    pub bad_name_uppercase_extension: Option<bool>,
    #[serde(default)]
    pub bad_name_emoji: Option<bool>,
    #[serde(default)]
    pub bad_name_spaces: Option<bool>,
    #[serde(default)]
    pub bad_name_non_ascii: Option<bool>,
    #[serde(default)]
    pub bad_name_restricted_charset: Option<bool>,
    #[serde(default)]
    pub bad_name_allowed_chars: Option<String>,
    #[serde(default)]
    pub bad_name_dedupe_non_alnum: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct FailedFix {
    pub path: String,
    pub error: String,
}

#[derive(Debug, Serialize)]
pub struct FixResponse {
    pub fixed: Vec<String>,
    pub failed: Vec<FailedFix>,
}

fn default_true() -> bool {
    true
}

fn default_min_file_size() -> u64 {
    8192
}

fn default_tool_id() -> String {
    "duplicates".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResponse {
    pub id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanStatusResponse {
    pub id: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<ScanProgress>,
    pub results: Option<ScanResults>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResults {
    pub total_groups: usize,
    pub total_items: usize,
    pub wasted_bytes: u64,
    pub scanning_time_ms: u64,
    pub groups: Vec<FileGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannedFile {
    pub path: String,
    pub modified_date: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileGroup {
    pub size: u64,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hash: String,
    pub files: Vec<ScannedFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultsResponse {
    pub excluded_directories: Vec<String>,
    pub excluded_items: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_directory: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryEntry {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryListingResponse {
    pub path: String,
    pub directories: Vec<DirectoryEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserDirectoryResponse {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteRequest {
    pub tool_id: String,
    pub files: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct FailedDeletion {
    pub path: String,
    pub error: String,
}

#[derive(Debug, Serialize)]
pub struct DeleteResponse {
    pub deleted: Vec<String>,
    pub failed: Vec<FailedDeletion>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LinkType {
    Hard,
    Soft,
}

#[derive(Debug, Deserialize)]
pub struct LinkRequest {
    pub tool_id: String,
    pub link_type: LinkType,
    pub files: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct FailedLink {
    pub path: String,
    pub error: String,
}

#[derive(Debug, Serialize)]
pub struct LinkResponse {
    pub linked: Vec<String>,
    pub failed: Vec<FailedLink>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDirectoriesRequest {
    #[serde(default)]
    pub included: Vec<String>,
    #[serde(default)]
    pub excluded: Vec<String>,
    #[serde(default)]
    pub excluded_items: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateToolStateRequest {
    #[serde(default)]
    pub checked_files: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLastBrowserDirectoryRequest {
    pub path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DirectoryQuery {
    pub path: String,
    #[serde(default)]
    pub hidden: bool,
}

#[derive(Debug, Deserialize)]
pub struct FileQuery {
    pub path: String,
}
