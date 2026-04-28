use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use czkawka_core::common::traits::Search;
use czkawka_core::tools::empty_folder::EmptyFolder;

use crate::models::{FileGroup, ScanRequest, ScanResults, ScannedFile};
use crate::scanners::{configure_common_data, make_stop_flag};

pub fn run(request: ScanRequest, progress_sender: &Sender<ProgressData>) -> Result<ScanResults, String> {
    let mut finder = EmptyFolder::new();
    configure_common_data(&mut finder, &request);

    let stop_flag = make_stop_flag();
    finder.search(&stop_flag, Some(progress_sender));

    let info = finder.get_information();
    let empty_list = finder.get_empty_folder_list();

    let mut groups = Vec::new();

    for entry in empty_list.values() {
        let path = entry.path.to_string_lossy().to_string();
        let modified_date = std::fs::metadata(&entry.path)
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_millis() as u64);
        groups.push(FileGroup {
            size: 0,
            hash: String::new(),
            files: vec![ScannedFile {
                path,
                modified_date,
                dimensions: None,
                similarity: None,
            }],
        });
    }

    let total_items = groups.len();

    Ok(ScanResults {
        total_groups: groups.len(),
        total_items,
        wasted_bytes: 0,
        scanning_time_ms: info.scanning_time.as_millis() as u64,
        groups,
    })
}
