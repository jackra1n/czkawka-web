use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use czkawka_core::common::model::{CheckingMethod, HashType};
use czkawka_core::common::tool_data::{CommonData, DeleteMethod};
use czkawka_core::common::traits::Search;
use czkawka_core::tools::duplicate::{DuplicateFinder, DuplicateFinderParameters};

use crate::models::{DuplicateFile, DuplicateGroup, ScanRequest, ScanResults};

pub fn run_scan(request: ScanRequest) -> Result<ScanResults, String> {
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
