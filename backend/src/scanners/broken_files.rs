use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use czkawka_core::common::traits::Search;
use czkawka_core::tools::broken_files::{BrokenFiles, BrokenFilesParameters, CheckedTypes};

use crate::models::{FileGroup, ScanRequest, ScanResults, ScannedFile};
use crate::scanners::configure_common_data;

pub fn run(
    request: ScanRequest,
    progress_sender: &Sender<ProgressData>,
    stop_flag: std::sync::Arc<std::sync::atomic::AtomicBool>,
) -> Result<ScanResults, String> {
    let mut checked_types = CheckedTypes::NONE;

    if let Some(ref types) = request.broken_file_types {
        for t in types.split(',') {
            match t.trim().to_lowercase().as_str() {
                "pdf" => checked_types |= CheckedTypes::PDF,
                "audio" => checked_types |= CheckedTypes::AUDIO,
                "image" => checked_types |= CheckedTypes::IMAGE,
                "archive" => checked_types |= CheckedTypes::ARCHIVE,
                "video" | "video_ffprobe" => checked_types |= CheckedTypes::VIDEO_FFPROBE,
                "video_ffmpeg" => checked_types |= CheckedTypes::VIDEO_FFMPEG,
                _ => {}
            }
        }
    }

    if checked_types == CheckedTypes::NONE {
        checked_types = CheckedTypes::PDF
            | CheckedTypes::AUDIO
            | CheckedTypes::IMAGE
            | CheckedTypes::ARCHIVE
            | CheckedTypes::VIDEO_FFPROBE;
    }

    let params = BrokenFilesParameters::new(checked_types);
    let mut finder = BrokenFiles::new(params);
    configure_common_data(&mut finder, &request);

    finder.search(&stop_flag, Some(progress_sender));

    let info = finder.get_information();
    let broken_files = finder.get_broken_files();

    let mut groups = Vec::new();
    let mut total_files = 0;

    for entry in broken_files {
        let path = entry.path.to_string_lossy().to_string();
        let modified_date = if entry.modified_date > 0 {
            Some(entry.modified_date * 1000)
        } else {
            None
        };
        let similarity = Some(entry.get_error_string());
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
