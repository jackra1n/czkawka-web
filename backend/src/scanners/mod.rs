use std::path::PathBuf;

use czkawka_core::common::model::CheckingMethod;
use czkawka_core::common::progress_data::{
    CacheLoadPhase, DuplicateStage, ExifRemoverStage, ProgressData, SameMusicStage,
    SimilarImagesStage, SimilarVideosStage, ToolStage, VideoOptimizerStage,
};
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

pub fn stage_label(stage: ToolStage) -> String {
    match stage {
        ToolStage::CollectingFiles(CheckingMethod::Name) => "Scanning by name".to_string(),
        ToolStage::CollectingFiles(CheckingMethod::SizeName) => {
            "Scanning by size and name".to_string()
        }
        ToolStage::CollectingFiles(CheckingMethod::Size) => "Scanning by size".to_string(),
        ToolStage::CollectingFiles(_) => "Collecting files".to_string(),
        ToolStage::CollectingFolders => "Collecting folders".to_string(),

        ToolStage::DeletingFiles => "Deleting files".to_string(),
        ToolStage::RenamingFiles => "Renaming files".to_string(),
        ToolStage::MovingFiles => "Moving files".to_string(),
        ToolStage::HardlinkingFiles => "Creating hard links".to_string(),
        ToolStage::SymlinkingFiles => "Creating symlinks".to_string(),
        ToolStage::OptimizingVideos => "Optimizing videos".to_string(),
        ToolStage::CleaningExif => "Cleaning EXIF data".to_string(),

        ToolStage::Duplicate(stage) => match stage {
            DuplicateStage::HidingHardLinks => "Hiding hard links".to_string(),
            DuplicateStage::LoadingPreHashCache(CacheLoadPhase::Loading) => {
                "Loading pre-hash cache".to_string()
            }
            DuplicateStage::LoadingPreHashCache(CacheLoadPhase::FilteringOutdated) => {
                "Filtering outdated cache entries".to_string()
            }
            DuplicateStage::PreHashing => "Pre-hashing files".to_string(),
            DuplicateStage::SavingPreHashCache => "Saving pre-hash cache".to_string(),
            DuplicateStage::LoadingHashCache(CacheLoadPhase::Loading) => {
                "Loading cache".to_string()
            }
            DuplicateStage::LoadingHashCache(CacheLoadPhase::FilteringOutdated) => {
                "Filtering outdated cache entries".to_string()
            }
            DuplicateStage::FullHashing => "Hashing files".to_string(),
            DuplicateStage::SavingHashCache => "Saving cache".to_string(),
        },

        ToolStage::SameMusic(_, stage) => match stage {
            SameMusicStage::LoadingTagsCache(CacheLoadPhase::Loading) => {
                "Loading tag cache".to_string()
            }
            SameMusicStage::LoadingTagsCache(CacheLoadPhase::FilteringOutdated) => {
                "Filtering outdated cache entries".to_string()
            }
            SameMusicStage::ReadingTags => "Reading music tags".to_string(),
            SameMusicStage::SavingTagsCache => "Saving tag cache".to_string(),
            SameMusicStage::ComparingTags => "Comparing tags".to_string(),
            SameMusicStage::LoadingFingerprintCache(CacheLoadPhase::Loading) => {
                "Loading fingerprint cache".to_string()
            }
            SameMusicStage::LoadingFingerprintCache(CacheLoadPhase::FilteringOutdated) => {
                "Filtering outdated cache entries".to_string()
            }
            SameMusicStage::CalculatingFingerprints => "Calculating fingerprints".to_string(),
            SameMusicStage::SavingFingerprintCache => "Saving fingerprint cache".to_string(),
            SameMusicStage::ComparingFingerprints => "Comparing fingerprints".to_string(),
        },

        ToolStage::SimilarImages(stage) => match stage {
            SimilarImagesStage::HidingHardLinks => "Hiding hard links".to_string(),
            SimilarImagesStage::CalculatingHashes => "Calculating image hashes".to_string(),
            SimilarImagesStage::ComparingHashes => "Comparing image hashes".to_string(),
        },

        ToolStage::SimilarVideos(_, stage) => match stage {
            SimilarVideosStage::HidingHardLinks => "Hiding hard links".to_string(),
            SimilarVideosStage::CalculatingHashes => "Calculating video hashes".to_string(),
            SimilarVideosStage::CreatingThumbnails => "Creating video thumbnails".to_string(),
            SimilarVideosStage::LoadingAudioCache(CacheLoadPhase::Loading) => {
                "Loading audio cache".to_string()
            }
            SimilarVideosStage::LoadingAudioCache(CacheLoadPhase::FilteringOutdated) => {
                "Filtering outdated cache entries".to_string()
            }
            SimilarVideosStage::CalculatingAudioFingerprints => {
                "Calculating audio fingerprints".to_string()
            }
            SimilarVideosStage::SavingAudioCache => "Saving audio cache".to_string(),
            SimilarVideosStage::ComparingAudioFingerprints => {
                "Comparing audio fingerprints".to_string()
            }
            SimilarVideosStage::CreatingAudioThumbnails => "Creating audio thumbnails".to_string(),
        },

        ToolStage::ExifRemover(stage) => match stage {
            ExifRemoverStage::LoadingCache(CacheLoadPhase::Loading) => {
                "Loading EXIF cache".to_string()
            }
            ExifRemoverStage::LoadingCache(CacheLoadPhase::FilteringOutdated) => {
                "Filtering outdated cache entries".to_string()
            }
            ExifRemoverStage::ExtractingTags => "Extracting EXIF tags".to_string(),
            ExifRemoverStage::SavingCache => "Saving EXIF cache".to_string(),
        },

        ToolStage::VideoOptimizer(stage) => match stage {
            VideoOptimizerStage::CreatingThumbnails => "Creating thumbnails".to_string(),
            VideoOptimizerStage::ProcessingVideos => "Processing videos".to_string(),
        },

        ToolStage::BrokenFilesChecking => "Checking files".to_string(),
        ToolStage::BadExtensionsChecking => "Checking extensions".to_string(),
        ToolStage::BadNamesChecking => "Checking names".to_string(),
        ToolStage::EmptyFilesCheckingContent => "Checking file content".to_string(),
    }
}

pub fn progress_to_scan_progress(data: &ProgressData) -> ScanProgress {
    ScanProgress {
        stage_label: stage_label(data.stage),
        current_stage_idx: data.stage.current_stage_idx(),
        max_stage_idx: data.stage.max_stage_idx(),
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
