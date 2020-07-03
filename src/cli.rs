//! Defining command-line interface flags.

use structopt::StructOpt;

// extern crate assert_cmd;

/// This struct contains all flags that are used in this program.
#[derive(Debug, PartialEq, StructOpt)]
#[structopt(
    name = "The Struggle", 
    about = "A command line tool for tracking your job applications"
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

#[cfg(test)]
mod test_cli {
    use super::*;

    use assert_cmd::Command;

    #[test]
    fn test_invalid_arg() {
        Command::cargo_bin("ts")
            .unwrap()
            .arg("-q")
            .assert()
            .failure();
    }

    #[test]
    fn test_get_args() {
        let args = get_args();
        assert_eq!(Args {
            add: None,
            update: None,
            delete: None,
            list: false,
            insights: false
        }, args);
    }
}
