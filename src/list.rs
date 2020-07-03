//! Listing stored job applications in a PrettyTable.

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
            bFl => 
            "DATE ADDED", 
            "COMPANY", 
            "JOB TITLE", 
            "STATUS", 
            "NOTES"
        ]
    );

    add_rows(&mut job_table, master);

    job_table.set_titles(Row::new(vec![
        Cell::new(&format!("{} TRACKED JOB APPLICATIONS", job_table.len() - 1))
            .style_spec("bcH5")
    ]));
    
    job_table.set_format(format_table());
    job_table.printstd();
}
