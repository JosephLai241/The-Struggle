#===============================================================================
#                               Global Variables
#===============================================================================
import datetime as dt
from colorama import init, Fore, Style

### Automatically turn off color changes at the end of every print. Ensure colorama works on Windows
init(autoreset=True)

date = dt.datetime.now().strftime("%m-%d-%y %H:%M:%S")

options = ["y","n"]

f_name = "job_applications.csv" 

insight_options = ["all","pending","in_progress","offers","hired","rejected"]
job_categories = ["DATE ADDED","COMPANY","JOB TITLE","STATUS","NOTES"]
list_options = ["date","date_reverse","company","title","status","notes"]
status_options = ["PENDING","IN PROGRESS","OFFER RECEIVED","HIRED","REJECTED"]

status_prompt = """
    SELECT JOB STATUS
------------------------
    0: PENDING
    1: IN PROGRESS
    2: OFFER RECEIVED
    3: HIRED
    4: REJECTED
------------------------
            """

### Set color of font depending on status
def set_color(description,details):
    if details[3] == status_options[0]:
        print(Fore.BLUE + Style.BRIGHT + description)
    elif details[3] == status_options[1]:
        print(Fore.YELLOW + Style.BRIGHT + description)
    elif details[3] == status_options[2]:
        print(Fore.MAGENTA + Style.BRIGHT + description)
    elif details[3] == status_options[3]:
        print(Fore.GREEN + description)
    elif details[3] == status_options[4]:
        print(Fore.RED + description)
