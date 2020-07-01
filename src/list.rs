use crate::format::*;
use crate::model::Job;

use prettytable::*;

use std::collections::BTreeMap;

/// Add each stored job as a row in the PrettyTable.
fn add_rows(job_table: &mut Table, master: BTreeMap<u16, Job>) {
    for i in 0u16..master.len() as u16 {
        let job_details = vec![
            master.get_key_value(&i).unwrap().1.date.to_string(),
            master.get_key_value(&i).unwrap().1.company.to_string(),
            master.get_key_value(&i).unwrap().1.title.to_string(),
            master.get_key_value(&i).unwrap().1.status.to_string(),
            master.get_key_value(&i).unwrap().1.notes.to_string()
        ];

        let style = set_color(&job_details[3]);
        let pt_row = convert_details(&job_details, &style);

        job_table.add_row(Row::new(pt_row));
    }
}

/// Print the stored jobs in a PrettyTable.
pub fn list_jobs(master: BTreeMap<u16, Job>) {
    let mut job_table = Table::new();

    job_table.add_row(
        row![
            bFl -> "DATE ADDED", 
            bFl -> "COMPANY", 
            bFl -> "JOB TITLE", 
            bFl -> "STATUS", 
            bFl -> "NOTES"
        ]
    );

    add_rows(&mut job_table, master);

    job_table.printstd();
}

////////////////////////////////////////////////////////////////////////////////

// /// Enum containing all the ways you can sort the table of jobs. Currently Rust's 
// /// PrettyTable does not support sorting so I am commenting this out for now.
// enum Titles {
//     Date,
//     Reverse,
//     Company,
//     JobTitle,
//     Status,
//     Notes
// }

// /// Return the string indicating how the table is sorted. Currently Rust's 
// /// PrettyTable does not support sorting so I am commenting this out for now.
// impl Titles {
//     fn return_title(sort: Titles) -> String {
//         match sort {
//             Titles::Date => "Sorting by Date (Descending)".to_string(),
//             Titles::Reverse => "Sorting by Newest".to_string(),
//             Titles::Company => "Sorting by Company Name".to_string(),
//             Titles::JobTitle => "Sorting by Job Title".to_string(),
//             Titles::Status => "Sorting by Status".to_string(),
//             Titles::Notes => "Sorting by Notes".to_string()
//         }
//     }
// }
