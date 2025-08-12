//! Contains all CLI options.

use clap::{Parser, Subcommand};

/// Contains all CLI options for `fetters`.
#[derive(Debug, Parser)]
#[command(name = "fetters")]
#[command(about, version)]
pub struct Cli {
    /// Run a subcommand.
    #[command(subcommand)]
    pub command: Command,

    #[arg(long, help = "Show the ASCII art.")]
    pub art: bool,
}

/// Contains all subcommands for `fetters`.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Track a new job application.
    Add {
        /// The name of the company.
        company: String,
    },
    /// Configure `fetters` by opening its config file.
    Config {
        #[arg(short, long, help = "Display the current configuration settings.")]
        show: bool,
    },
    /// Delete a tracked job application.
    Delete(QueryArgs),
    /// List job applications.
    List(QueryArgs),
    /// Open a link associated with a job (based on the record's 'ID') in your browser.
    Open(QueryArgs),
    /// Configuration options for job sprints.
    Sprint {
        #[arg(short, long, help = "Display the current sprint name.")]
        current: bool,
        #[arg(short, long, help = "Create a new job sprint.")]
        new: bool,
        #[arg(long, help = "Show all job sprints tracked by fetters.")]
        show_all: bool,
        #[arg(long, help = "Set the current job sprint.")]
        set: Option<String>,
    },
    /// Update a tracked job application.
    Update(QueryArgs),
}

/// All flags you can use to query jobs.
#[derive(Debug, Parser)]
pub struct QueryArgs {
    #[arg(
        short,
        long,
        help = "Filter results by company name. Supports searching with partial text."
    )]
    pub company: Option<String>,
    #[arg(
        short,
        long,
        help = "Filter results by links. Supports searching with partial text."
    )]
    pub link: Option<String>,
    #[arg(
        short,
        long,
        help = "Filter results by notes. Supports searching with partial text."
    )]
    pub notes: Option<String>,
    #[arg(
        long,
        help = "Filter results by sprint name. Supports searching with partial text."
    )]
    pub sprint: Option<String>,
    #[arg(
        short,
        long,
        help = "Filter results by application status. Supports searching with partial text."
    )]
    pub status: Option<String>,
    #[arg(
        short,
        long,
        help = "Filter results by job title. Supports searching with partial text."
    )]
    pub title: Option<String>,
}
