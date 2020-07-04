#===============================================================================
#                           Command-line Interface
#===============================================================================
import argparse
import sys

class Parser():
    """
    Methods for parsing CLI arguments.
    """

    ### Initialize objects that will be used in class methods.
    def __init__(self):
        self._usage = "track.py [-a COMPANY_NAME] [-u COMPANY_NAME] [-d COMPANY_NAME] [-l OPTIONAL_SORT_METHOD] [-i OPTIONAL_DISPLAY_METHOD]"
        self._description = r"""
Struggle Tracker - A program that helps you track your job applications

Author: Joseph Lai
"""
        self._epilog = r"""
LIST SORT OPTIONS
  date (default)    sort by date (descending)
  date_reverse      sort by date (ascending) 
  company           sort by company name
  title             sort by job title
  status            sort by status
  notes             sort by notes

INSIGHT DISPLAY OPTIONS
  all (default)     print all job status insights
  pending           print percentage of pending job applications
  in_progress       print percentage of in progress job applications
  offers            print percentage of job applications with an offer
  hired             print percentage of jobs you have been hired at
  rejected          print percentage of rejected job applications

EXAMPLES

Add job application at Stack Overflow to the spreadsheet:

    $ ./track.py -a "Stack Overflow"

Update job application at Apple in the spreadsheet. Returns list of matches at Apple if you applied to more than one job at the company:

    $ ./track.py -u Apple

Delete job application in the spreadsheet. Returns list of matches at Apple if you applied to more than one job at the company:

    $ ./track.py -d Apple

List all saved job applications in the terminal and sort by company name:

    $ ./track.py -l company

List job application insights for rejected jobs:

    $ ./track.py -i rejected

"""

    ### Add all flag options.
    def _add_flags(self, parser):
        tracker = parser.add_argument_group("TRACKING OPTIONS")
        tracker.add_argument(
            "-a", 
            "--add", 
            metavar = "", 
            nargs = 1, 
            help = "add new company to track")
        tracker.add_argument(
            "-u", 
            "--update", 
            metavar = "", 
            nargs = 1, 
            help = "update an existing company in the spreadsheet")
        tracker.add_argument(
            "-d", 
            "--delete", 
            metavar = "", 
            nargs = 1, 
            help = "delete an existing company in the spreadsheet")
        tracker.add_argument(
            "-l", 
            "--list", 
            const = "date", 
            default = None, 
            metavar = "", 
            nargs = "?", 
            help = "list all saved job applications")
        tracker.add_argument(
            "-i", 
            "--insights", 
            const = "all", 
            default = None, 
            metavar = "", 
            nargs = "?", 
            help = "display job application insights")

    ### Get args.
    def parse_args(self):
        parser = argparse.ArgumentParser(
            description = self._description,
            epilog = self._epilog,
            formatter_class = argparse.RawDescriptionHelpFormatter,
            usage = self._usage)

        self._add_flags(parser)

        ### Print help message if no arguments are present.
        if len(sys.argv[1:]) == 0:
            parser.print_help()
            parser.exit()

        args = parser.parse_args()
        return args, parser
