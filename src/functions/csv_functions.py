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

### Update a job in the spreadsheet
def update_job(master):
    with open(f_name,"w",encoding="utf-8") as updatejob:
        writer = csv.DictWriter(updatejob,fieldnames=global_vars.job_categories)
        writer.writeheader()
        writer.writerows(master)

### Delete job from spreadsheet
def delete_job(master):
    with open(f_name,"w",encoding="utf-8"):
        pass