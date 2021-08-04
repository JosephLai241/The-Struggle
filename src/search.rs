//! Searching and printing matching job applications within a 
//! PrettyTable.

use crate::display::display_prompt;
use crate::format::*;
use crate::model::Job;

use ansi_term::*;
use prettytable::*;
use regex::Regex;

use std::collections::BTreeMap;
use std::io;
use std::process;

/// Find and add matches to the PrettyTable `matches` and the Vector `match_indexes`.
fn add_matches(
    company: &str, 
    master: &BTreeMap<u16, Job>, 
    match_indexes: &mut Vec<u16>, 
    matches: &mut Table) {
        let re = Regex::new(&format!(r"(?i){}", company)).unwrap();

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

                    let style = set_color(&job_details[4]);
                    let pt_row = convert_details(&job_details, &style);

                    matches.add_row(Row::new(pt_row));
                    match_indexes.push(i);
                },
                None => ()
            }
        }
}

/// Print job matches from the spreadsheet in a PrettyTable. Returns a 
/// vector of the match indexes within the master BTreeMap (the position of the 
/// job listing within the spreadsheet). Exits the program if no matches were found.
pub fn print_matches(company: &str, master: &BTreeMap<u16, Job>) -> Vec<u16> {
    let mut matches = Table::new();
    let mut match_indexes: Vec<u16> = Vec::new();

    matches.add_row(
        row![
            bil =>
            "NUMBER",
            "DATE ADDED", 
            "COMPANY", 
            "JOB TITLE", 
            "STATUS", 
            "NOTES"
    ]);

    add_matches(company, &master, &mut match_indexes, &mut matches);

    if match_indexes.is_empty() {
        println!("\n{}\n", Colour::Red.bold().paint("No matches found!"));
        process::exit(1);
    }

    matches.set_titles(Row::new(vec![
        Cell::new(&format!("FOUND {} MATCHES", matches.len() - 1))
            .style_spec("bicH6")
    ]));

    matches.set_format(format_table());
    matches.printstd();
    
    match_indexes
}

/// Select a match returned from searching.
pub fn select_match(match_indexes: Vec<u16>) -> u16 {
    loop {
        display_prompt(format!("\n{}", Style::new().bold().paint("Select a job to modify (number): ")));
        
        let mut select = String::new();

        match io::stdin().read_line(&mut select) {
            Ok(_) => {
                match select.trim().parse::<u16>() {
                    Ok(select_int) => {
                        if match_indexes.iter().any(|index| index == &select_int) {
                            return match_indexes[
                                match_indexes
                                    .iter()
                                    .position(|index| index == &select_int)
                                    .unwrap()
                            ];
                        } else {
                            println!("\n{}", 
                                Colour::Red.bold().paint("Please select a valid match.")
                            );
                        }
                    },
                    Err(_) => {
                        println!("\n{}", 
                            Colour::Red.bold().paint("Please select a valid match.")
                        );
                    }
                }
            },
            Err(_) => ()
        }
    }
}

/// Defining the types of data that can be passed through `print_selection()`.
pub enum DataType<'a> {
    Btm(u16, &'a BTreeMap<u16, Job>),
    Singular(&'a Job)
}

/// Match the data type that is passed into `print_selection()` and return the
/// vector of string type `job_details`.
fn match_data_type(data: DataType) -> Vec<String> {
    match data {
        DataType::Btm(job_index, master) => {
            vec![
                master.get(&job_index).unwrap().date.to_string(),
                master.get(&job_index).unwrap().company.to_string(),
                master.get(&job_index).unwrap().title.to_string(),
                master.get(&job_index).unwrap().status.to_string(),
                master.get(&job_index).unwrap().notes.to_string()
            ]
        },
        DataType::Singular(job) => {
            vec![
                job.date.clone(),
                job.company.clone(),
                job.title.clone(),
                job.status.clone(),
                job.notes.clone()
            ]
        }
    }
}

/// Print the selected job for updating or deletion.
pub fn print_selection(data: DataType, title: String) {
    println!("\n{}", Colour::Cyan.bold().paint(title));

    let mut selection = Table::new();

    selection.set_titles(
        row![
            bil => 
            "DATE ADDED", 
            "COMPANY", 
            "JOB TITLE", 
            "STATUS", 
            "NOTES"
    ]);

    let job_details = match_data_type(data);

    let style = set_color(&job_details[3]);
    let pt_row = convert_details(&job_details, &style);

    selection.add_row(Row::new(pt_row));
    selection.set_format(format_table());

    selection.printstd();
}

#[cfg(test)]
mod test_search {
    use super::*;

    #[test]
    fn test_print_matches() {
        let correct_indexes: Vec<u16> = vec![0, 1, 3];

        let mut test_master: BTreeMap<u16, Job> = BTreeMap::new();

        let test_job = Job {
            date: "".to_string(),
            company: "FIND THIS".to_string(),
            title: "".to_string(),
            status: "HIRED".to_string(),
            notes: "".to_string()
        };
        test_master.insert(0, test_job);

        let test_job = Job {
            date: "".to_string(),
            company: "FIND THIS".to_string(),
            title: "".to_string(),
            status: "REJECTED".to_string(),
            notes: "".to_string()
        };
        test_master.insert(1, test_job);

        let test_job = Job {
            date: "".to_string(),
            company: "NOT THIS".to_string(),
            title: "".to_string(),
            status: "REJECTED".to_string(),
            notes: "".to_string()
        };
        test_master.insert(2, test_job);

        let test_job = Job {
            date: "".to_string(),
            company: "FIND THIS".to_string(),
            title: "".to_string(),
            status: "REJECTED".to_string(),
            notes: "".to_string()
        };
        test_master.insert(3, test_job);

        let match_indexes = print_matches(&"find", &test_master);

        assert_eq!(correct_indexes, match_indexes);
    }
}
