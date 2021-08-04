//! Deleting a job application from the spreadsheet.

use crate::display::display_prompt;
use crate::mcsv::overwrite;
use crate::model::Job;

use ansi_term::*;

use std::collections::BTreeMap;
use std::io;
use std::process;

/// Update the keys within the BTreeMap after deleting a job listing.
fn update_keys(master: &mut BTreeMap<u16, Job>) -> BTreeMap<u16, Job> {
    let mut update: BTreeMap<u16, Job> = BTreeMap::new();

    for (index, job) in master.values_mut().enumerate() {
        let index = index as u16;
        
        update.insert(index, Job::new_job(
            job.date.clone(),
            job.company.clone(),
            job.title.clone(),
            job.status.clone(),
            job.notes.clone()
        ));
    }

    update
}

/// Confirm deletion of the selected job listing from the master BTreeMap. Then 
/// rewrite the spreadsheet.
pub fn delete_job(job_index: u16, master: &mut BTreeMap<u16, Job>) {
    loop {
        let mut confirm_delete = String::new();

        display_prompt(format!("\n{}", Style::new().bold().paint("Confirm deletion? [Y/N] ")));

        match io::stdin().read_line(&mut confirm_delete) {
            Ok(_) => { 
                match confirm_delete.trim().to_uppercase().as_str() {
                    "Y" => {
                        master.remove(&job_index);
                        let mut update = update_keys(master);
                        overwrite(&mut update).expect("Failed to overwrite spreadsheet.");
                        
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
