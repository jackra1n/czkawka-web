use czkawka_core::common::traits::Search;
use czkawka_core::tools::temporary::{Temporary, TemporaryParameters};

use crate::models::{FileGroup, ScanRequest, ScanResults, ScannedFile};
use crate::scanners::{configure_common_data, make_stop_flag};

pub fn run(request: ScanRequest) -> Result<ScanResults, String> {
    let mut finder = Temporary::new(TemporaryParameters::default());
    configure_common_data(&mut finder, &request);

    let stop_flag = make_stop_flag();
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    let temp_list = finder.get_temporary_files();

    let mut groups = Vec::new();

    for entry in temp_list {
        let path = entry.path.to_string_lossy().to_string();
        let modified_date = if entry.modified_date > 0 {
            Some(entry.modified_date * 1000)
        } else {
            None
        };
        groups.push(FileGroup {
            size: entry.size,
            hash: String::new(),
            files: vec![ScannedFile {
                path,
                modified_date,
                dimensions: None,
                similarity: None,
            }],
        });
    }

    let total_items = groups.len();

    Ok(ScanResults {
        total_groups: groups.len(),
        total_items,
        wasted_bytes: 0,
        scanning_time_ms: info.scanning_time.as_millis() as u64,
        groups,
    })
}
