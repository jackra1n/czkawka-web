use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use czkawka_core::common::traits::Search;
use czkawka_core::tools::empty_files::EmptyFiles;

use crate::models::{FileGroup, ScanRequest, ScanResults, ScannedFile};
use crate::scanners::configure_common_data;

pub fn run(
    request: ScanRequest,
    progress_sender: &Sender<ProgressData>,
    stop_flag: std::sync::Arc<std::sync::atomic::AtomicBool>,
) -> Result<ScanResults, String> {
    let mut finder = EmptyFiles::new();
    configure_common_data(&mut finder, &request);

    finder.search(&stop_flag, Some(progress_sender));

    let info = finder.get_information();
    let empty_list = finder.get_empty_files();

    let mut groups = Vec::new();

    for entry in empty_list {
        let path = entry.path.to_string_lossy().to_string();
        let modified_date = if entry.modified_date > 0 {
            Some(entry.modified_date * 1000)
        } else {
            None
        };
        groups.push(FileGroup {
            size: 0,
            hash: String::new(),
            files: vec![ScannedFile {
                path,
                modified_date,
                dimensions: None,
                similarity: None,
                size: Some(0),
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
