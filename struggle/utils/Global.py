#===============================================================================
#                               Global Variables
#===============================================================================
from colorama import init, Fore, Style
import datetime as dt

### Automate sending reset sequences to turn off color changes at the end of 
### every print.
init(autoreset = True)

### Get current date.
date = dt.datetime.now().strftime("%m-%d-%y %H:%M:%S")

### Confirmation options.
options = ["y", "n"]

### Spreadsheet file name.
f_name = "../job_applications.csv" 

### Categories or options used throughout the program.
job_categories = ["DATE ADDED", "COMPANY", "JOB TITLE", "STATUS", "NOTES"]
list_options = ["date", "newest", "company", "title", "status", "notes"]
status_options = ["PENDING", "IN PROGRESS", "OFFER RECEIVED", "HIRED", "REJECTED"]

### Job status prompt.
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

### Set color of font depending on status.
def set_color(description, details):
    if details[3] == status_options[0]:
        print(Fore.BLUE + Style.BRIGHT + description)
    elif details[3] == status_options[1]:
        print(Fore.YELLOW + Style.BRIGHT + description)
    elif details[3] == status_options[2]:
        print(Fore.MAGENTA + Style.BRIGHT + description)
    elif details[3] == status_options[3]:
        print(Fore.GREEN + Style.BRIGHT + description)
    elif details[3] == status_options[4]:
        print(Fore.RED + Style.BRIGHT + description)
