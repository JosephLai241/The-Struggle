#===============================================================================
#                       Return Job Application Insights
#===============================================================================
from .. import global_vars
from ..functions import csv_functions

def print_insights(args,insight_functions,parser):
    insight_functions.check_insight_arg(args,parser)
    master = csv_functions.get_jobs()
    insights = insight_functions.Calculate(master,global_vars.status_options)
    n_jobs = insights.count_all(master)
    table = insights.make_table(n_jobs)
    insights.print_table(args.insights,table)
