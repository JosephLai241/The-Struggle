#===============================================================================
#                       Command-line Interface Functions
#===============================================================================
import argparse
import sys

### Argparse text
usage = "track.py [-a COMPANY_NAME] [-u COMPANY_NAME] [-d COMPANY_NAME] [-l]"
description = "Struggle Tracker - A program that helps you track your job applications"
epilog = r"""
EXAMPLES

    Add job application at Stack Overflow to the spreadsheet:

        $ ./track.py -a "Stack Overflow"

    Update job application at Apple in the spreadsheet. Returns list of matches at Apple if you applied to more than one job at the company:

        $ ./track.py -u Apple

    Delete job application in the spreadsheet. Returns list of matches at Apple if you applied to more than one job at the company:

        $ ./track.py -d Apple

    List all saved job applications in the terminal:

        $ ./track.py -l
"""

# Parse args
def parse_args():
    parser = argparse.ArgumentParser(usage = usage, \
                                    formatter_class = argparse.RawDescriptionHelpFormatter, \
                                    description = description, \
                                    epilog = epilog)

    scraper = parser.add_argument_group("Tracking Options")
    scraper.add_argument("-a","--add",nargs=1,metavar="",help="add new company to track")
    scraper.add_argument("-u","--update",nargs=1,metavar="",help="update an existing company in the spreadsheet")
    scraper.add_argument("-d","--delete",nargs=1,metavar="",help="delete an existing company in the spreadsheet")
    scraper.add_argument("-l","--list",action="store_true",help="list all saved job applications")

    if len(sys.argv[1:]) == 0:
        parser.print_help()
        parser.exit()

    args = parser.parse_args()
    return parser,args