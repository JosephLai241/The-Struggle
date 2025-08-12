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
    Delete { company: String },
    /// List job applications.
    List {
        #[arg(
            short,
            long,
            help = "Filter results by company name. Supports searching with partial text."
        )]
        company: Option<String>,
        #[arg(
            short,
            long,
            help = "Filter results by links. Supports searching with partial text."
        )]
        link: Option<String>,
        #[arg(
            short,
            long,
            help = "Filter results by notes. Supports searching with partial text."
        )]
        notes: Option<String>,
        #[arg(
            long,
            help = "Filter results by sprint name. Supports searching with partial text."
        )]
        sprint: Option<String>,
        #[arg(
            short,
            long,
            help = "Filter results by application status. Supports searching with partial text."
        )]
        status: Option<String>,
        #[arg(
            short,
            long,
            help = "Filter results by job title. Supports searching with partial text."
        )]
        title: Option<String>,
    },
    /// Open a link associated with a job (based on the record's 'ID') in your browser.
    Open {
        /// The ID of the record associated with the link.
        job_id: i32,
    },
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
    Update {
        #[arg(
            short,
            long,
            help = "Filter results by company name. Supports searching with partial text."
        )]
        company: Option<String>,
        #[arg(
            short,
            long,
            help = "Filter results by links. Supports searching with partial text."
        )]
        link: Option<String>,
        #[arg(
            short,
            long,
            help = "Filter results by notes. Supports searching with partial text."
        )]
        notes: Option<String>,
        #[arg(
            long,
            help = "Filter results by sprint name. Supports searching with partial text."
        )]
        sprint: Option<String>,
        #[arg(
            short,
            long,
            help = "Filter results by application status. Supports searching with partial text."
        )]
        status: Option<String>,
        #[arg(
            short,
            long,
            help = "Filter results by job title. Supports searching with partial text."
        )]
        title: Option<String>,
    },
}
