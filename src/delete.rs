use crate::mcsv::overwrite;
use crate::model::Job;

use ansi_term::*;
use prettytable::*;

use std::collections::BTreeMap;
use std::io;
use std::process;

/// Update the keys within the BTreeMap after deleting a job.
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

/// Print the selected job for deletion.
fn print_selection(job_index: u16, master: &mut BTreeMap<u16, Job>) {
    println!("\n{}", Colour::Cyan.bold().paint("SELECTED JOB"));
    let mut to_delete = Table::new();

    to_delete.add_row(
        row![
            bFl => 
            "DATE ADDED", 
            "COMPANY", 
            "JOB TITLE", 
            "STATUS", 
            "NOTES"
        ]
    );

    to_delete.add_row(row![
        master.get(&job_index).unwrap().date.to_string(),
        master.get(&job_index).unwrap().company.to_string(),
        master.get(&job_index).unwrap().title.to_string(),
        master.get(&job_index).unwrap().status.to_string(),
        master.get(&job_index).unwrap().notes.to_string(),
    ]);

    to_delete.printstd();
}

/// Delete the selected job from the master BTreeMap. Then rewrite the spreadsheet.
pub fn delete_job(job_index: u16, master: &mut BTreeMap<u16, Job>) {
    print_selection(job_index, master);

    loop {
        let mut confirm_delete = String::new();

        println!("\n{}", Style::new().bold().paint("Confirm deletion? [Y/N]"));

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
