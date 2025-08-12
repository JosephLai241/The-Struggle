//! Contains utilities for displaying job applications.

use owo_colors::OwoColorize;
use tabled::{
    Table,
    settings::{Color, Style, location::Locator, object::Rows, style::LineText},
};

use crate::models::TabledJob;

/// Display jobs in a table.
pub fn display_jobs(jobs: Vec<TabledJob>, sprint_name: &str) {
    // TODO: SHOW THE SPRINT AT THE TOP OF THE TABLE INSTEAD OF THE SEPARATE COLUMN.
    let mut table = Table::new(jobs);

    table
        .with(LineText::new(format!("{sprint_name} sprint"), Rows::first()).offset(2))
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
