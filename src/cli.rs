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
    /// Use ChatGPT to generate a resume based on your command-line history (Bash, Zsh, and Fish
    /// history) (requires API key).
    ///
    /// Consolidates the history written at the following locations depending on your operating
    /// system.
    ///
    /// These are the history locations fetters will target if you are using Linux or Windows
    /// (Windows Subsystem for Linux):
    /// `/home/<USERNAME>/bash_history`,
    /// `/home/<USERNAME>/.zsh_history`,
    /// `/home/<USERNAME>/.local/share/fish/fish_history`
    ///
    /// These are the history locations fetters will target if you are using MacOS:
    /// `/users/<USERNAME>/.bash_history`,
    /// `/users/<USERNAME>/.zsh_history`,
    /// `/users/<USERNAME>/.local/share/fish/fish_history`
    AutoResume {
        /// Save the generated resume to a directory path.
        #[arg(short, long)]
        save_to: Option<String>,
    },
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
        /// The query (regex) string for the particular job application. This query searches for
        /// matching strings in the job application company, title, notes, and stint.
        query: String,
    },
    /// Display job application insights in a piechart. Set a ChatGPT API key to receive a more
    /// in-depth summary of your job applications.
    ///
    /// The piechart is comprised of two characters: 'o' and '?'. 'o' is used for application
    /// status that have a known/mapped style in the configuration. '?' is used for application
    /// status that do not have a known/mapped style. The colors used for slices that use the '?'
    /// character are randomly selected and are different each time you render the insights.
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
        /// The query (regex) string for the particular job application. This query searches for
        /// matching strings in the job application company, title, notes, and stint.
        query: Option<String>,
    },
    /// Open a job application in the browser if a link was also provided.
    Open {
        /// The job ID.
        id: i32,
    },
    /// Update an existing job application.
    Update {
        /// The query (regex) string for the particular job application. This query searches for
        /// matching strings in the job application company, title, notes, and stint.
        query: String,
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
