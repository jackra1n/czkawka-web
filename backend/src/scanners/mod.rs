use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use czkawka_core::common::tool_data::{CommonData, DeleteMethod};

use crate::models::ScanRequest;

pub mod big_files;
pub mod duplicates;
pub mod empty_folders;
pub mod similar_images;

pub fn configure_common_data<T: CommonData>(tool: &mut T, request: &ScanRequest) {
    let included: Vec<PathBuf> = request.directories.iter().map(PathBuf::from).collect();
    let excluded: Vec<PathBuf> = request.exclude_directories.iter().map(PathBuf::from).collect();

    tool.set_included_paths(included);
    tool.set_excluded_paths(excluded);

    let excluded_items: Vec<String> = request
        .excluded_items
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    if !excluded_items.is_empty() {
        tool.set_excluded_items(excluded_items);
    }

    tool.set_minimal_file_size(request.min_file_size);
    tool.set_recursive_search(true);
    tool.set_delete_method(DeleteMethod::None);
    tool.set_dry_run(true);
    tool.set_hide_hard_links(true);
}

pub fn make_stop_flag() -> Arc<AtomicBool> {
    Arc::new(AtomicBool::new(false))
}
