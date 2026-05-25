use std::path::PathBuf;

use czkawka_core::common::progress_data::{CurrentStage, ProgressData};
use czkawka_core::common::tool_data::{CommonData, DeleteMethod};

use crate::models::{ScanProgress, ScanRequest, SharedProgress};

pub mod bad_extensions;
pub mod bad_names;
pub mod big_files;
pub mod broken_files;
pub mod duplicates;
pub mod empty_files;
pub mod empty_folders;
pub mod exif_remover;
pub mod invalid_symlinks;
pub mod same_music;
pub mod similar_images;
pub mod similar_videos;
pub mod temporary;

pub fn configure_common_data<T: CommonData>(tool: &mut T, request: &ScanRequest) {
    let included: Vec<PathBuf> = request.directories.iter().map(PathBuf::from).collect();
    let excluded: Vec<PathBuf> = request
        .exclude_directories
        .iter()
        .map(PathBuf::from)
        .collect();

    tool.set_included_paths(included);
    tool.set_excluded_paths(excluded);

    let excluded_items: Vec<String> = request
        .excluded_items
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    if !excluded_items.is_empty() {
        tool.set_excluded_items(excluded_items);
    }

    tool.set_minimal_file_size(request.min_file_size);
    tool.set_recursive_search(true);
    tool.set_delete_method(DeleteMethod::None);
    tool.set_dry_run(true);
    tool.set_hide_hard_links(request.hide_hard_links);
}

pub fn stage_label(stage: CurrentStage) -> String {
    match stage {
        CurrentStage::CollectingFiles => "Collecting files",
        CurrentStage::DeletingFiles => "Deleting files",
        CurrentStage::RenamingFiles => "Renaming files",
        CurrentStage::MovingFiles => "Moving files",
        CurrentStage::HardlinkingFiles => "Creating hard links",
        CurrentStage::SymlinkingFiles => "Creating symlinks",
        CurrentStage::OptimizingVideos => "Optimizing videos",
        CurrentStage::CleaningExif => "Cleaning EXIF data",

        CurrentStage::DuplicateCacheSaving => "Saving cache",
        CurrentStage::DuplicateCacheLoading => "Loading cache",
        CurrentStage::DuplicatePreHashCacheSaving => "Saving pre-hash cache",
        CurrentStage::DuplicatePreHashCacheLoading => "Loading pre-hash cache",
        CurrentStage::DuplicateScanningName => "Scanning by name",
        CurrentStage::DuplicateScanningSizeName => "Scanning by size and name",
        CurrentStage::DuplicateScanningSize => "Scanning by size",
        CurrentStage::DuplicateHidingHardLinks => "Hiding hard links",
        CurrentStage::DuplicatePreHashing => "Pre-hashing files",
        CurrentStage::DuplicateFullHashing => "Hashing files",

        CurrentStage::SameMusicCacheSavingTags => "Saving tag cache",
        CurrentStage::SameMusicCacheLoadingTags => "Loading tag cache",
        CurrentStage::SameMusicCacheSavingFingerprints => "Saving fingerprint cache",
        CurrentStage::SameMusicCacheLoadingFingerprints => "Loading fingerprint cache",
        CurrentStage::SameMusicReadingTags => "Reading music tags",
        CurrentStage::SameMusicCalculatingFingerprints => "Calculating fingerprints",
        CurrentStage::SameMusicComparingTags => "Comparing tags",
        CurrentStage::SameMusicComparingFingerprints => "Comparing fingerprints",

        CurrentStage::SimilarImagesHidingHardLinks => "Hiding hard links",
        CurrentStage::SimilarImagesCalculatingHashes => "Calculating image hashes",
        CurrentStage::SimilarImagesComparingHashes => "Comparing image hashes",

        CurrentStage::SimilarVideosHidingHardLinks => "Hiding hard links",
        CurrentStage::SimilarVideosCalculatingHashes => "Calculating video hashes",
        CurrentStage::SimilarVideosCreatingThumbnails => "Creating video thumbnails",

        CurrentStage::BrokenFilesChecking => "Checking files",
        CurrentStage::BadExtensionsChecking => "Checking extensions",
        CurrentStage::BadNamesChecking => "Checking names",

        CurrentStage::ExifRemoverCacheLoading => "Loading EXIF cache",
        CurrentStage::ExifRemoverExtractingTags => "Extracting EXIF tags",
        CurrentStage::ExifRemoverCacheSaving => "Saving EXIF cache",

        CurrentStage::VideoOptimizerCreatingThumbnails => "Creating thumbnails",
        CurrentStage::VideoOptimizerProcessingVideos => "Processing videos",
    }
    .to_string()
}

pub fn progress_to_scan_progress(data: &ProgressData) -> ScanProgress {
    ScanProgress {
        stage_label: stage_label(data.sstage),
        current_stage_idx: data.current_stage_idx,
        max_stage_idx: data.max_stage_idx,
        entries_checked: data.entries_checked,
        entries_to_check: data.entries_to_check,
        bytes_checked: data.bytes_checked,
        bytes_to_check: data.bytes_to_check,
    }
}

pub fn spawn_progress_reader(
    receiver: crossbeam_channel::Receiver<ProgressData>,
    shared_progress: SharedProgress,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        while let Ok(data) = receiver.recv() {
            let scan_progress = progress_to_scan_progress(&data);
            if let Ok(mut progress) = shared_progress.lock() {
                *progress = Some(scan_progress);
            }
        }
    })
}
