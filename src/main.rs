//! `fetters` - a CLI tool for tracking your job applications.

mod cli;
mod commands;
mod config;
mod errors;
mod models;
mod repositories;
mod schema;
mod sqlite;
mod utils;

use clap::Parser;
use lazy_static::lazy_static;
use owo_colors::OwoColorize;

use crate::cli::{Cli, Command};
use crate::commands::add::add_job;
use crate::commands::list::list_jobs;
use crate::commands::update::update_job;
use crate::config::configuration::Config;
use crate::errors::FettersError;
use crate::repositories::{sprint::SprintRepository, statuses::StatusRepository};
use crate::sqlite::Database;

lazy_static! {
    /// ASCII art for `fetters`.
    static ref ASCII_ART: &'static [u8; 695] = include_bytes!("../art.txt");
}

/// Run `fetters`.
fn main() -> Result<(), FettersError> {
    let config = Config::load_or_create()?;
    let mut database = Database::new_connection(&config.db_path)?;

    // Ensure the default statuses are stored in the `statuses` SQLite table.
    let mut status_repo = StatusRepository {
        connection: &mut database.connection,
    };
    status_repo.seed_statuses()?;

    let mut sprint_repo = SprintRepository {
        connection: &mut database.connection,
    };
    let current_sprint = sprint_repo.get_current_sprint(&config.sprint)?;

    let cli = Cli::parse();

    match cli.command {
        Command::Add { company } => {
            if let Err(error) = add_job(&mut database.connection, &company, &current_sprint) {
                println!("{}", error.red().bold());
            }
        }
        Command::Config { show } => (),
        Command::Delete { company } => (),
        Command::List {
            company,
            link,
            notes,
            sprint,
            status,
            title,
        } => {
            if let Err(error) = list_jobs(
                &mut database.connection,
                &company,
                &link,
                &notes,
                &sprint,
                &status,
                &title,
                &current_sprint,
            ) {
                println!("{}", error.red().bold());
            }
        }
        Command::Open { job_id } => (),
        Command::Sprint {
            current,
            new,
            show_all,
            set,
        } => (),
        Command::Update {
            company,
            link,
            notes,
            sprint,
            status,
            title,
        } => {
            if let Err(error) = update_job(
                &mut database.connection,
                &company,
                &link,
                &notes,
                &sprint,
                &status,
                &title,
                &current_sprint,
            ) {
                println!("{}", error.red().bold());
            }
        }
    }

    Ok(())
}
