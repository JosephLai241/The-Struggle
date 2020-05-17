#===============================================================================
#                           New Job Profile Functions
#===============================================================================
from . import csv_functions
from .. import global_vars, model

job_categories = global_vars.job_categories
status_options = ["PENDING","IN PROGRESS","OFFER RECEIVED","HIRED","REJECTED"]

status_prompt = global_vars.status_prompt

### Enter job title to track
def new_title(args):
    while True:
        try:
            title = str(input("\nWhat is the title of the position you are applying for at %s? " % args.new[0])).strip()
            if not title:
                raise ValueError
            else:
                return title
        except ValueError:
            print("\nNo job title was entered!\n")

### Enter status
def new_status():
    while True:
        try:
            status = str(input(status_prompt).strip())
            if not status or int(status) not in range(0,5):
                raise ValueError
            else:
                return status_options[int(status)]
        except ValueError:
            print("\nNot an option!\n")

### Enter notes
def new_notes():
    notes = str(input("\nEnter notes regarding this position: ")).strip()
    return notes

### Make new Job
def new_job(args,status,title,notes):
    return model.Job(global_vars.date,args.new[0],title,status,notes)

### Make master dictionary
def make_master(job,details):
    return dict(zip(job_categories,details))

### Print settings
def print_settings(job):
    c_len = len(job.company) + 2
    t_len = len(job.title) + 2 if len(job.title) > 10 else len(job_categories[2]) + 2
    n_len = len(job.notes) + 2
    details = [job.date,job.company,job.title,job.status,job.notes]
    table_header = f"\n{job_categories[0]:<{19}} {job_categories[1]:<{c_len}} {job_categories[2]:<{t_len}} {job_categories[3]:<{16}} {job_categories[4]:<{n_len}}"

    print(table_header)
    print("-"*len(table_header))
    description = f"{details[0]:<{19}} {details[1]:<{c_len}} {details[2]:<{t_len}} {details[3]:<{16}} {details[4]:<{n_len}}\n"
    global_vars.set_color(description,details,c_len,t_len,n_len)

    return details

### Confirm settings
def confirm_new_job(job,parser):
    details = print_settings(job)
    master = make_master(job,details)

    while True:
        try:
            confirm = input("\nConfirm? [Y/N] ").strip().lower()
            if confirm == global_vars.options[0]:
                return master
            elif confirm == global_vars.options[1]:
                print("\nEXITING.\n")
                parser.exit()
            elif confirm not in global_vars.options:
                raise ValueError
        except ValueError:
            print("\nNot an option! Try again.\n")

### Write job to spreadsheet
def confirm_write(master):
    print("\nAdding new job to spreadsheet...")
    csv_functions.add_job(master)
