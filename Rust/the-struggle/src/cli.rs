use clap::arg_enum;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "The Struggle", 
    about = "A Rust command line tool for tracking your job applications"
)]
pub struct Args {
    // Add a job
    #[structopt(
        short = "a", 
        long = "add", 
        help = "Add a new job to the spreadsheet"
    )]
    pub add: Option<String>,

    // Update an existing job
    #[structopt(
        short = "u", 
        long = "update", 
        help = "Update an existing job in the spreadsheet"
    )]
    pub update: Option<String>,

    // Delete an existing job
    #[structopt(
        short = "d", 
        long = "delete", 
        help = "Delete an existing job in the spreadsheet"
    )]
    pub delete: Option<String>,

    // List all existing jobs
    #[structopt(
        short = "l", 
        long = "list", 
        help = "List all saved job applications",
        possible_values = &Sort::variants(),
        case_insensitive = true
    )]
    pub list: Option<String>,

    // Display job application insights
    #[structopt(
        short = "i", 
        long = "insights", 
        help = "Display job application insights",
        possible_values = &Display::variants(),
        case_insensitive = true
    )]
    pub insights: Option<String>,

}

arg_enum! {
    // List sort options
    #[derive(Debug)]
    enum Sort {
        Date,
        Reversed,
        Company,
        Title,
        Status,
        Notes
    }
}

arg_enum! {
    // Insights display options
    #[derive(Debug)]
    enum Display {
        All,
        Pending,
        InProgress,
        Offers,
        Hired,
        Rejected
    }
}

pub fn get_args() -> Args {
    // Return Args struct.
    return Args::from_args();
}
