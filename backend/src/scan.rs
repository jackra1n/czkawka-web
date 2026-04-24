use crate::models::{ScanRequest, ScanResults};

pub fn run_scan(request: ScanRequest) -> Result<ScanResults, String> {
    match request.tool_id.as_str() {
        "big-files" => crate::scanners::big_files::run(request),
        "duplicates" => crate::scanners::duplicates::run(request),
        "empty-files" => crate::scanners::empty_files::run(request),
        "empty-folders" => crate::scanners::empty_folders::run(request),
        "invalid-symlinks" => crate::scanners::invalid_symlinks::run(request),
        "same-music" => crate::scanners::same_music::run(request),
        "similar-images" => crate::scanners::similar_images::run(request),
        "similar-videos" => crate::scanners::similar_videos::run(request),
        "temporary" => crate::scanners::temporary::run(request),
        _ => Err(format!("Unsupported tool_id: {}", request.tool_id)),
    }
}
