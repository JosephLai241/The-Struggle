mod cli;
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
        let master = mcsv::get_jobs()?;


    } else if let Some(company) = flags.delete {
        println!("DELETE {}", company);

    } else if let Some(sort) = flags.list {
        println!("SORT LIST BY {}", sort);

    } else if let Some(display) = flags.insights {
        println!("DISPLAY INSIGHTS BY {}", display);

    } else {
        println!("NO ARGUMENTS GIVEN.");
    }

    Ok(())
}
