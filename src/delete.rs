use crate::mcsv::overwrite;
use crate::model::Job;

use std::collections::BTreeMap;

/// Update the keys within the BTreeMap after deleting a job.
fn update_keys(master: &mut BTreeMap<u16, Job>) -> BTreeMap<u16, Job> {
    let mut update: BTreeMap<u16, Job> = BTreeMap::new();

    for (index, job) in master.values_mut().enumerate() {
        let index = index as u16;

        let date = job.date.clone();
        let company = job.company.clone();
        let title = job.title.clone();
        let status = job.status.clone();
        let notes = job.notes.clone();

        update.insert(index, Job::new_job(date, company, title, status, notes));
    }

    update
}

/// Delete the selected job from the master BTreeMap. Then rewrite the spreadsheet.
pub fn delete_job(job_index: u16, master: &mut BTreeMap<u16, Job>) {
    master.remove(&job_index);
    
    let mut update = update_keys(master);
    overwrite(&mut update).expect("Failed to overwrite spreadsheet.");
}
