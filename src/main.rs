mod add;
mod cli;
mod list;
mod mcsv;
mod model;
mod titles;

mod search;

use ansi_term::*;

fn main() {
    titles::main_title();

    let flags = cli::get_args();

    if let Some(company) = flags.add {
        let new_job = add::add_job(company);
        add::confirm_add(new_job);

    } else if let Some(company) = flags.update {
        println!("UPDATE {}", company);
        
        let master = mcsv::get_jobs().unwrap();
        
    } else if let Some(company) = flags.delete {
        let master = mcsv::get_jobs().unwrap();
        let match_indexes = search::print_matches(&company, master);
        let index = search::select_match(match_indexes);

        println!("{:?}", index);
    } else if flags.list == true {
        let master = mcsv::get_jobs().unwrap();
        list::list_jobs(master);
    } else if flags.insights == true {
        println!("DISPLAY INSIGHTS");

        let master = mcsv::get_jobs().unwrap();

    } else {
        println!("{}\n", Colour::Red.bold().paint("NO ARGUMENTS GIVEN.\n"));
    }
}
