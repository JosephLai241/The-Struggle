mod cli;
mod mcsv;
mod model;
mod add;

fn main() {
    let flags = cli::get_args();

    if let Some(company) = flags.add {
        let new_job = add::add_job(company);
        add::confirm_new_job(new_job);
    } else if let Some(company) = flags.update {
        println!("UPDATE {}", company);

    } else if let Some(company) = flags.delete {
        println!("DELETE {}", company);

    } else if let Some(sort) = flags.list {
        println!("SORT LIST BY {}", sort);

    } else if let Some(display) = flags.insights {
        println!("DISPLAY INSIGHTS BY {}", display);

    }
}
