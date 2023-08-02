//! Contains functionality pertaining to the command-line interface.

use clap::{Parser, Subcommand};

use crate::utils::config;

/// The command-line interface for `fetters`.
#[derive(Debug, Parser)]
#[command(about, author, version)]
pub struct Args {
    /// Display the ASCII art for `fetters`.
    #[arg(long)]
    pub banner: bool,

    /// Contains all subcommands for `fetters`.
    #[command(subcommand)]
    pub subcommand: Option<Subcommands>,
}

/// Contains all subcommands that may be run with `fetters`.
#[derive(Debug, Subcommand)]
pub enum Subcommands {
    /// Add a new job.
    Add {
        /// The name of the company.
        company: String,
        /// The link to the job listing.
        #[arg(short, long)]
        link: Option<String>,
        /// Any notes pertaining to the job listing.
        #[arg(short, long)]
        notes: Option<String>,
        /// Skip interactive prompts for optional job attributes. This will skip the following
        /// attributes if they are not provided: link, notes, stint.
        #[arg(long)]
        skip: bool,
        /// The job application's status.
        #[arg(short, long, value_parser = check_status_options)]
        status: Option<String>,
        /// The stint associated with this job application.
        #[arg(long)]
        stint: Option<String>,
        /// The title of the role.
        #[arg(short, long)]
        title: Option<String>,
    },
    /// Delete an existing job application.
    Delete {
        /// The query string for the particular job application.
        query: String,
        /// Include searching for text in links.
        #[arg(long)]
        links: bool,
        /// Include searching for text in notes.
        #[arg(long)]
        notes: bool,
        /// Include searching for text in the stint.
        #[arg(long)]
        stint: bool,
        /// Include searching for text in job titles.
        #[arg(long)]
        titles: bool,
    },
    /// Display job application insights.
    Insights {
        /// Display insights for a given date range delimited by a comma.
        ///
        /// Sets the upper range to the current date if no upper date range is provided
        ///
        /// Accepts ISO8601 date formats delimited by a forward slash, ie. '2023/08/01'
        ///
        /// Example acceptable date ranges: '2023/06/01,2023/08/01', '2023/06/01'
        #[arg(short, long)]
        date_range: Option<String>,
        /// Display insights for a given stint.
        #[arg(long)]
        stint: Option<String>,
    },
    /// List all or search for a particular job application.
    List {
        /// The query (regex) string for the particular job application. If this is provided
        /// without any other flags, all flags are enabled and the query will be applied to all
        /// fields.
        query: Option<String>,
    },
    /// Open a job application in the browser if a link was also provided.
    Open {
        /// The job ID.
        id: i32,
    },
    /// Update an existing job application.
    Update {
        /// The query string for the particular job application.
        query: String,
        /// Include searching for text in links.
        #[arg(long)]
        links: bool,
        /// Include searching for text in notes.
        #[arg(long)]
        notes: bool,
        /// Include searching for text in the stint.
        #[arg(long)]
        stint: bool,
        /// Include searching for text in job titles.
        #[arg(long)]
        titles: bool,
    },
}

/// Check the provided status option against the status options defined in the configuration file.
fn check_status_options(status: &str) -> Result<String, String> {
    match config::configure_fetters() {
        Ok(fetters_settings) => {
            let lowercase_status_options = fetters_settings
                .presets
                .status_mappings
                .clone()
                .into_keys()
                .map(|status| status.to_lowercase())
                .collect::<Vec<String>>();

            if lowercase_status_options.contains(&status.to_lowercase()) {
                Ok(status.to_string())
            } else {
                Err(format!(
                    "Invalid status! Add a new status with a new job listing or choose from one of the following: {:?}",
                    fetters_settings.presets.status_mappings.into_keys().collect::<Vec<String>>()
                ))
            }
        }
        Err(error) => Err(format!("Failed to load fetters configuration! {error}")),
    }
}
