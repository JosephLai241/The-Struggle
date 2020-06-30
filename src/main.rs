mod cli;
mod list;
mod mcsv;
mod model;
mod add;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let flags = cli::get_args();

    if let Some(company) = flags.add {
        let new_job = add::add_job(company);
        add::confirm_new_job(new_job)?;

    } else if let Some(company) = flags.update {
        println!("UPDATE {}", company);
        
    } else if let Some(company) = flags.delete {
        println!("DELETE {}", company);

    } else if flags.list == true {
        let master = mcsv::get_jobs()?;
        list::list_jobs(master);

    } else if flags.insights == true {
        println!("DISPLAY INSIGHTS");

    } else {
        println!("NO ARGUMENTS GIVEN.");
    }

    Ok(())
}
