//! Contains all functionality pertaining to the `add` subcommand.

use ansi_term::Color;
use rusqlite::Connection;

use crate::{
    errors::FettersError,
    models::{config::FettersSettings, job::Job},
    prompts, sqlite,
};

/// Add a new job to the SQLite instance.
pub fn add_job(
    company: String,
    connection: &Connection,
    fetters_settings: &FettersSettings,
    link: Option<String>,
    notes: Option<String>,
    skip: bool,
    status: Option<String>,
    stint: Option<String>,
    title: Option<String>,
) -> Result<(), FettersError> {
    let title = match title {
        Some(title) => title,
        None => prompts::add::add_job_title(fetters_settings)?,
    };
    let status = match status {
        Some(status) => status,
        None => prompts::add::add_job_status(fetters_settings)?,
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
        Some(stint_name) => sqlite::queries::get_all_stints(connection)?
            .iter()
            .find(|stint| stint.stint == stint_name)
            .map(|stint| stint.id.unwrap_or(0)),
        None => {
            if skip {
                None
            } else {
                prompts::add::add_job_stint(connection)?
            }
        }
    };

    let new_job = Job::new(company, link, notes, status, stint, title);

    sqlite::queries::write_job(connection, new_job)?;

    println!(
        "\n{}\n",
        Color::Green
            .bold()
            .paint("💯 Successfully added a new job!")
    );

    Ok(())
}
