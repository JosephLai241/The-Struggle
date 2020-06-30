use crate::mcsv;
use crate::model::Job;

use ansi_term::*;
use chrono::prelude::*;
use prettytable::*;

use std::error::Error;
use std::io;
use std::process;

/// Get the job title at the company.
fn get_title(company: &String) -> String {
    let mut title = String::new();

    loop {
        println!("What is the title of the position you are applying for at {}?", company);
        match io::stdin().read_line(&mut title) {
            Ok(_) => {
                let input = title.trim().to_string();

                if input.is_empty() {
                    println!("Please enter a job title.\n");
                } else { 
                    return title.trim().to_string(); 
                }
            },
            Err(e) => { println!("Error! {:?}", e); }
        }
    }
}

/// Get the job application status.
fn get_status() -> String {
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

    let mut status = String::new();
    loop {
        println!("{}", status_prompt);
        match io::stdin().read_line(&mut status) {
            Ok(_) => {
                if status.trim().is_empty() {
                    println!("Please select a valid status option.");
                    status.clear();
                } else if !status.trim().chars().all(char::is_numeric) {
                    println!("Please select a valid status option.");
                    status.clear();
                } else {
                    let status_int = status.trim().parse::<usize>().unwrap();
                    
                    if std::usize::MIN <= status_int && 
                        status_int <= status_options.len() - 1 as usize {
                            return status_options[status_int].to_string();
                    } else {
                        println!("\nOut of range!");
                        status.clear();
                    }
                }
            },
            Err(e) => {println!("Error! {:?}", e);}
        }
    }
}

/// Get notes (or enter through to leave notes blank) about the job listing.
fn get_notes() -> String {
    let mut notes = String::new();
    loop {
        println!("\nEnter any notes for this position:");
        match io::stdin().read_line(&mut notes) {
            Ok(_) => { return notes.trim().to_string(); },
            Err(e) => { println!("Error! {:?}", e); }
        }
    }
}

/// Return the Job struct created from the date, job title, job status, and notes.
pub fn add_job(company: String) -> Job {
    let date = Local::now().format("%m-%d-%Y %H:%M:%S").to_string();
    let title = get_title(&company);
    let status = get_status();
    let notes = get_notes();

    Job::new_job(date, company, title, status, notes)
}

/// Print the PrettyTable containing new job listing information.
fn print_job(job: &Job) {
    println!("\n{}", Colour::Cyan.bold().paint("Current settings for the new job"));

    ptable!(
        [bF -> "DATE ADDED", bF -> "COMPANY", bF -> "JOB TITLE", bF -> "STATUS", bF -> "NOTES"],
        [&job.date, &job.company, &job.title, &job.status, &job.notes]
    );
}

/// Print the job listing to add, then ask user to confirm. On confirm, the program
/// will append the job to the spreadsheet.
pub fn confirm_new_job(new_job: Job) -> Result<(), Box<dyn Error>> {
    print_job(&new_job);

    let options: Vec<String> = vec!["Y".to_string(), "N".to_string()];

    let mut confirm_in = String::new();
    loop {
        println!("\nConfirm? [Y/N]");
        match io::stdin().read_line(&mut confirm_in) {
            Ok(_) => { 
                let confirm = confirm_in.trim().to_uppercase();
                if options.iter().any(|ch| ch == &confirm) {
                    // If input in options.
                    if confirm == options[0] {  
                        // If input is "Y".
                        mcsv::write_new_job(&new_job)?;
                    } else {
                        // If input is "N".
                        println!("\n{}", Colour::Red.bold().paint("CANCELLING."));
                        process::exit(1);
                    }
                } else { 
                    // If an invalid option is entered.
                    println!("\nNot an option!");
                    confirm_in.clear(); 
                }
            },
            Err(e) => { println!("Error! {:?}", e); }
        }
    }
}
