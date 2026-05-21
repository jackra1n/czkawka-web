use crossbeam_channel::unbounded;
use czkawka_core::common::progress_data::ProgressData;

use crate::models::{ScanRequest, ScanResults, SharedProgress};
use crate::scanners::spawn_progress_reader;

pub fn run_scan(
    request: ScanRequest,
    shared_progress: SharedProgress,
    stop_flag: std::sync::Arc<std::sync::atomic::AtomicBool>,
) -> Result<ScanResults, String> {
    let (sender, receiver) = unbounded::<ProgressData>();

    let reader_handle = spawn_progress_reader(receiver, shared_progress);

    let result = match request.tool_id.as_str() {
        "bad-extensions" => crate::scanners::bad_extensions::run(request, &sender, stop_flag),
        "bad-names" => crate::scanners::bad_names::run(request, &sender, stop_flag),
        "big-files" => crate::scanners::big_files::run(request, &sender, stop_flag),
        "broken-files" => crate::scanners::broken_files::run(request, &sender, stop_flag),
        "duplicates" => crate::scanners::duplicates::run(request, &sender, stop_flag),
        "empty-files" => crate::scanners::empty_files::run(request, &sender, stop_flag),
        "empty-folders" => crate::scanners::empty_folders::run(request, &sender, stop_flag),
        "exif-remover" => crate::scanners::exif_remover::run(request, &sender, stop_flag),
        "invalid-symlinks" => crate::scanners::invalid_symlinks::run(request, &sender, stop_flag),
        "same-music" => crate::scanners::same_music::run(request, &sender, stop_flag),
        "similar-images" => crate::scanners::similar_images::run(request, &sender, stop_flag),
        "similar-videos" => crate::scanners::similar_videos::run(request, &sender, stop_flag),
        "temporary" => crate::scanners::temporary::run(request, &sender, stop_flag),
        _ => Err(format!("Unsupported tool_id: {}", request.tool_id)),
    };

    drop(sender);
    let _ = reader_handle.join();

    result
}
