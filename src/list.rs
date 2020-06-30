use crate::model::Job;

use prettytable::*;

use std::collections::BTreeMap;

/// Set the color of the job within the PrettyTable depending on the job status.
fn set_job_color(index: &u16, master: &BTreeMap<u16, Job>) -> String {
    let job_status = &master.get_key_value(&index).unwrap().1.status;
    match job_status.as_str() {
        "PENDING" => return "Fbl".to_string(),
        "IN PROGRESS" => return "Fyl".to_string(),
        "OFFER RECEIVED" => return "Fml".to_string(),
        "HIRED" => return "Fgl".to_string(),
        "REJECTED" => return "Frl".to_string(),
        _ => return "".to_string()
    };
}

/// Return a new vector of styled PrettyTable cells to add to the master table.
fn convert_job_details(job_details: Vec<String>, style: &str) -> Vec<Cell> {
    let mut pt_row: Vec<Cell> = Vec::new();
    for job in job_details {
        pt_row.push(Cell::new(&job).style_spec(style));
    }

    pt_row
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

    for i in 0u16..master.len() as u16 {
        let job_details = vec![
            master.get_key_value(&i).unwrap().1.date.to_string(),
            master.get_key_value(&i).unwrap().1.company.to_string(),
            master.get_key_value(&i).unwrap().1.title.to_string(),
            master.get_key_value(&i).unwrap().1.status.to_string(),
            master.get_key_value(&i).unwrap().1.notes.to_string()
        ];

        let style = set_job_color(&i, &master);
        let pt_row = convert_job_details(job_details, &style);

        job_table.add_row(Row::new(pt_row));
    }

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
