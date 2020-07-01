use crate::mcsv::overwrite;
use crate::model::Job;

use std::collections::BTreeMap;

/// Update the keys within the BTreeMap after deleting a job.
fn update_keys(master: &mut BTreeMap<u16, Job>) -> BTreeMap<u16, Job> {
    let mut update: BTreeMap<u16, Job> = BTreeMap::new();

    for (index, job) in master.values_mut().enumerate() {
        let index = index as u16;
        
        update.insert(index, Job::new_job(
            job.date.clone(),
            job.company.clone(),
            job.title.clone(),
            job.status.clone(),
            job.notes.clone()
        ));
    }

    update
}

/// Delete the selected job from the master BTreeMap. Then rewrite the spreadsheet.
pub fn delete_job(job_index: u16, master: &mut BTreeMap<u16, Job>) {
    master.remove(&job_index);
    let mut update = update_keys(master);
    overwrite(&mut update).expect("Failed to overwrite spreadsheet.");
}
