use crate::mcsv;
use crate::model::Job;

use chrono::prelude::*;

use std::error::Error;
use std::io;
use std::process;

/// Get the title of the company.
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

/// Enter the job application status.
fn get_status() -> String {

    let status_options: Vec<String> = vec!["PENDING".to_string(), "IN PROGRESS".to_string(), 
        "OFFER RECEIVED".to_string(), "HIRED".to_string(), "REJECTED".to_string()];

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

/// Enter notes (or enter through to leave notes blank) about the job listing.
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

/// Return Job struct from date, job title, job status, and notes.
pub fn add_job(company: String) -> Job {
    let date = Local::now().format("%m-%d-%Y %H:%M:%S").to_string();
    let title = get_title(&company);
    let status = get_status();
    let notes = get_notes();

    Job::new_job(date, company, title, status, notes)
}

/// Format how the table header and jobs are printed in the terminal
fn format_print(c_len: &usize, t_len: usize, n_len: &usize, vector: &Vec<&String>) -> usize {
    let line = format!(
        "\n{:<21} {:<c_len$} {:<t_len$} {:<16} {:<n_len$}",
        vector[0], vector[1], vector[2], vector[3], vector[4], 
        c_len = c_len, t_len = t_len, n_len = n_len
    );
    println!("{}", line);

    let max = line.len();
    return max;
}

/// Print the new job listing to add to the spreadsheet.
fn print_job(job: &Job) {
    let job_categories: Vec<String> = vec!["DATE ADDED".to_string(), "COMPANY".to_string(), 
        "JOB TITLE".to_string(), "STATUS".to_string(), "NOTES".to_string()];
    
    let c_len = &job.company.len() + 4;
    let t_len = if &job.title.len() > &10 { &job.title.len() + 2 } 
        else { job_categories[2].len() };
    let n_len = &job.notes.len() + 2;

    // TODO: Find a way around borrow issue. Traits?
    let jc_borrowed: Vec<&String> = vec![&job_categories[0], &job_categories[1], 
        &job_categories[2], &job_categories[3], &job_categories[4]];
    let max = format_print(&c_len, t_len, &n_len, &jc_borrowed);
    println!("{:-<width$}", "", width = max - 2);
    
    let details: Vec<&String> = vec![&job.date, &job.company, &job.title, &job.status,
        &job.notes];

    format_print(&c_len, t_len, &n_len, &details);
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
                    // If input in options
                    if confirm == options[0] {  
                        // If input is "Y"
                        mcsv::write_new_job(&new_job)?;
                    } else {
                        // If input is "N"
                        println!("\nEXITING.");
                        process::exit(1);
                    }
                } else { 
                    // If invalid option is entered
                    println!("\nNot an option!");
                    confirm_in.clear(); 
                }
            },
            Err(e) => { println!("Error! {:?}", e); }
        }
    }
}
