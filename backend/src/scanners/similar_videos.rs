use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use czkawka_core::common::traits::Search;
use czkawka_core::tools::similar_videos::{
    SimilarVideos, SimilarVideosParameters, crop_detect_from_str_opt,
};

use crate::models::{FileGroup, ScanRequest, ScanResults, ScannedFile};
use crate::scanners::{configure_common_data, make_stop_flag};

pub fn run(request: ScanRequest, progress_sender: &Sender<ProgressData>) -> Result<ScanResults, String> {
    let tolerance = request.tolerance.unwrap_or(5).clamp(0, 20);
    let duration = request.vid_hash_duration.unwrap_or(10).clamp(2, 60);
    let crop_detect = request
        .crop_detect
        .as_deref()
        .and_then(crop_detect_from_str_opt)
        .unwrap_or(czkawka_core::tools::similar_videos::DEFAULT_CROP_DETECT);

    let params = SimilarVideosParameters::new(
        tolerance,
        false,
        false,
        czkawka_core::tools::similar_videos::DEFAULT_SKIP_FORWARD_AMOUNT,
        duration,
        crop_detect,
        false,
        czkawka_core::tools::similar_videos::DEFAULT_VIDEO_PERCENTAGE_FOR_THUMBNAIL,
        false,
        2,
    );

    let mut finder = SimilarVideos::new(params);
    configure_common_data(&mut finder, &request);

    let stop_flag = make_stop_flag();
    finder.search(&stop_flag, Some(progress_sender));

    let info = finder.get_information();
    let similar_vectors = finder.get_similar_videos();

    let mut groups = Vec::new();
    let mut total_files = 0;

    for vec in similar_vectors {
        if vec.len() < 2 {
            continue;
        }
        let mut files = Vec::new();
        for entry in vec {
            let path = entry.path.to_string_lossy().to_string();
            let modified_date = if entry.modified_date > 0 {
                Some(entry.modified_date * 1000)
            } else {
                None
            };
            let dimensions = if let (Some(w), Some(h)) = (entry.width, entry.height) {
                Some(format!("{}x{}", w, h))
            } else {
                None
            };
            let similarity = entry.duration.map(|d| {
                let minutes = (d / 60.0) as u32;
                let seconds = (d % 60.0) as u32;
                if minutes > 0 {
                    format!("{:02}:{:02}", minutes, seconds)
                } else {
                    format!("00:{:02}", seconds)
                }
            });
            files.push(ScannedFile {
                path,
                modified_date,
                dimensions,
                similarity,
            });
        }
        total_files += files.len();
        let group_size = files
            .first()
            .map(|f| std::fs::metadata(&f.path).map(|m| m.len()).unwrap_or(0))
            .unwrap_or(0);
        groups.push(FileGroup {
            size: group_size,
            hash: String::new(),
            files,
        });
    }

    let wasted_space: u64 = groups
        .iter()
        .map(|g| {
            if g.files.len() > 1 {
                g.size * (g.files.len() as u64 - 1)
            } else {
                0
            }
        })
        .sum();

    Ok(ScanResults {
        total_groups: groups.len(),
        total_items: total_files,
        wasted_bytes: wasted_space,
        scanning_time_ms: info.scanning_time.as_millis() as u64,
        groups,
    })
}
