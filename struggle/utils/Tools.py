#===============================================================================
#                           Run all tracking tools
#===============================================================================
from .Cli import Parser
from . import Cli, Delete, Insights, List, New, Titles, Update

class Run():
    """
    Run all job tracking options.
    """

    Titles.Titles.main_title()

    args, parser = Parser().parse_args()
    
    if args.add:
        ### Add new job to spreadsheet.
        New.AddJob.add(args, parser)
    if args.update:
        ### Update an existing job in the spreadsheet.
        Update.UpdateJob.update(args, parser)
    if args.delete:
        ### Delete an existing job in the spreadsheet.
        Delete.DeleteJob.delete(args, parser)
    if args.list:
        ### List all existing jobs in the spreadsheet.
        List.ListJobs.list_all(args, parser)
    if args.insights:
        ### Return job application insights.
        Insights.ShowInsights.insights(args, parser)
