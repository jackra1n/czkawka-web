use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use czkawka_core::common::tool_data::CommonData;
use czkawka_core::common::traits::Search;
use czkawka_core::tools::big_file::{BigFile, BigFileParameters, SearchMode};

use crate::models::{FileGroup, ScanRequest, ScanResults, ScannedFile};
use crate::scanners::configure_common_data;

pub fn run(
    request: ScanRequest,
    progress_sender: &Sender<ProgressData>,
    stop_flag: std::sync::Arc<std::sync::atomic::AtomicBool>,
) -> Result<ScanResults, String> {
    let number_of_files = request.number_of_files.unwrap_or(50).max(1) as usize;
    let search_mode = match request.search_mode.as_deref() {
        Some("smallest") => SearchMode::SmallestFiles,
        _ => SearchMode::BiggestFiles,
    };

    let params = BigFileParameters::new(number_of_files, search_mode);
    let mut finder = BigFile::new(params);
    configure_common_data(&mut finder, &request);

    finder.search(&stop_flag, Some(progress_sender));

    let info = finder.get_information();
    let big_files = finder.get_big_files();

    let mut groups = Vec::new();

    for entry in big_files {
        let path = entry.path.to_string_lossy().to_string();
        let modified_date = if entry.modified_date > 0 {
            Some(entry.modified_date * 1000)
        } else {
            None
        };
        groups.push(FileGroup {
            size: entry.size,
            hash: String::new(),
            files: vec![ScannedFile {
                path,
                modified_date,
                dimensions: None,
                similarity: None,
                size: Some(entry.size),
                exif_tags: None,
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
