//! Searching and printing matching job applications within a 
//! PrettyTable.

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
            bFl -> "NUMBER",
            bFl -> "DATE ADDED", 
            bFl -> "COMPANY", 
            bFl -> "JOB TITLE", 
            bFl -> "STATUS", 
            bFl -> "NOTES"
        ]
    );

    add_matches(company, &master, &mut match_indexes, &mut matches);

    if match_indexes.is_empty() {
        println!("\n{}\n", 
            Colour::Red.bold().paint("No matches found!")
        );
        process::exit(1);
    }

    matches.set_titles(Row::new(vec![
        Cell::new(&format!("FOUND {} MATCHES", matches.len() - 1))
            .style_spec("bcH6")
    ]));

    matches.set_format(format_table());
    matches.printstd();
    
    match_indexes
}

/// Select a match returned from searching.
pub fn select_match(match_indexes: Vec<u16>) -> u16 {
    loop {
        let mut select = String::new();

        println!("\n{}", Style::new().bold().paint("Select a job to modify (number):"));

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
