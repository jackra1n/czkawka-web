use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use czkawka_core::common::tool_data::CommonData;
use czkawka_core::common::traits::Search;
use czkawka_core::tools::bad_extensions::{BadExtensions, BadExtensionsParameters};

use crate::models::{FileGroup, ScanRequest, ScanResults, ScannedFile};
use crate::scanners::configure_common_data;

pub fn run(
    request: ScanRequest,
    progress_sender: &Sender<ProgressData>,
    stop_flag: std::sync::Arc<std::sync::atomic::AtomicBool>,
) -> Result<ScanResults, String> {
    let params = BadExtensionsParameters {
        include_files_without_extension: request.include_files_without_extension.unwrap_or(false),
    };

    let mut finder = BadExtensions::new(params);
    configure_common_data(&mut finder, &request);

    finder.search(&stop_flag, Some(progress_sender));

    let info = finder.get_information();
    let bad_files = finder.get_bad_extensions_files();

    let mut groups = Vec::new();
    let mut total_files = 0;

    for entry in bad_files {
        let path = entry.path.to_string_lossy().to_string();
        let modified_date = if entry.modified_date > 0 {
            Some(entry.modified_date * 1000)
        } else {
            None
        };
        let similarity = Some(format!(
            "{} -> {}",
            if entry.current_extension.is_empty() {
                "(none)"
            } else {
                &entry.current_extension
            },
            entry.proper_extension
        ));
        groups.push(FileGroup {
            size: entry.size,
            hash: String::new(),
            files: vec![ScannedFile {
                path,
                modified_date,
                dimensions: None,
                similarity,
                size: Some(entry.size),
                exif_tags: None,
            }],
        });
        total_files += 1;
    }

    Ok(ScanResults {
        total_groups: groups.len(),
        total_items: total_files,
        wasted_bytes: 0,
        scanning_time_ms: info.scanning_time.as_millis() as u64,
        groups,
    })
}
