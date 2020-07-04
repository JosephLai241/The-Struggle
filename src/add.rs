//! Adding a new job application to the spreadsheet.

use crate::mcsv;
use crate::model::Job;

use ansi_term::*;
use chrono::prelude::*;

use std::io;
use std::process;

/// Get the job title at the company.
fn get_title(company: &String) -> String {
    loop {
        println!("{}", Style::new()
            .bold()
            .paint(format!(
                "What is the title of the position you are applying for at {}?", 
                company
        )));

        let mut title = String::new();

        match io::stdin().read_line(&mut title) {
            Ok(_) => {
                let input = title.trim().to_string();

                if !input.is_empty() {
                    return title.trim().to_string(); 
                } else { 
                    println!("{}\n",
                        Colour::Red.bold().paint("Please enter a job title.")
                    );
                }
            },
            Err(e) => { println!("Error! {:?}", e); }
        }
    }
}

/// Get the job application status.
pub fn get_status() -> String {
    let status_options: Vec<String> = vec![
        "PENDING".to_string(), 
        "IN PROGRESS".to_string(), 
        "OFFER RECEIVED".to_string(), 
        "HIRED".to_string(), 
        "REJECTED".to_string()
    ];

    let status_prompt = r#"
    SELECT JOB STATUS
-------------------------
    0: PENDING
    1: IN PROGRESS
    2: OFFER RECEIVED
    3: HIRED
    4: REJECTED
-------------------------"#;

    loop {
        println!("{}", Style::new().bold().paint(status_prompt));
        
        let mut status = String::new();

        match io::stdin().read_line(&mut status) {
            Ok(_) => {
                match status.trim().parse::<usize>() {
                    Ok(status_int) => {
                        if (0..5).contains(&status_int) {
                            return status_options[status_int].to_string();
                        } else {
                            println!("\n{}",
                                Colour::Red.bold().paint("Please select a valid status option.")
                            );
                        }
                    },
                    Err(_) => {
                        println!("\n{}",
                            Colour::Red.bold().paint("Please select a valid status option.")
                        );
                    }
                }
            },
            Err(e) => { println!("Error! {:?}", e); }
        }
    }
}

/// Get notes (or enter through to leave notes blank) about the job listing.
fn get_notes() -> String {
    println!("\n{}",
        Style::new().bold().paint("(Optional) Enter any notes for this position:")
    );

    let mut notes = String::new();
    
    match io::stdin().read_line(&mut notes) {
        Ok(_) => { return notes.trim().to_string(); },
        Err(e) => { 
            println!("Error! {:?}", e);
            return "".to_string(); 
        }
    }
}

/// Return the Job struct created from the date, job title, job application 
/// status, and notes.
pub fn add_job(company: String) -> Job {
    let title = get_title(&company);

    Job::new_job(
        Local::now().format("%m-%d-%Y %H:%M:%S").to_string(),
        company,
        title,
        get_status(),
        get_notes()
    )
}

/// Confirm addition of the new job listing to the spreadsheet.
pub fn confirm_add(new_job: Job) {
    loop {
        println!("\n{}", Style::new().bold().paint("Confirm? [Y/N]"));
        
        let mut confirm_in = String::new();

        match io::stdin().read_line(&mut confirm_in) {
            Ok(_) => { 
                match confirm_in.trim().to_uppercase().as_str() {
                    "Y" => {
                        mcsv::write_new_job(&new_job)
                            .expect("Failed writing to spreadsheet");
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
