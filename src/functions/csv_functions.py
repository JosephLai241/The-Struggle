#===============================================================================
#                               Export Functions
#===============================================================================
import csv
import os.path
from os import path
from .. import global_vars

f_name = global_vars.f_name

### Check if spreasheet exists
def existence():
    return path.exists("job_applications.csv")

### Write job to spreadsheet
def add_job(master):
    exists = existence()
    write_mode = "a" if exists else "w"
    with open(f_name,write_mode,encoding="utf-8") as addjob:
        if write_mode == "w":
            writer = csv.DictWriter(addjob,fieldnames=global_vars.job_categories)
            writer.writeheader()
            writer.writerow(master) 
        else:
            writer = csv.writer(addjob)
            writer.writerow(list(master.values()))

### Rewrite spreadsheet after changes are made (updating or deleting a job listing)
def overwrite(master):
    with open(f_name,"w",encoding="utf-8") as updatejob:
        writer = csv.DictWriter(updatejob,fieldnames=global_vars.job_categories)
        writer.writeheader()
        writer.writerows(master)

### Get all jobs in spreadsheet
def get_jobs():
    master = []
    with open(f_name,"r") as listing:
        csv_file = csv.reader(listing,delimiter=",")
        next(csv_file)
        for row in csv_file:
            master.append(row)
    
    return master