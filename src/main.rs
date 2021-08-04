mod add;
mod cli;
mod delete;
mod display;
mod format;
mod insights;
mod list;
mod mcsv;
mod model;
mod search;
mod titles;
mod update;

use ansi_term::*;

/// It really is a struggle out there.
fn main() {
    titles::main_title();

    match cli::get_args() {
        cli::Args { add: Some(company), .. } => {
            let new_job = add::add_job(company);
            search::print_selection(
                search::DataType::Singular(&new_job), 
                "NEW JOB".to_string());
            
            add::confirm_add(new_job);
        },
        cli::Args { update: Some(company), .. } => {
            let mut master = mcsv::get_jobs().unwrap();

            let match_indexes = search::print_matches(&company, &master);
            let job_index = search::select_match(match_indexes);

            let section_int = update::select_attribute();
            let update = update::get_update(section_int);
            update::change_attribute(job_index, &mut master, update);
            search::print_selection(
                search::DataType::Btm(job_index, &master), 
                "UPDATED JOB".to_string());

            update::update_job(&mut master);
        },
        cli::Args { delete: Some(company), .. } => {
            let mut master = mcsv::get_jobs().unwrap();

            let match_indexes = search::print_matches(&company, &master);
            let job_index = search::select_match(match_indexes);
            search::print_selection(
                search::DataType::Btm(job_index, &master), 
                "SELECTED JOB".to_string());

            delete::delete_job(job_index, &mut master);
        },
        cli::Args { search: Some(company), .. } => {
            let master = mcsv::get_jobs().unwrap();
            search::print_matches(&company, &master);
        },
        cli::Args { list: true, .. } => {
            let master = mcsv::get_jobs().unwrap();
            list::list_jobs(master);
        },
        cli::Args { insights: true, .. } => {
            let master = mcsv::get_jobs().unwrap();
            let job_stats = insights::get_stats(master);
            insights::display_insights(job_stats);
        },
        _ => println!("{}\n", Colour::Red.bold().paint("NO ARGUMENTS GIVEN 👎"))
    }
}
