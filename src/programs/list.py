#===============================================================================
#                               List Saved Jobs
#===============================================================================
from prettytable import PrettyTable
from .. import global_vars
from ..functions import csv_functions

def list_jobs(args,list_functions,parser):
    list_functions.check_list_arg(args,parser)
    table = PrettyTable(global_vars.job_categories)
    table.align = "l"
    master = csv_functions.get_jobs()
    list_functions.add_jobs(master,table)
    list_functions.select_sort(args,table)
