use crate::add::get_status;
use crate::mcsv::overwrite;
use crate::model::Job;

use ansi_term::*;
use prettytable::*;

use std::collections::BTreeMap;
use std::io;
use std::process;

/// Select a job attribute to update.
pub fn select_attribute() -> u16 {
    let update_prompt = r#"
        UPDATE SECTION
-----------------------------
    0: COMPANY NAME
    1: JOB TITLE
    2: APPLICATION STATUS
    3: NOTES
-----------------------------"#;

    loop {
        let mut section = String::new();

        println!("{}", Style::new().bold().paint(update_prompt));

        match io::stdin().read_line(&mut section) {
            Ok(_) => {
                match section.trim().parse::<u16>() {
                    Ok(section_int) => {
                        if (0..4).contains(&section_int) {
                            return section_int;
                        } else {
                            println!("\n{}",
                                Colour::Red.bold().paint("Please select a valid section.")
                            );
                        }
                    },
                    Err(_) => {
                        println!("\n{}",
                            Colour::Red.bold().paint("Please select a valid section.")
                        );
                    }
                }
            },
            Err(e) => { println!("Error! {:?}", e); }
        }
    }
}

/// Input a new value for the selected job attribute.
pub fn update_attribute(section_int: u16) -> (u16, String) {
    loop {
        let mut update = String::new();

        let mut update_index = 0;

        match section_int {
            0 => println!("\nWhat is the new company name?"),
            1 => {
                println!("\nWhat is the new job title?");
                update_index = 1;
            },
            2 => {
                let new_status = get_status();
                return (2, new_status);
            },
            3 => {
                println!("\nWhat are the new notes?");
                update_index = 3;
            },
            _ => ()
        }

        match io::stdin().read_line(&mut update) {
            Ok(_) => { return (update_index, update.trim().to_string()); },
            Err(e) => { println!("Error! {:?}", e); }
        }
    }
}

/// Update the job attribute in the master BTreeMap that contains all existing
/// job listings. Then rewrite the spreadsheet.
pub fn update_job(job_index: u16, master: &mut BTreeMap<u16, Job>, update: (u16, String)) {
    println!("\n{}", Colour::Cyan.bold().paint("UPDATED JOB"));
    let mut to_update = Table::new();

    to_update.add_row(
        row![
            bFl => 
            "DATE ADDED", 
            "COMPANY", 
            "JOB TITLE", 
            "STATUS", 
            "NOTES"
        ]
    );

    if let Some(job) = master.get_mut(&job_index) {
        match update.0 {
            0 => { job.company = update.1 },
            1 => { job.title = update.1 },
            2 => { job.status = update.1 },
            3 => { job.notes = update.1 },
            _ => ()
        }
    }

    to_update.add_row(row![
        master.get(&job_index).unwrap().date.to_string(),
        master.get(&job_index).unwrap().company.to_string(),
        master.get(&job_index).unwrap().title.to_string(),
        master.get(&job_index).unwrap().status.to_string(),
        master.get(&job_index).unwrap().notes.to_string(),
    ]);

    to_update.printstd();

    loop {
        let mut confirm_delete = String::new();

        println!("\n{}", Style::new().bold().paint("Confirm update? [Y/N]"));

        match io::stdin().read_line(&mut confirm_delete) {
            Ok(_) => { 
                match confirm_delete.trim().to_uppercase().as_str() {
                    "Y" => {
                        overwrite(master).expect("Failed to overwrite spreadsheet.");
                        break;
                    },
                    "N" => {
                        println!("\n{}\n", Colour::Red.bold().paint("CANCELLING."));
                        process::exit(1);
                    },
                    _ => println!("\n{}", Colour::Red.bold().paint("Not an option!"))
                }
            },
            Err(e) => { println!("Error! {:?}", e); }
        }
    }
}
