use crossbeam_channel::Sender;
use czkawka_core::common::model::CheckingMethod;
use czkawka_core::common::progress_data::ProgressData;
use czkawka_core::common::traits::Search;
use czkawka_core::tools::same_music::{MusicSimilarity, SameMusic, SameMusicParameters};

use crate::models::{FileGroup, ScanRequest, ScanResults, ScannedFile};
use crate::scanners::configure_common_data;

pub fn run(
    request: ScanRequest,
    progress_sender: &Sender<ProgressData>,
    stop_flag: std::sync::Arc<std::sync::atomic::AtomicBool>,
) -> Result<ScanResults, String> {
    let check_type = request
        .music_check_type
        .as_deref()
        .map(|s| match s.to_lowercase().as_str() {
            "content" | "audio_content" | "fingerprint" => Ok(CheckingMethod::AudioContent),
            "tags" | "audio_tags" | "metadata" => Ok(CheckingMethod::AudioTags),
            _ => Err(format!("Unknown music check type: {s}")),
        })
        .transpose()?
        .unwrap_or(CheckingMethod::AudioTags);

    let music_similarity = if check_type == CheckingMethod::AudioTags {
        MusicSimilarity::TRACK_TITLE
            | MusicSimilarity::TRACK_ARTIST
            | MusicSimilarity::YEAR
            | MusicSimilarity::LENGTH
            | MusicSimilarity::GENRE
            | MusicSimilarity::BITRATE
    } else {
        MusicSimilarity::NONE
    };

    let params = SameMusicParameters::new(music_similarity, true, check_type, 5.0, 5.0, true);

    let mut finder = SameMusic::new(params);
    configure_common_data(&mut finder, &request);

    finder.search(&stop_flag, Some(progress_sender));

    let info = finder.get_information();
    let groups = finder.get_duplicated_music_entries();

    let mut music_groups = Vec::new();
    let mut total_files = 0;

    for group in groups {
        if group.len() < 2 {
            continue;
        }
        let mut files = Vec::new();
        let mut hash = String::new();
        for entry in group {
            let path = entry.path.to_string_lossy().to_string();
            let modified_date = if entry.modified_date > 0 {
                Some(entry.modified_date * 1000)
            } else {
                None
            };
            let similarity = if !entry.track_title.is_empty() || !entry.track_artist.is_empty() {
                Some(
                    format!("{} - {}", entry.track_title, entry.track_artist)
                        .trim_end_matches(" - ")
                        .to_string(),
                )
            } else {
                None
            };
            files.push(ScannedFile {
                path,
                modified_date,
                dimensions: None,
                similarity,
                size: Some(entry.size),
                exif_tags: None,
            });
            if hash.is_empty() {
                hash = format!(
                    "{}|{}|{}|{}",
                    entry.track_title, entry.track_artist, entry.year, entry.genre
                );
            }
        }
        total_files += files.len();
        music_groups.push(FileGroup {
            size: group[0].size,
            hash,
            files,
        });
    }

    let wasted_space: u64 = music_groups
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
        total_groups: music_groups.len(),
        total_items: total_files,
        wasted_bytes: wasted_space,
        scanning_time_ms: info.scanning_time.as_millis() as u64,
        groups: music_groups,
    })
}
