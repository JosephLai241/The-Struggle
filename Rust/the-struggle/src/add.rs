use crate::model::Job;

use chrono::prelude::*;
use std::io;

fn get_title(company: &String) -> String {
    // Enter a job title.
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
            Err(e) => {println!("Error! {:?}", e);}
        }
    }
}

fn get_status() -> String {
    // Enter job application status.
    let status_options: Vec<String> = vec!["PENDING".to_string(), "IN PROGRESS".to_string(), 
                                            "OFFER RECEIVED".to_string(), "HIRED".to_string(), 
                                            "REJECTED".to_string()];

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
                    println!("Please select a status option.")
                } else{
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

fn get_notes() -> String {
    // Enter notes about the job listing.
    let mut notes = String::new();

    loop {
        println!("\nEnter any notes for this position:");
        match io::stdin().read_line(&mut notes) {
            Ok(_) => {
                return notes.trim().to_string();
            },
            Err(e) => {println!("Error! {:?}", e);}
        }
    }
}

pub fn add_job(company: String) -> Job {
    // Return Job struct from date, job title, job status, and notes.
    let local: DateTime<Local> = Local::now();

    let date = local.format("%m-%d-%Y %H:%M:%S").to_string();
    let title = get_title(&company);
    let status = get_status();
    let notes = get_notes();

    return Job::new_job(date, company, title, status, notes);
}

fn format_print(c_len: &usize, t_len: usize, n_len: &usize, vector: &Vec<String>) -> usize {
    // Format how the table header and jobs are printed in the terminal.
    let line = format!(
        "\n{:<21} {:<c_len$} {:<t_len$} {:<16} {:<n_len$}",
        vector[0], vector[1], vector[2], vector[3], 
        vector[4], 
        c_len = c_len, t_len = t_len, n_len = n_len
    );
    println!("{}", line);

    let max = line.len();
    return max;
}

pub fn print_job(job: Job) -> Vec<String> {
    // Print the new job listing to add to the spreadsheet.
    let job_categories: Vec<String> = vec!["DATE ADDED".to_string(), "COMPANY".to_string(), 
                                            "JOB TITLE".to_string(), "STATUS".to_string(), 
                                            "NOTES".to_string()];
    let c_len = &job.company.len() + 4;
    let t_len = if &job.title.len() > &10 {&job.title.len() + 2} 
                else {job_categories[2].len()};
    let n_len = &job.notes.len() + 2;

    let max = format_print(&c_len, t_len, &n_len, &job_categories);
    println!("{:-<width$}", "", width = max - 2);
    
    let details: Vec<String> = vec![job.date, job.company, job.title, job.status,
    job.notes];

    format_print(&c_len, t_len, &n_len, &details);

    return details;
}

pub fn confirm_new_job(new_job: Job) {
    
}