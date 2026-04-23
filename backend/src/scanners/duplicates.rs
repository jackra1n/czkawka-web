use czkawka_core::common::model::{CheckingMethod, HashType};
use czkawka_core::common::traits::Search;
use czkawka_core::tools::duplicate::{DuplicateFinder, DuplicateFinderParameters};

use crate::models::{FileGroup, ScanRequest, ScanResults, ScannedFile};
use crate::scanners::{configure_common_data, make_stop_flag};

pub fn run(request: ScanRequest) -> Result<ScanResults, String> {
    let params = DuplicateFinderParameters::new(
        CheckingMethod::Hash,
        HashType::Blake3,
        true,
        8192,
        257144,
        false,
    );

    let mut finder = DuplicateFinder::new(params);
    configure_common_data(&mut finder, &request);

    let stop_flag = make_stop_flag();
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
                    files.push(ScannedFile {
                        path,
                        modified_date,
                        dimensions: None,
                        similarity: None,
                    });
                    if hash.is_empty() {
                        hash = entry.hash.clone();
                    }
                }
                total_files += files.len();
                duplicate_groups.push(FileGroup {
                    size: group[0].size,
                    hash,
                    files,
                });
            }
        }
    }

    Ok(ScanResults {
        total_groups: duplicate_groups.len(),
        total_items: total_files,
        wasted_bytes: info.lost_space_by_hash,
        scanning_time_ms: info.scanning_time.as_millis() as u64,
        groups: duplicate_groups,
    })
}
