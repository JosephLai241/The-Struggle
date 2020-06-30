// use clap::arg_enum;
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

////////////////////////////////////////////////////////////////////////////////

    // /// Flag for listing all existing jobs. Rust's PrettyTable does not have a sort
    // /// option yet, so this functionality is currently not available.
    // #[structopt(
    //     short = "l", 
    //     long = "list", 
    //     help = "List all saved job applications",
    //     possible_values = &Sort::variants(),
    //     case_insensitive = true
    // )]
    // pub list: Option<String>,

    // /// Flag for displaying job application insights. Will consider adding this
    // /// option back if it is requested.
    // #[structopt(
    //     short = "i", 
    //     long = "insights", 
    //     help = "Display job application insights",
    //     possible_values = &Display::variants(),
    //     case_insensitive = true
    // )]
    // pub insights: Option<String>,
}

// arg_enum! {
//     /// Enum for list sorting options. Rust's PrettyTable does not have a sort
//     /// option yet, so this functionality is currently not available.
//     enum Sort {
//         Date,
//         Reverse,
//         Company,
//         Title,
//         Status,
//         Notes
//     }
// }

// arg_enum! {
//     /// Enum for insight display options.
//     enum Display {
//         All,
//         Pending,
//         InProgress,
//         Offers,
//         Hired,
//         Rejected
//     }
// }

/// Return Args struct.
pub fn get_args() -> Args {
    return Args::from_args();
}
