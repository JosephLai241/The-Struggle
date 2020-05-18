#===============================================================================
#                       Command-line Interface Functions
#===============================================================================
import argparse
import sys

### Argparse text
usage = "track.py [-a COMPANY_NAME] [-u COMPANY_NAME] [-d COMPANY_NAME] [-l] [-i]"
description = "Struggle Tracker - A program that helps you track your job applications"
epilog = r"""
LIST OPTIONS
  date           sort by date (descending)
  date_reverse   sort by date (ascending) 
  company        sort by company name
  title          sort by job title
  status         sort by status
  notes          sort by notes

INSIGHT OPTIONS
  all            print all job status insights
  pending        print percentage of pending job applications
  in_progress    print percentage of in progress job applications
  offers         print percentage of job applications with an offer
  hired          print percentage of jobs you have been hired at
  rejected       print percentage of rejected job applications

EXAMPLES

    Add job application at Stack Overflow to the spreadsheet:

        $ ./track.py -a "Stack Overflow"

    Update job application at Apple in the spreadsheet. Returns list of matches at Apple if you applied to more than one job at the company:

        $ ./track.py -u Apple

    Delete job application in the spreadsheet. Returns list of matches at Apple if you applied to more than one job at the company:

        $ ./track.py -d Apple

    List all saved job applications in the terminal and sort by company name:

        $ ./track.py -l company
"""

# Parse args
def parse_args():
    parser = argparse.ArgumentParser(usage = usage, \
                                    formatter_class = argparse.RawDescriptionHelpFormatter, \
                                    description = description, \
                                    epilog = epilog)

    scraper = parser.add_argument_group("TRACKING OPTIONS")
    scraper.add_argument("-a","--add",nargs=1,metavar="",help="add new company to track")
    scraper.add_argument("-u","--update",nargs=1,metavar="",help="update an existing company in the spreadsheet")
    scraper.add_argument("-d","--delete",nargs=1,metavar="",help="delete an existing company in the spreadsheet")
    scraper.add_argument("-l","--list",const="date",default=None,nargs="?",metavar="",help="list all saved job applications")
    scraper.add_argument("-i","--insights",const="all",default=None,nargs="?",metavar="",help="display job application insights")

    if len(sys.argv[1:]) == 0:
        parser.print_help()
        parser.exit()

    args = parser.parse_args()
    print(args)
    return parser,args
