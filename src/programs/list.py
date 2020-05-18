#===============================================================================
#                               List Saved Jobs
#===============================================================================
from prettytable import PrettyTable
from .. import global_vars

def list_jobs(args,list_functions):
    table = PrettyTable(global_vars.job_categories)
    table.align = "l"
    master = list_functions.get_jobs()
    list_functions.add_jobs(master,table)
    list_functions.select_sort(args,table)
