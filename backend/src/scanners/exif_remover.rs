use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use czkawka_core::common::traits::Search;
use czkawka_core::tools::exif_remover::ExifRemover;

use crate::models::{FileGroup, ScanRequest, ScanResults, ScannedFile};
use crate::scanners::{configure_common_data, make_stop_flag};

pub fn run(request: ScanRequest, progress_sender: &Sender<ProgressData>) -> Result<ScanResults, String> {
    let mut finder = ExifRemover::new(Default::default());
    configure_common_data(&mut finder, &request);

    let stop_flag = make_stop_flag();
    finder.search(&stop_flag, Some(progress_sender));

    let info = finder.get_information();
    let exif_files = finder.get_exif_files();

    let mut groups = Vec::new();
    let mut total_files = 0;

    for entry in exif_files {
        let path = entry.path.to_string_lossy().to_string();
        let modified_date = if entry.modified_date > 0 {
            Some(entry.modified_date * 1000)
        } else {
            None
        };
        let tag_count = entry.exif_tags.len();
        let similarity = Some(format!(
            "{tag_count} tag{}",
            if tag_count == 1 { "" } else { "s" }
        ));
        groups.push(FileGroup {
            size: entry.size,
            hash: String::new(),
            files: vec![ScannedFile {
                path,
                modified_date,
                dimensions: None,
                similarity,
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
