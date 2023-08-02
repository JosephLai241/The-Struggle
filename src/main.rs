//! `fetters` - A command-line tool for tracking your job applications.

// Disables error variant lint raised for `errors.rs`.
#![allow(clippy::enum_variant_names)]
use std::collections::{btree_map::Entry, BTreeMap, HashMap};

use ansi_term::Color;
use clap::Parser;
use cli::{Args, Subcommands};
use errors::FettersError;
use lazy_static::lazy_static;
use regex::Regex;

use crate::models::{job::Job, search::SearchResult};

mod cli;
mod errors;
mod models;
mod prompts;
mod sqlite;
mod utils;

lazy_static! {
    /// The ASCII art for `fetters`.
    static ref ASCII_ART: &'static [u8; 1204] = include_bytes!("../art.txt");
}

/// Run `fetters`.
fn main() -> Result<(), FettersError> {
    let fetters_settings = utils::config::configure_fetters()?;

    let args = Args::parse();

    if args.banner {
        println!(
            "{}",
            Color::Red
                .bold()
                .paint(String::from_utf8_lossy(&ASCII_ART[..]))
        );
    } else if let Some(subcommand) = args.subcommand {
        let connection = sqlite::setup::open_sqlite()?;

        match subcommand {
            Subcommands::Add {
                company,
                link,
                notes,
                skip,
                status,
                stint,
                title,
            } => {
                let title = match title {
                    Some(title) => title,
                    None => prompts::add::add_job_title(&fetters_settings)?,
                };
                let status = match status {
                    Some(status) => status,
                    None => prompts::add::add_job_status(&fetters_settings)?,
                };

                let notes = match notes {
                    Some(notes) => Some(notes),
                    None => {
                        if skip {
                            None
                        } else {
                            prompts::add::add_job_notes()?
                        }
                    }
                };

                let link = match link {
                    Some(link) => Some(link),
                    None => {
                        if skip {
                            None
                        } else {
                            prompts::add::add_job_link()?
                        }
                    }
                };

                let stint = match stint {
                    Some(stint_name) => sqlite::queries::get_all_stints(&connection)?
                        .iter()
                        .find(|stint| stint.stint == stint_name)
                        .map(|stint| stint.id.unwrap_or(0)),
                    None => {
                        if skip {
                            None
                        } else {
                            prompts::add::add_job_stint(&connection)?
                        }
                    }
                };

                let new_job = Job::new(company, link, notes, status, stint, title);

                sqlite::queries::write_job(&connection, new_job)?;

                println!(
                    "\n{}\n",
                    Color::Green
                        .bold()
                        .paint("💯 Successfully added a new job!")
                );
            }
            Subcommands::Delete {
                query,
                links,
                notes,
                stint,
                titles,
            } => {}
            Subcommands::Insights => {}
            Subcommands::List {
                mut company,
                query,
                mut links,
                mut notes,
                mut status,
                mut stint,
                mut titles,
            } => {
                if query.is_some()
                    && (company, links, notes, status, stint, titles)
                        == (false, false, false, false, false, false)
                {
                    company = true;
                    links = true;
                    notes = true;
                    status = true;
                    stint = true;
                    titles = true;
                }

                let mut table = utils::display::instantiate_table(&fetters_settings);

                let all_jobs = sqlite::queries::get_all_jobs(&connection)?;
                let all_stints = sqlite::queries::get_all_stints(&connection)?;

                let mapped_stints =
                    all_stints
                        .into_iter()
                        .fold(HashMap::new(), |mut hashmap, stint| {
                            hashmap.insert(stint.id, stint);
                            hashmap
                        });

                if let Some(pattern) = query {
                    let regex = Regex::new(&pattern)?;
                    let mut matched_jobs: BTreeMap<Option<i32>, SearchResult> = BTreeMap::new();

                    for job in all_jobs {
                        let stint_name = match job.stint {
                            Some(id) => mapped_stints
                                .get(&Some(id))
                                .map(|stint| stint.stint.clone())
                                .unwrap_or("".to_string()),
                            None => "".to_string(),
                        };
                        let mut search_result =
                            SearchResult::new_with_job_and_stint(job.clone(), stint_name);

                        if company {
                            let colorized_match = utils::display::colorize_matching_substrings(
                                &fetters_settings,
                                job.company,
                                &regex,
                            );

                            search_result.job.company = colorized_match;

                            matched_jobs
                                .entry(search_result.job.id)
                                .or_insert_with(|| search_result.clone());
                        }

                        if let (Some(link), true) = (job.link, links) {
                            let colorized_match = utils::display::colorize_matching_substrings(
                                &fetters_settings,
                                link,
                                &regex,
                            );

                            search_result.job.link = Some(colorized_match.clone());
                            search_result.matched_link = true;

                            if let Entry::Vacant(entry) = matched_jobs.entry(search_result.job.id) {
                                entry.insert(search_result.clone());
                            } else if let Some(mut existing_match) =
                                matched_jobs.get_mut(&search_result.job.id)
                            {
                                existing_match.job.link = Some(colorized_match);
                                existing_match.matched_link = true;
                            }
                        }

                        if let (Some(notes), true) = (job.notes, notes) {
                            let colorized_match = utils::display::colorize_matching_substrings(
                                &fetters_settings,
                                notes,
                                &regex,
                            );

                            search_result.job.notes = Some(colorized_match.clone());
                            search_result.matched_notes = true;

                            if let Entry::Vacant(entry) = matched_jobs.entry(search_result.job.id) {
                                entry.insert(search_result.clone());
                            } else if let Some(mut existing_match) =
                                matched_jobs.get_mut(&search_result.job.id)
                            {
                                existing_match.job.notes = Some(colorized_match.clone());
                                existing_match.matched_notes = true;
                            }
                        }

                        if status {
                            let colorized_match = utils::display::colorize_matching_substrings(
                                &fetters_settings,
                                job.status,
                                &regex,
                            );

                            search_result.job.status = colorized_match.clone();
                            search_result.matched_status = true;

                            if let Entry::Vacant(entry) = matched_jobs.entry(search_result.job.id) {
                                entry.insert(search_result.clone());
                            } else if let Some(mut existing_match) =
                                matched_jobs.get_mut(&search_result.job.id)
                            {
                                existing_match.job.status = colorized_match.clone();
                                existing_match.matched_status = true;
                            }
                        }

                        if let (Some(stint_id), true) = (job.stint, stint) {
                            if let Some(stint) = mapped_stints.get(&Some(stint_id)) {
                                let stint_name = stint.stint.clone();

                                let colorized_match = utils::display::colorize_matching_substrings(
                                    &fetters_settings,
                                    stint_name.clone(),
                                    &regex,
                                );

                                search_result.painted_stint_name = colorized_match.clone();
                                search_result.matched_stint = true;

                                if let Entry::Vacant(entry) =
                                    matched_jobs.entry(search_result.job.id)
                                {
                                    entry.insert(search_result.clone());
                                } else if let Some(mut existing_match) =
                                    matched_jobs.get_mut(&search_result.job.id)
                                {
                                    existing_match.painted_stint_name = colorized_match.clone();
                                    existing_match.matched_stint = true;
                                }
                            }
                        }

                        if titles {
                            let colorized_match = utils::display::colorize_matching_substrings(
                                &fetters_settings,
                                job.title,
                                &regex,
                            );

                            search_result.job.title = colorized_match.clone();
                            search_result.matched_title = true;

                            if let Entry::Vacant(entry) = matched_jobs.entry(search_result.job.id) {
                                entry.insert(search_result.clone());
                            } else if let Some(mut existing_match) =
                                matched_jobs.get_mut(&search_result.job.id)
                            {
                                existing_match.job.title = colorized_match.clone();
                                existing_match.matched_title = true;
                            }
                        }
                    }

                    utils::display::display_queried_jobs(matched_jobs, &mut table);
                } else {
                    utils::display::display_all_jobs(all_jobs, &mapped_stints, &mut table);
                }
            }
            Subcommands::Open { id } => {
                let all_jobs = sqlite::queries::get_all_jobs(&connection)?;
                let mapped_jobs = all_jobs
                    .into_iter()
                    .fold(HashMap::new(), |mut hashmap, job| {
                        hashmap.insert(job.id, job);
                        hashmap
                    });

                if let Some(job) = mapped_jobs.get(&Some(id)) {
                    if let Some(ref link) = job.link {
                        if !link.is_empty() {
                            println!(
                                "{}",
                                Color::Green
                                    .bold()
                                    .paint("🖥️ Opening the job listing in your default browser...")
                            );

                            open::that(link)?;
                        } else {
                            println!(
                                "{}",
                                Color::Fixed(172)
                                    .bold()
                                    .paint("⚠️  The selected job has an empty link!")
                            );
                        }
                    } else {
                        println!(
                            "{}",
                            Color::Fixed(172)
                                .bold()
                                .paint("⚠️  The selected job does not have an associated link!")
                        );
                    }
                } else {
                    println!(
                        "{}",
                        Color::Red.bold().paint("⚠️  Did not find a matching job!")
                    );
                }
            }
            Subcommands::Update {
                query,
                links,
                notes,
                stint,
                titles,
            } => {}
        }
    }

    Ok(())
}
