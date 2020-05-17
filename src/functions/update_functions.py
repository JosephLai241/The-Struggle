#===============================================================================
#                               Update Functions
#===============================================================================
import csv
import re
from colorama import Style
from . import csv_functions
from .. import global_vars, model

job_categories = global_vars.job_categories
status_options = global_vars.status_options
status_prompt = global_vars.status_prompt

section_prompt = """
     UPDATE SECTION
-----------------------------
    0: COMPANY NAME
    1: JOB TITLE
    2: APPLICATION STATUS
    3: NOTES
-----------------------------
            """

### Search spreadsheet for existing job listings. Create master list of all listings
def find_job(args):
    master = []
    matches = []
    with open(global_vars.f_name,"r") as spreadsheet:
        csv_file = csv.reader(spreadsheet,delimiter=",")
        line = 0
        next(csv_file)
        for row in csv_file:
            if re.search("%s" % str(args.update[0]),"%s" % row[1],re.I):
                match = [model.Job(row[0],row[1],row[2],row[3],row[4]),line]
                matches.append(match)
            job = dict(zip(job_categories,row))
            master.append(job)
            line += 1

    return master,matches

### Set formatting for header
def set_print_format(matches):
    c_len = 0
    t_len = 0
    n_len = 0
    for match in matches:
        if len(match[0].company) > c_len:
            c_len = len(match[0].company) + 2
        if len(match[0].title) > t_len:
            t_len = len(match[0].title) + 2 if len(match[0].title) > 10 else len(job_categories[2]) + 2
        if len(match[0].notes) > n_len:
            n_len = len(match[0].notes) + 2

    return c_len,t_len,n_len

### Print all matches
def print_matches(matches):
    c_len,t_len,n_len = set_print_format(matches)
    found_header = f"\n{'='*61} EXISTING JOB LISTINGS {'='*62}"
    table_header = f"\nNumber {job_categories[0]:<{19}} {job_categories[1]:<{c_len}} {job_categories[2]:<{t_len}} {job_categories[3]:<{16}} {job_categories[4]:<{n_len}}"
    print(found_header)
    print(table_header)
    print("-"*len(table_header))
    
    n = 0
    for match in matches:
        details = [match[0].date,match[0].company,match[0].title,match[0].status,match[0].notes]
        description = f"{n:<6} {details[0]:<{19}} {details[1]:<{c_len}} {details[2]:<{t_len}} {details[3]:<{16}} {details[4]:<{n_len}}\n"
        global_vars.set_color(description,details,c_len,t_len,n_len)

        n += 1

    return n

### Select a match
def select_job(matches,n):
    while True:
        try:
            selected = str(input("Select a job to update (number): "))
            if int(selected) not in range(0,n):
                raise ValueError
            else:
                print((Style.BRIGHT + "\nChanging %s listing at %s\n") % (matches[int(selected)][0].title,matches[int(selected)][0].company))
                return int(selected)
        except ValueError:
            print("\nNot an option! Try again.\n")

### Display update prompt
def update_prompt():
    while True:
        try:
            section = str(input(section_prompt))
            if not section or int(section) not in range(0,4):
                raise ValueError
            else:
                return int(section)
        except ValueError:
            print("\nNot an option!\n")

### Update job status
def check_status():
    while True:
        try:
            status = str(input(status_prompt).strip())
            if not status or int(status) not in range(0,5):
                raise ValueError
            else:
                return status_options[int(status)]
        except ValueError:
            print("\nNot an option!\n")

### Update section
def update_section(section,matches,selected):
    if section == 0:
        update = str(input("\nWhat is the new company name? ")).strip()
        matches[selected][0].company = update
    elif section == 1:
        update = str(input("\nWhat is the new job title? ")).strip()
        matches[selected][0].title = update
    elif section == 2:
        update = check_status()
        matches[selected][0].status = update
    elif section == 3:
        update = str(input("\nWhat are the new notes? ")).strip()
        matches[selected][0].notes = update

    return matches[selected]

### Header for listing changes
def list_changes_header(matches,selected):
    c_len,t_len,n_len = set_print_format(matches)
    details = [matches[selected][0].date,matches[selected][0].company,matches[selected][0].title,matches[selected][0].status,matches[selected][0].notes]
    description = f"{details[0]:<{19}} {details[1]:<{c_len}} {details[2]:<{t_len}} {details[3]:<{16}} {details[4]:<{n_len}}\n"
    table_header = f"\n{job_categories[0]:<{19}} {job_categories[1]:<{c_len}} {job_categories[2]:<{t_len}} {job_categories[3]:<{16}} {job_categories[4]:<{n_len}}"
    print(table_header)
    print("-"*len(table_header))

    return c_len,t_len,n_len,details,description

### List the changes made to a current listing
def list_changes(matches,selected):
    c_len,t_len,n_len,details,description = list_changes_header(matches,selected)
    print(Style.BRIGHT + "\nNEW LISTING\n")
    global_vars.set_color(description,details,c_len,t_len,n_len)

### Confirm changes
def confirm_update(parser):
    while True:
        try:
            confirm = str(input("Confirm Changes? [Y/N] ")).strip().lower()
            if confirm not in global_vars.options:
                raise ValueError
            elif confirm == global_vars.options[0]:
                return
            elif confirm == global_vars.options[1]:
                print("\nEXITING.\n")
                parser.exit()
        except ValueError:
            print("\nNot an option! Try again.\n")

### Update spreadsheet
def write_changes(job_listing,master):
    index = job_listing[1]
    master[index] = dict(zip(job_categories,[job_listing[0].date,job_listing[0].company,job_listing[0].title,job_listing[0].status,job_listing[0].notes]))
    csv_functions.update_job(master)