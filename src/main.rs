//! `fetters` - A command-line tool for tracking your job applications.

// Disables error variant lint raised for `errors.rs`.
#![allow(clippy::enum_variant_names)]
// Disables too many arguments lint raised for the modules in the `subcommands/` directory.
#![allow(clippy::too_many_arguments)]

use ansi_term::Color;
use clap::Parser;
use cli::{Args, Subcommands};
use errors::FettersError;
use lazy_static::lazy_static;
use subcommands::{add, delete, insights, list, open, resume::generate_resume, update};

mod cli;
mod errors;
mod models;
mod prompts;
mod schema;
mod sqlite;
mod subcommands;
mod utils;

lazy_static! {
    /// The ASCII art for `fetters`.
    static ref ASCII_ART: &'static [u8; 1204] = include_bytes!("../art.txt");
}

/// Run `fetters`.
#[tokio::main]
async fn main() -> Result<(), FettersError> {
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
        let mut connection = sqlite::setup::open_sqlite()?;

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
                add::add_job(
                    company,
                    &mut connection,
                    &fetters_settings,
                    link,
                    notes,
                    skip,
                    status,
                    stint,
                    title,
                )?;
            }
            Subcommands::AutoResume { save_to } => {
                generate_resume(&fetters_settings, save_to).await?;
            }
            Subcommands::Delete { query } => {
                delete::delete_job(&mut connection, query)?;
            }
            Subcommands::Insights { date_range, stint } => {
                insights::display_insights(&mut connection, date_range, &fetters_settings, stint)
                    .await?;
            }
            Subcommands::List { query } => {
                list::list_jobs(&mut connection, &fetters_settings, query)?;
            }
            Subcommands::Open { id } => {
                open::open_job_link(&mut connection, id)?;
            }
            Subcommands::Update { query } => {
                update::update_job(&mut connection, query)?;
            }
        }
    }

    Ok(())
}
