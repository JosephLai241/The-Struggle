//! Contains all functionality pertaining to the `open` subcommand.

use std::collections::HashMap;

use ansi_term::Color;
use rusqlite::Connection;

use crate::{errors::FettersError, sqlite};

/// Open a link associated with the given job, if applicable.
pub fn open_job_link(connection: &Connection, id: i32) -> Result<(), FettersError> {
    let all_jobs = sqlite::queries::get_all_jobs(connection)?;
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

    Ok(())
}
