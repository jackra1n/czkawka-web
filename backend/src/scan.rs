use crossbeam_channel::unbounded;
use czkawka_core::common::progress_data::ProgressData;

use crate::models::{ScanRequest, ScanResults, SharedProgress};
use crate::scanners::spawn_progress_reader;

pub fn run_scan(
    request: ScanRequest,
    shared_progress: SharedProgress,
) -> Result<ScanResults, String> {
    let (sender, receiver) = unbounded::<ProgressData>();

    let reader_handle = spawn_progress_reader(receiver, shared_progress);

    let result = match request.tool_id.as_str() {
        "bad-extensions" => crate::scanners::bad_extensions::run(request, &sender),
        "bad-names" => crate::scanners::bad_names::run(request, &sender),
        "big-files" => crate::scanners::big_files::run(request, &sender),
        "broken-files" => crate::scanners::broken_files::run(request, &sender),
        "duplicates" => crate::scanners::duplicates::run(request, &sender),
        "empty-files" => crate::scanners::empty_files::run(request, &sender),
        "empty-folders" => crate::scanners::empty_folders::run(request, &sender),
        "exif-remover" => crate::scanners::exif_remover::run(request, &sender),
        "invalid-symlinks" => crate::scanners::invalid_symlinks::run(request, &sender),
        "same-music" => crate::scanners::same_music::run(request, &sender),
        "similar-images" => crate::scanners::similar_images::run(request, &sender),
        "similar-videos" => crate::scanners::similar_videos::run(request, &sender),
        "temporary" => crate::scanners::temporary::run(request, &sender),
        _ => Err(format!("Unsupported tool_id: {}", request.tool_id)),
    };

    drop(sender);
    let _ = reader_handle.join();

    result
}
