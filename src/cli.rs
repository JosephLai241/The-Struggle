//! Defining command-line interface flags.

use structopt::StructOpt;

/// This struct contains all flags that are used in this program.
#[derive(StructOpt)]
#[structopt(
    name = "The Struggle", 
    about = "A Rust command line tool for tracking your job applications"
)]
pub struct Args {
    /// Flag for adding a job to the spreadsheet.
    #[structopt(
        short = "a", 
        long = "add", 
        help = "Add a new job to the spreadsheet"
    )]
    pub add: Option<String>,

    /// Flag for updating an existing job.
    #[structopt(
        short = "u", 
        long = "update", 
        help = "Update an existing job in the spreadsheet"
    )]
    pub update: Option<String>,

    /// Flag for deleting an existing job.
    #[structopt(
        short = "d", 
        long = "delete", 
        help = "Delete an existing job in the spreadsheet"
    )]
    pub delete: Option<String>,

    /// Flag for listing all existing jobs.
    #[structopt(
        short = "l", 
        long = "list", 
        help = "List all saved job applications",
    )]
    pub list: bool,

    /// Flag for displaying job application insights.
    #[structopt(
        short = "i", 
        long = "insights", 
        help = "Display job application insights",
    )]
    pub insights: bool,
}

/// Return Args struct.
pub fn get_args() -> Args {
    return Args::from_args();
}
