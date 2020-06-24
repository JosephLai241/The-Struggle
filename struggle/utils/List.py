#===============================================================================
#                               List Functions
#===============================================================================
import csv

from colorama import init, Style
from prettytable import PrettyTable

from .Csv import GetCSV
from .Global import job_categories, list_options

### Automate sending reset sequences to turn off color changes at the end of 
### every print.
init(autoreset = True)

### Table headers.
titles = {
    0: "\nSorting by Date (DESCENDING)\n",
    1: "\nSorting by Newest\n",
    2: "\nSorting by Company Name\n",
    3: "\nSorting by Job Title\n",
    4: "\nSorting by Status\n",
    5: "\nSorting by Notes\n"
}

class Check():
    """
    Method to check list args.
    """

    ### Check list arg.
    @staticmethod
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

class ModifyList():
    """
    Methods for modifying the list of all saved jobs.
    """

    ### Add jobs to table.
    @staticmethod
    def add_jobs(master, table):
        for job in master:
            table.add_row([job[0], job[1], job[2], job[3], job[4]])

    ### Determine how to sort PrettyTable, then print it.
    @staticmethod
    def select_sort(args, table):
        sort_by = str(args.list).strip().lower()

        sort_n = list_options.index(sort_by)
        index = sort_n + 1

        if sort_n == 0:
            index = 0
        elif sort_n == 1:
            print(Style.BRIGHT + titles[1])
            print(table.get_string(sortby = job_categories[0], reversesort = True))
            return
        else:
            sort_n -= 1
            index -= 1

        print(Style.BRIGHT + titles[index])
        print(table.get_string(sortby = job_categories[sort_n]))

class ListJobs():
    """
    Run List methods.
    """

    @staticmethod
    def list_all(args, parser):
        Check.check_list_arg(args, parser)

        table = PrettyTable(job_categories)
        table.align = "l"

        master = GetCSV.get_jobs()
        
        ModifyList.add_jobs(master, table)
        ModifyList.select_sort(args, table)
