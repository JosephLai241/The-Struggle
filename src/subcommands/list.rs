//! Contains all functionality pertaining to the `list` subcommand.

use std::collections::{BTreeMap, HashMap};

use ansi_term::Style;
use diesel::SqliteConnection;
use regex::Regex;

use crate::{
    errors::FettersError,
    models::{config::FettersSettings, search::SearchResult},
    sqlite, utils,
};

/// List all jobs or query for jobs that are stored in the SQLite instance.
pub fn list_jobs(
    connection: &mut SqliteConnection,
    fetters_settings: &FettersSettings,
    query: Option<String>,
) -> Result<(), FettersError> {
    let mut table = utils::display::instantiate_table(fetters_settings);

    let all_jobs = sqlite::queries::get_all_jobs(connection)?;
    let all_stints = sqlite::queries::get_all_stints(connection)?;

    let mapped_stints = all_stints
        .into_iter()
        .fold(HashMap::new(), |mut hashmap, stint| {
            hashmap.insert(stint.id, stint);
            hashmap
        });

    let status_mappings = fetters_settings.get_status_mappings_and_colors();

    if let Some(pattern) = query {
        let regex = Regex::new(&pattern)?;
        let mut matched_jobs: BTreeMap<Option<i32>, SearchResult> = BTreeMap::new();

        let highlight_style = fetters_settings.get_match_color_style();

        for job in all_jobs {
            let status_style = status_mappings
                .get(&job.status)
                .unwrap_or(&Style::default())
                .to_owned();

            let stint_name = match job.stint {
                Some(id) => mapped_stints
                    .get(&Some(id))
                    .map(|stint| stint.stint.clone())
                    .unwrap_or("".to_string()),
                None => "".to_string(),
            };
            let mut search_result = SearchResult::new_with_job_and_stint(job.clone(), stint_name);

            let (colorized_company, matched_company) = utils::display::colorize_matching_substrings(
                highlight_style,
                job.company,
                &regex,
                status_style,
            );
            let (colorized_notes, matched_notes) = utils::display::colorize_matching_substrings(
                highlight_style,
                job.notes.unwrap_or("".to_string()),
                &regex,
                status_style,
            );
            let (colorized_stint, matched_stint) = utils::display::colorize_matching_substrings(
                highlight_style,
                mapped_stints
                    .get(&job.stint)
                    .map(|stint| stint.stint.clone())
                    .unwrap_or("".to_string()),
                &regex,
                status_style,
            );
            let (colorized_title, matched_title) = utils::display::colorize_matching_substrings(
                highlight_style,
                job.title,
                &regex,
                status_style,
            );

            if matched_company || matched_notes || matched_stint || matched_title {
                search_result.job.company = colorized_company;
                search_result.job.notes = Some(colorized_notes);
                search_result.painted_stint_name = colorized_stint;
                search_result.job.title = colorized_title;

                matched_jobs.insert(search_result.job.id, search_result);
            }
        }

        utils::display::display_queried_jobs(matched_jobs, &status_mappings, &mut table);
    } else {
        utils::display::display_all_jobs(all_jobs, &mapped_stints, &status_mappings, &mut table);
    }

    Ok(())
}
