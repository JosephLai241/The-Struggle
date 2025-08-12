//! Contains utilities for displaying job applications.

use owo_colors::OwoColorize;
use tabled::{
    Table, Tabled,
    settings::{
        Alignment, Color, Remove, Style,
        location::Locator,
        object::{Columns, Rows},
    },
};

use crate::models::TabledJob;

/// Display jobs in a table.
pub fn display_jobs(jobs: &Vec<TabledJob>, sprint_name: &str) {
    let mut table = Table::new(jobs);

    table
        .with(Style::rounded())
        .modify(Rows::first(), Color::FG_BRIGHT_WHITE)
        .modify(Locator::content("GHOSTED"), Color::rgb_fg(133, 133, 133))
        .modify(Locator::content("HIRED"), Color::FG_BRIGHT_GREEN)
        .modify(Locator::content("IN PROGRESS"), Color::FG_BRIGHT_YELLOW)
        .modify(
            Locator::content("NOT HIRING ANYMORE"),
            Color::rgb_fg(117, 117, 117),
        )
        .modify(Locator::content("OFFER RECEIVED"), Color::FG_BRIGHT_MAGENTA)
        .modify(Locator::content("PENDING"), Color::FG_BRIGHT_BLUE)
        .modify(Locator::content("REJECTED"), Color::FG_BRIGHT_RED);

    println!(
        "\n{}\n",
        format!("{sprint_name} SPRINT").green().bold().underline()
    );
    println!("{table}");
}

/// Display a single job. This generic function works with any struct that implements `Tabled`.
pub fn display_single_job<T: Tabled>(job: T) {
    let mut table = Table::new([job]);
    table
        .with(Style::rounded())
        .with(Remove::column(Columns::first()))
        .modify(Columns::first(), Alignment::left())
        .modify(Locator::content("GHOSTED"), Color::FG_BRIGHT_WHITE)
        .modify(Locator::content("HIRED"), Color::FG_BRIGHT_GREEN)
        .modify(Locator::content("IN PROGRESS"), Color::FG_BRIGHT_YELLOW)
        .modify(
            Locator::content("NOT HIRING ANYMORE"),
            Color::rgb_fg(201, 201, 201),
        )
        .modify(Locator::content("OFFER RECEIVED"), Color::FG_BRIGHT_MAGENTA)
        .modify(Locator::content("PENDING"), Color::FG_BRIGHT_BLUE)
        .modify(Locator::content("REJECTED"), Color::FG_BRIGHT_RED);

    println!("{table}");
}
