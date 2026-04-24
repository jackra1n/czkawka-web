use crate::models::{ScanRequest, ScanResults};

pub fn run_scan(request: ScanRequest) -> Result<ScanResults, String> {
    match request.tool_id.as_str() {
        "big-files" => crate::scanners::big_files::run(request),
        "duplicates" => crate::scanners::duplicates::run(request),
        "empty-folders" => crate::scanners::empty_folders::run(request),
        "similar-images" => crate::scanners::similar_images::run(request),
        _ => Err(format!("Unsupported tool_id: {}", request.tool_id)),
    }
}
