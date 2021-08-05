//! Updating a job application in the spreadsheet.

use crate::add::get_status;
use crate::mcsv::overwrite;
use crate::model::Job;
use crate::prompt::display_prompt;

use ansi_term::*;

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
        println!("{}", Style::new().bold().paint(update_prompt));
        
        let mut section = String::new();

        match io::stdin().read_line(&mut section) {
            Ok(_) => {
                match section.trim().parse::<u16>() {
                    Ok(section_int) => {
                        if (0..4).contains(&section_int) {
                            return section_int;
                        } else {
                            println!("\n{}", Colour::Red.bold().paint("Please select a valid section."));
                        }
                    },
                    Err(_) => {
                        println!("\n{}", Colour::Red.bold().paint("Please select a valid section."));
                    }
                }
            },
            Err(e) => { 
                println!("Error! {:?}", e); 
            }
        }
    }
}

/// Input a new value for the selected job attribute.
pub fn get_update(section_int: u16) -> (u16, String) {
    loop {
        let mut update = String::new();
        let mut update_index = 0;

        match section_int {
            0 => display_prompt(format!("{}", Colour::Green.bold().paint("\nEnter the new company name: "))),
            1 => {
                display_prompt(format!("{}", Colour::Green.bold().paint("\nEnter the new job title: ")));
                update_index = 1;
            },
            2 => {
                let new_status = get_status();
                return (2, new_status);
            },
            3 => {
                display_prompt(format!("{}", Colour::Green.bold().paint("\nEnter the new note: ")));
                update_index = 3;
            },
            _ => ()
        }

        match io::stdin().read_line(&mut update) {
            Ok(_) => {
                return (update_index, update.trim().to_string());
            },
            Err(e) => {
                println!("Error! {:?}", e);
            }
        }
    }
}

/// Change the attribute of the selected job listing.
pub fn change_attribute(
    job_index: u16, 
    master: &mut BTreeMap<u16, Job>, 
    update: (u16, String)
) {
        if let Some(job) = master.get_mut(&job_index) {
            match update.0 {
                0 => {
                    job.company = update.1
                },
                1 => {
                    job.title = update.1
                },
                2 => {
                    job.status = update.1
                },
                3 => {
                    job.notes = update.1
                },
                _ => ()
            }
        }
}

/// Update the job attribute in the master BTreeMap that contains all existing
/// job listings. Then rewrite the spreadsheet.
pub fn update_job(master: &mut BTreeMap<u16, Job>) {
    loop {
        display_prompt(format!("\n{}", Style::new().bold().paint("Confirm update? [Y/N] ")));
        
        let mut confirm_delete = String::new();

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
