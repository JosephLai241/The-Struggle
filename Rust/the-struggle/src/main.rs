mod cli;
mod m_csv;
mod model;
mod add;

use std::string::String;

fn main() {
    let flags = cli::get_args();

    if let Some(company) = flags.add {
        let new_job = add::add_job(company);
        let details = add::print_job(new_job);
        // println!("{:?}", details);

    } else if let Some(company) = flags.update {
        println!("UPDATE {}", company);

    } else if let Some(company) = flags.delete {
        println!("DELETE {}", company);

    } else if let Some(sort) = flags.list {
        println!("SORT LIST BY {}", sort);

    } else if let Some(display) = flags.insights {
        println!("DISPLAY INSIGHTS BY {}", display);

    }

    // USE UNWRAP() TO GET MASTER BTREEMAP
    // let master = m_csv::get_jobs().unwrap();
    // println!("PRINTING FOR MASTER NOW\n");
    // println!("PRINTING MASTER LENGTH\n");
    // println!("{:?}", master.len() - 1);
    // for i in 0u16..master.len() as u16 {
    //     println!("{}", master.get(&i).unwrap().company);
    //     println!("{}", master.get(&i).unwrap().title);
    //     println!();
    // }
}
