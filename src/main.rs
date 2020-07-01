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

use ansi_term::*;

fn main() {
    titles::main_title();

    let flags = cli::get_args();

    if let Some(company) = flags.add {
        let new_job = add::add_job(company);
        add::confirm_add(new_job);

    } else if let Some(company) = flags.update {        
        let mut master = mcsv::get_jobs().unwrap();

        let match_indexes = search::print_matches(&company, &master);
        let job_index = search::select_match(match_indexes);

        
        
    } else if let Some(company) = flags.delete {
        let mut master = mcsv::get_jobs().unwrap();

        let match_indexes = search::print_matches(&company, &master);
        let job_index = search::select_match(match_indexes);

        delete::delete_job(job_index, &mut master);

    } else if flags.list == true {
        let master = mcsv::get_jobs().unwrap();
        list::list_jobs(master);

    } else if flags.insights == true {
        let master = mcsv::get_jobs().unwrap();
        let job_stats = insights::get_stats(master);
        insights::display_insights(job_stats);

    } else {
        println!("{}\n", Colour::Red.bold().paint("NO ARGUMENTS GIVEN.\n"));
    }
}
