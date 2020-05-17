#===============================================================================
#                               Global Variables
#===============================================================================
import datetime as dt
from colorama import init, Fore, Style

### Automatically turn off color changes at the end of every print. Ensure colorama works on Windows
init(autoreset=True)


date = dt.datetime.now().strftime("%m-%d-%y %H:%M:%S")

options = ["y","n"]

job_categories = ["DATE ADDED","COMPANY","JOB TITLE","STATUS","NOTES"]
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

f_name = "job_applications.csv" 

### Set color of font depending on status
def set_color(description,details,c_len,t_len,n_len):
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