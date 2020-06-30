use crate::model::Job;

use ansi_term::*;
use prettytable::*;
use regex::Regex;

use std::collections::BTreeMap;
use std::io;
use std::process;

/// Set the color of the match within the PrettyTable depending on the job status.
fn set_match_color(status: &str) -> String {
    match status {
        "PENDING" => return "Fbl".to_string(),
        "IN PROGRESS" => return "Fyl".to_string(),
        "OFFER RECEIVED" => return "Fml".to_string(),
        "HIRED" => return "Fgl".to_string(),
        "REJECTED" => return "Frl".to_string(),
        _ => return "".to_string()
    };
}

/// Return a new vector of styled PrettyTable cells to add to the master table.
fn convert_match_details(job_details: &Vec<&String>, style: &str) -> Vec<Cell> {
    let mut pt_row: Vec<Cell> = Vec::new();
    for job in job_details {
        pt_row.push(Cell::new(&job).style_spec(style));
    }

    pt_row
}

/// Find and print job matches from the spreadsheet in a PrettyTable. Returns a 
/// vector of the match indexes within the master BTreeMap. Exits the program if
/// no matches were found.
pub fn print_matches(company: &str, master: BTreeMap<u16, Job>) -> Vec<u16> {
    let mut matches = Table::new();
    let mut match_indexes: Vec<u16> = Vec::new();

    matches.add_row(
        row![
            bFl -> "NUMBER",
            bFl -> "DATE ADDED", 
            bFl -> "COMPANY", 
            bFl -> "JOB TITLE", 
            bFl -> "STATUS", 
            bFl -> "NOTES"
        ]
    );

    let search_string = format!(r"(?i){}", company);
    let re = Regex::new(&search_string).unwrap();

    for i in 0u16..master.len() as u16 {
        let existing_company = &master.get_key_value(&i).unwrap().1.company.to_string();
        match re.find(&existing_company) {
            Some(_) => {
                let index = &i.to_string();
                let job_details = vec![
                    index,
                    &master.get_key_value(&i).unwrap().1.date,
                    &master.get_key_value(&i).unwrap().1.company,
                    &master.get_key_value(&i).unwrap().1.title,
                    &master.get_key_value(&i).unwrap().1.status,
                    &master.get_key_value(&i).unwrap().1.notes
                ];

                let style = set_match_color(&job_details[4]);
                let pt_row = convert_match_details(&job_details, &style);

                matches.add_row(Row::new(pt_row));
                match_indexes.push(i);
            },
            None => ()
        }
    }

    if match_indexes.is_empty() {
        println!(
            "\n{}\n", 
            Colour::Red.bold().paint("No matches found!")
        );
        process::exit(1);
    }

    matches.printstd();
    match_indexes
}

/// Select a match returned from searching.
pub fn select_match(match_indexes: Vec<u16>) -> u16 {
    let mut select = String::new();
    loop {
        println!("\n{}", Style::new().bold().paint("Select a job to modify (number):"));
        match io::stdin().read_line(&mut select) {
            Ok(_) => {
                if select.trim().is_empty() {
                    println!(
                        "\n{}", 
                        Colour::Red.bold().paint("Please select a valid match.")
                    );
                    select.clear();
                } else if !select.trim().chars().all(char::is_numeric) {
                    println!(
                        "\n{}", 
                        Colour::Red.bold().paint("Please select a valid match.")
                    );
                    select.clear();
                } else {
                    let select_int = select.trim().parse::<u16>().unwrap();
                    if !match_indexes.iter().any(|index| index == &select_int) {
                        println!(
                            "\n{}", 
                            Colour::Red.bold().paint("Please select a valid match.")
                        );
                        select.clear();
                    } else {
                        return match_indexes[
                            match_indexes
                            .iter()
                            .position(|index| index == &select_int)
                            .unwrap()
                        ];
                    }
                }
            },
            Err(_) => ()
        }
    }
}
