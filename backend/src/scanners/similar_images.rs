use czkawka_core::common::traits::Search;
use czkawka_core::re_exported::{FilterType, HashAlg};
use czkawka_core::tools::similar_images::core::get_string_from_similarity;
use czkawka_core::tools::similar_images::{SimilarImages, SimilarImagesParameters};

use crate::models::{FileGroup, ScanRequest, ScanResults, ScannedFile};
use crate::scanners::{configure_common_data, make_stop_flag};

fn parse_hash_alg(s: &str) -> Result<HashAlg, String> {
    match s.to_lowercase().as_str() {
        "mean" => Ok(HashAlg::Mean),
        "gradient" => Ok(HashAlg::Gradient),
        "blockhash" => Ok(HashAlg::Blockhash),
        "vertgradient" | "vert_gradient" | "vert gradient" => Ok(HashAlg::VertGradient),
        "doublegradient" | "double_gradient" | "double gradient" => Ok(HashAlg::DoubleGradient),
        "median" => Ok(HashAlg::Median),
        _ => Err(format!("Unknown hash algorithm: {s}")),
    }
}

fn parse_filter_type(s: &str) -> Result<FilterType, String> {
    match s.to_lowercase().as_str() {
        "lanczos3" => Ok(FilterType::Lanczos3),
        "nearest" => Ok(FilterType::Nearest),
        "triangle" => Ok(FilterType::Triangle),
        "gaussian" => Ok(FilterType::Gaussian),
        "catmullrom" | "catmull_rom" | "catmull rom" => Ok(FilterType::CatmullRom),
        _ => Err(format!("Unknown resize filter: {s}")),
    }
}

pub fn run(request: ScanRequest) -> Result<ScanResults, String> {
    let hash_alg = request
        .hash_alg
        .as_deref()
        .map(parse_hash_alg)
        .transpose()?
        .unwrap_or(HashAlg::Gradient);

    let hash_size = request.hash_size.unwrap_or(16);
    if ![8, 16, 32, 64].contains(&hash_size) {
        return Err(format!("Invalid hash size: {hash_size}. Must be 8, 16, 32, or 64."));
    }

    let image_filter = request
        .resize_filter
        .as_deref()
        .map(parse_filter_type)
        .transpose()?
        .unwrap_or(FilterType::Lanczos3);

    let max_difference = request.similarity.unwrap_or(5).min(40);

    let params = SimilarImagesParameters::new(
        max_difference,
        hash_size,
        hash_alg,
        image_filter,
        false,
        false,
    );

    let mut finder = SimilarImages::new(params);
    configure_common_data(&mut finder, &request);

    let stop_flag = make_stop_flag();
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    let similar_vectors = finder.get_similar_images();

    let mut groups = Vec::new();
    let mut total_files = 0;

    for vec in similar_vectors {
        if vec.len() < 2 {
            continue;
        }
        let mut files = Vec::new();
        for entry in vec {
            let path = entry.path.to_string_lossy().to_string();
            let modified_date = std::fs::metadata(&entry.path)
                .and_then(|m| m.modified())
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_millis() as u64);
            let dimensions = if entry.width > 0 && entry.height > 0 {
                Some(format!("{}x{}", entry.width, entry.height))
            } else {
                None
            };
            let similarity = Some(get_string_from_similarity(
                entry.difference,
                hash_size,
            ));
            files.push(ScannedFile {
                path,
                modified_date,
                dimensions,
                similarity,
            });
        }
        total_files += files.len();
        let group_size = files.first().map(|f| {
            std::fs::metadata(&f.path).map(|m| m.len()).unwrap_or(0)
        }).unwrap_or(0);
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
        total_duplicate_groups: groups.len(),
        total_duplicate_files: total_files,
        wasted_space_bytes: wasted_space,
        scanning_time_ms: info.scanning_time.as_millis() as u64,
        groups,
    })
}
