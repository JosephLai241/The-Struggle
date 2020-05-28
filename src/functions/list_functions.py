#===============================================================================
#                               List Functions
#===============================================================================

from colorama import init, Style
import csv

from .. import global_vars

init(autoreset = True)

job_categories = global_vars.job_categories
list_options = global_vars.list_options

titles = {0: "\nSorting by Date (DESCENDING)\n",
          1: "\nSorting by Date (ASCENDING)\n",
          2: "\nSorting by Company Name\n",
          3: "\nSorting by Job Title\n",
          4: "\nSorting by Status\n",
          5: "\nSorting by Notes\n"
         }

### Check list arg
def check_list_arg(args, parser):
    try:
        if args.list in list_options:
            return
        else:
            raise ValueError
    except ValueError:
        print("\nNot a sorting option!\n")
        print(Style.BRIGHT + "CHOOSE FROM: %s\n" % ", ".join(list_options))
        
        parser.exit()

### Add jobs to table
def add_jobs(master, table):
    for job in master:
        table.add_row([job[0], job[1], job[2], job[3], job[4]])

### Determine how to sort PrettyTable, then print
def select_sort(args, table):
    sort_by = str(args.list).strip().lower()
    index = 0
    sort_n = 0
    if sort_by == list_options[0]:
        index = sort_n = 0
    elif sort_by == list_options[1]:
        print(Style.BRIGHT + titles[1])
        print(table.get_string(sortby = job_categories[0], reversesort = True))
        return
    elif sort_by == list_options[2]:
        index = 2
        sort_n = 1
    elif sort_by == list_options[3]:
        index = 3
        sort_n = 2
    elif sort_by == list_options[4]:
        index = 4
        sort_n = 3
    elif sort_by == list_options[5]:
        index = 5
        sort_n = 4

    print(Style.BRIGHT + titles[index])
    print(table.get_string(sortby = job_categories[sort_n]))
