use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use czkawka_core::common::traits::Search;
use czkawka_core::tools::bad_names::{BadNames, BadNamesParameters, NameIssues};

use crate::models::{FileGroup, ScanRequest, ScanResults, ScannedFile};
use crate::scanners::{configure_common_data, make_stop_flag};

pub fn run(request: ScanRequest, progress_sender: &Sender<ProgressData>) -> Result<ScanResults, String> {
    let checked_issues = NameIssues {
        uppercase_extension: request.bad_name_uppercase_extension.unwrap_or(true),
        emoji_used: request.bad_name_emoji.unwrap_or(true),
        space_at_start_or_end: request.bad_name_spaces.unwrap_or(true),
        non_ascii_graphical: request.bad_name_non_ascii.unwrap_or(true),
        restricted_charset_allowed: if request.bad_name_restricted_charset.unwrap_or(false) {
            let chars: Vec<char> = request
                .bad_name_allowed_chars
                .as_deref()
                .unwrap_or("_- .")
                .chars()
                .collect();
            Some(chars)
        } else {
            None
        },
        remove_duplicated_non_alphanumeric: request.bad_name_dedupe_non_alnum.unwrap_or(false),
    };

    let params = BadNamesParameters::new(checked_issues);
    let mut finder = BadNames::new(params);
    configure_common_data(&mut finder, &request);

    let stop_flag = make_stop_flag();
    finder.search(&stop_flag, Some(progress_sender));

    let info = finder.get_information();
    let bad_names = finder.get_bad_names_files();

    let mut groups = Vec::new();
    let mut total_files = 0;

    for entry in bad_names {
        let path = entry.path.to_string_lossy().to_string();
        let modified_date = if entry.modified_date > 0 {
            Some(entry.modified_date * 1000)
        } else {
            None
        };
        let similarity = Some(format!("-> {}", entry.new_name));
        groups.push(FileGroup {
            size: entry.size,
            hash: String::new(),
            files: vec![ScannedFile {
                path,
                modified_date,
                dimensions: None,
                similarity,
            }],
        });
        total_files += 1;
    }

    Ok(ScanResults {
        total_groups: groups.len(),
        total_items: total_files,
        wasted_bytes: 0,
        scanning_time_ms: info.scanning_time.as_millis() as u64,
        groups,
    })
}
