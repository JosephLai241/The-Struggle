//! Contains utility functions for displaying shit in the terminal.

use std::collections::{BTreeMap, HashMap};

use ansi_term::Style;
use regex::{Match, Regex};
use term_table::{row::Row, table_cell::TableCell, Table, TableStyle};

use crate::models::{config::FettersSettings, job::Job, search::SearchResult, stint::Stint};

/// Colorize the matched portions of a given string of text. Returns the original string if no
/// matches are found. Also returns a `bool` indicating whether a match was found.
pub fn colorize_matching_substrings(
    highlight_style: Style,
    original_string: String,
    regex: &Regex,
    status_style: Style,
) -> (String, bool) {
    let mut cloned_string = original_string.clone();
    let matches: Vec<Match<'_>> = regex.find_iter(&original_string).collect();

    let mut has_matched = false;

    for matched in matches {
        if !has_matched {
            has_matched = true;
        }

        let matched_substring =
            &original_string.clone()[matched.range().start..matched.range().end];
        let colorized_substring =
            format!("{}", highlight_style.paint(matched_substring.to_string()));

        cloned_string = cloned_string.replace(matched_substring, &colorized_substring);
    }

    if !has_matched {
        cloned_string = format!("{}", status_style.paint(cloned_string));
    }
    (cloned_string, has_matched)
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
    status_mappings: &BTreeMap<String, Style>,
    table: &mut Table,
) {
    for job in all_jobs {
        let style = status_mappings
            .get(&job.status)
            .unwrap_or(&Style::default())
            .to_owned();

        let stint_name = mapped_stints
            .get(&job.stint)
            .map(|stint| stint.stint.clone());

        table.add_row(Row::new(vec![
            TableCell::new(style.paint(job.id.unwrap_or(666).to_string())),
            TableCell::new(style.paint(job.date_added.clone())),
            TableCell::new(style.paint(job.company.clone())),
            TableCell::new(style.paint(job.title.clone())),
            TableCell::new(style.paint(job.status.clone())),
            TableCell::new(style.paint(job.notes.clone().unwrap_or("".to_string()))),
            TableCell::new(style.paint(job.link.is_some().to_string())),
            TableCell::new(style.paint(stint_name.unwrap_or("".to_string()))),
        ]));
    }

    println!("{}", table.render());
}

/// Display all jobs that were queried by the user.
pub fn display_queried_jobs(
    matched_jobs: BTreeMap<Option<i32>, SearchResult>,
    status_mappings: &BTreeMap<String, Style>,
    table: &mut Table,
) {
    for search_result in matched_jobs.values() {
        let style = status_mappings
            .get(&search_result.job.status)
            .unwrap_or(&Style::default())
            .to_owned();

        table.add_row(Row::new(vec![
            TableCell::new(style.paint(search_result.job.id.unwrap_or(666).to_string())),
            TableCell::new(style.paint(search_result.job.date_added.clone())),
            TableCell::new(search_result.job.company.clone()),
            TableCell::new(search_result.job.title.clone()),
            TableCell::new(style.paint(search_result.job.status.clone())),
            TableCell::new(search_result.job.notes.clone().unwrap_or("".to_string())),
            TableCell::new(style.paint(search_result.job.link.is_some().to_string())),
            TableCell::new(search_result.painted_stint_name.clone()),
        ]));
    }

    println!("{}", table.render());
}
