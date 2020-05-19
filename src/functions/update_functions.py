#===============================================================================
#                               Update Functions
#===============================================================================
from colorama import Fore, init, Style
from .. import global_vars
from . import csv_functions

init(autoreset=True)

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

### Select a match
def select_job(matches,n):
    while True:
        try:
            selected = str(input("Select a job to update (number): "))
            if int(selected) not in range(0,n):
                raise ValueError
            else:
                print((Style.BRIGHT + "\nUpdating %s job listing at %s\n") % (matches[int(selected)][0].title,matches[int(selected)][0].company))
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

### Update spreadsheet
def write_changes(job_listing,master):
    print(Fore.CYAN + Style.BRIGHT + "\nUPDATED LISTING.\n")
    index = job_listing[1]
    master[index] = dict(zip(job_categories,[job_listing[0].date,job_listing[0].company,job_listing[0].title,job_listing[0].status,job_listing[0].notes]))
    csv_functions.overwrite(master)
