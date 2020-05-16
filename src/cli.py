#===============================================================================
#                       Command-line Interface Functions
#===============================================================================
import argparse
import sys

### Argparse text
usage = "track.py [-n COMPANY_NAME] [-u COMPANY_NAME] [-d COMPANY_NAME]"
description = "Struggle Tracker - A program that helps you track your job applications"
epilog = r"""
EXAMPLES

    Add job application at Stack Overflow to the spreadsheet:

        $ ./track.py -n "Stack Overflow"

    Update job application at Apple in the spreadsheet:

        $ ./track.py -u Apple

    Delete job application in the spreadsheet:

        $ ./track.py -d Apple
"""

# Parse args
def parse_args():
    parser = argparse.ArgumentParser(usage = usage, \
                                    formatter_class = argparse.RawDescriptionHelpFormatter, \
                                    description = description, \
                                    epilog = epilog)

    scraper = parser.add_argument_group("Tracking Options")
    scraper.add_argument("-n","--new",nargs=1,metavar="",help="enter new company to track")
    scraper.add_argument("-u","--update",nargs=1,metavar="",help="update an existing company in the spreadsheet")
    scraper.add_argument("-d","--delete",nargs=1,metavar="",help="delete an existing company in the spreadsheet")

    if len(sys.argv[1:]) == 0:
        parser.print_help()
        parser.exit()

    args = parser.parse_args()
    print(args)
    return parser,args