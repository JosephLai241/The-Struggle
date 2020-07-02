mod add;
mod cli;
mod delete;
mod format;
mod insights;
mod list;
mod mcsv;
mod model;
mod search;
mod titles;
mod update;

use ansi_term::*;

fn main() {
    titles::main_title();

    let flags = cli::get_args();

    // Add a new job to the spreadsheet.
    if let Some(company) = flags.add {
        let new_job = add::add_job(company);
        add::confirm_add(new_job);

    // Update a job in the spreadsheet.
    } else if let Some(company) = flags.update {        
        let mut master = mcsv::get_jobs().unwrap();

        let match_indexes = search::print_matches(&company, &master);
        let job_index = search::select_match(match_indexes);

        let section_int = update::select_attribute();
        let update = update::update_attribute(section_int);

        update::update_job(job_index, &mut master, update);
        
    // Delete a job from the spreadsheet.
    } else if let Some(company) = flags.delete {
        let mut master = mcsv::get_jobs().unwrap();

        let match_indexes = search::print_matches(&company, &master);
        let job_index = search::select_match(match_indexes);

        delete::delete_job(job_index, &mut master);

    // List all stored jobs.
    } else if flags.list == true {
        let master = mcsv::get_jobs().unwrap();
        list::list_jobs(master);

    // Display insights for each application status.
    } else if flags.insights == true {
        let master = mcsv::get_jobs().unwrap();
        let job_stats = insights::get_stats(master);
        insights::display_insights(job_stats);

    // If no arguments were entered.
    } else {
        println!("{}\n", Colour::Red.bold().paint("NO ARGUMENTS GIVEN 👎"));
    }
}
