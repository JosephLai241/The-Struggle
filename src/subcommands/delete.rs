//! Contains all functionality pertaining to the `delete` subcommand.

use std::collections::{BTreeMap, HashMap};

use ansi_term::{Color, Style};
use diesel::SqliteConnection;
use inquire::Confirm;
use regex::Regex;

use crate::{
    errors::FettersError,
    models::{config::FettersSettings, search::SearchResult},
    sqlite::{self},
    utils,
};

/// Delete a job from the SQLite database.
pub fn delete_job(
    connection: &mut SqliteConnection,
    fetters_settings: &FettersSettings,
    id: Option<i32>,
    query: Option<String>,
    skip_confirmation: bool,
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

    let mut matched_jobs: BTreeMap<Option<i32>, SearchResult> = BTreeMap::new();

    if let Some(id) = id {
        if query.is_some() {
            println!(
                "\n{}\n",
                Color::Fixed(172)
                    .bold()
                    .paint(format!("An ID and query were both provided! The ID will take precedence over the query."))
            );
        }

        if let Some(selected_job) = all_jobs.iter().find(|job| job.id == Some(id)) {
            let stint_name = match selected_job.stint {
                Some(id) => mapped_stints
                    .get(&Some(id))
                    .map(|stint| stint.stint.clone())
                    .unwrap_or("".to_string()),
                None => "".to_string(),
            };

            let search_result =
                SearchResult::new_with_job_and_stint(selected_job.clone(), stint_name);
            matched_jobs.insert(search_result.job.id, search_result);

            println!(
                "\n{}\n",
                Color::Cyan.bold().paint("Displaying selected job.")
            );

            utils::display::display_queried_jobs(matched_jobs, &status_mappings, &mut table);

            if skip_confirmation {
                println!("{}\n", Color::Cyan.bold().paint("DELETING JOB."));

                sqlite::queries::delete_job(connection, id)?;

                println!(
                    "{}\n",
                    Color::Green
                        .bold()
                        .paint("🚮 Successfully deleted the job!")
                );
            } else {
                if let Some(confirm_deletion) = Confirm::new("Confirm deletion?")
                    .with_default(true)
                    .prompt_skippable()
                    .ok()
                    .flatten()
                {
                    if confirm_deletion {
                        println!("\n{}\n", Color::Cyan.bold().paint("DELETING JOB."));

                        sqlite::queries::delete_job(connection, id)?;

                        println!(
                            "{}\n",
                            Color::Green
                                .bold()
                                .paint("🚮 Successfully deleted the job!")
                        );
                    } else {
                        println!("\n{}\n", Color::Red.bold().paint("CANCELLING."));
                    }
                } else {
                    println!("\n{}\n", Color::Red.bold().paint("CANCELLING."));
                }
            }
        } else {
            println!(
                "\n{}\n",
                Color::Red
                    .bold()
                    .paint(format!("NO JOB WITH ID {id} FOUND!"))
            );
        }
    } else if let Some(pattern) = query {
        let regex = Regex::new(&pattern)?;

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

        if matched_jobs.is_empty() {
            println!("\n{}\n", Color::Red.bold().paint("No matches were found!"));
        } else {
            utils::display::display_queried_jobs(matched_jobs, &status_mappings, &mut table);
        }
    }

    Ok(())
}
