#===============================================================================
#                               List Functions
#===============================================================================

from colorama import init, Style
from .. import global_vars
import csv

init(autoreset=True)

job_categories = global_vars.job_categories
status_options = global_vars.status_options

### Add jobs to table 
def add_jobs(master,table):
    for job in master:
        table.add_row([job[0],job[1],job[2],job[3],job[4]])

### Determine how to sort PrettyTable, then print
def select_sort(args,table):
    sort_by = str(args.list).strip().lower()
    try:
        if sort_by == global_vars.list_options[0]:
            print(Style.BRIGHT + "\nSorting by Date (DESCENDING)\n")
            print(table.get_string(sortby=job_categories[0]))
        elif sort_by == global_vars.list_options[1]:
            print(Style.BRIGHT + "\nSorting by Date (ASCENDING)\n")
            print(table.get_string(sortby=job_categories[0],reversesort=True))
        elif sort_by == global_vars.list_options[2]:
            print(Style.BRIGHT + "\nSorting by Company Name\n")
            print(table.get_string(sortby=job_categories[1]))
        elif sort_by == global_vars.list_options[3]:
            print(Style.BRIGHT + "\nSorting by Job Title\n")
            print(table.get_string(sortby=job_categories[2]))
        elif sort_by == global_vars.list_options[4]:
            print(Style.BRIGHT + "\nSorting by Status\n")
            print(table.get_string(sortby=job_categories[3]))
        elif sort_by == global_vars.list_options[5]:
            print(Style.BRIGHT + "\nSorting by Notes\n")
            print(table.get_string(sortby=job_categories[4]))
        elif sort_by not in global_vars.list_options:
            raise ValueError
    except ValueError:
        print(Style.BRIGHT + "\nNot a sorting option!\n")