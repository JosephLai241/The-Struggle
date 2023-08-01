//! Contains utility functions for displaying shit in the terminal.

use std::collections::{BTreeMap, HashMap};

use ansi_term::{Color, Style};
use regex::{Match, Regex};
use term_table::{row::Row, table_cell::TableCell, Table, TableStyle};

use crate::models::{config::FettersSettings, job::Job, search::SearchResult, stint::Stint};

/// Colorize the matched portions of a given string of text. Returns the original string if no
/// matches are found.
pub fn colorize_matching_substrings(original_string: String, regex: &Regex) -> String {
    let mut cloned_string = original_string.clone();
    let matches: Vec<Match<'_>> = regex.find_iter(&original_string).collect();

    for matched in matches {
        let matched_substring =
            &original_string.clone()[matched.range().start..matched.range().end];
        let colorized_substring =
            format!("{}", Color::Red.bold().paint(matched_substring.to_string()));

        cloned_string = cloned_string.replace(matched_substring, &colorized_substring);
    }

    cloned_string
}

/// Instantiate a new `Table` for listing jobs.
pub fn instantiate_table(fetters_settings: &FettersSettings) -> Table {
    let mut table = Table::new();
    table.max_column_width = fetters_settings.display.max_column_width;

    table.style = TableStyle::rounded();

    table.add_row(Row::new(vec![
        TableCell::new(Style::new().bold().paint("ID")),
        TableCell::new(Style::new().bold().paint("Date Added")),
        TableCell::new(Style::new().bold().paint("Company")),
        TableCell::new(Style::new().bold().paint("Title")),
        TableCell::new(Style::new().bold().paint("Status")),
        TableCell::new(Style::new().bold().paint("Notes")),
        TableCell::new(Style::new().bold().paint("Has Link?")),
        TableCell::new(Style::new().bold().paint("Stint")),
    ]));

    table
}

/// Display all jobs stored in SQLite.
pub fn display_all_jobs(
    all_jobs: Vec<Job>,
    mapped_stints: &HashMap<Option<i32>, Stint>,
    table: &mut Table,
) {
    for job in all_jobs {
        let stint_name = mapped_stints
            .get(&job.stint)
            .and_then(|stint| Some(stint.stint.clone()));

        table.add_row(Row::new(vec![
            TableCell::new(job.id.clone().unwrap_or(666)),
            TableCell::new(job.date_added.clone()),
            TableCell::new(job.company.clone()),
            TableCell::new(job.title.clone()),
            TableCell::new(job.status.clone()),
            TableCell::new(job.notes.clone().unwrap_or("N/A".to_string())),
            TableCell::new(job.link.is_some()),
            TableCell::new(stint_name.unwrap_or("N/A".to_string())),
        ]));
    }

    println!("{}", table.render());
}

/// Display all jobs that were queried by the user.
pub fn display_queried_jobs(matched_jobs: BTreeMap<Option<i32>, SearchResult>, table: &mut Table) {
    for search_result in matched_jobs.values() {
        table.add_row(Row::new(vec![
            TableCell::new(search_result.job.id.clone().unwrap_or(666)),
            TableCell::new(search_result.job.date_added.clone()),
            TableCell::new(search_result.job.company.clone()),
            TableCell::new(search_result.job.title.clone()),
            TableCell::new(search_result.job.status.clone()),
            TableCell::new(search_result.job.notes.clone().unwrap_or("N/A".to_string())),
            TableCell::new(search_result.job.link.is_some()),
            TableCell::new(search_result.painted_stint_name.clone()),
        ]));
    }

    println!("{}", table.render());
}
