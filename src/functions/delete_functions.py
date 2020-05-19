#===============================================================================
#                               Delete Functions
#===============================================================================
from colorama import Fore, init, Style
from . import csv_functions

init(autoreset=True)

### Select a match
def select_job(matches,n):
    while True:
        try:
            selected = str(input("Select a job to delete (number): "))
            if int(selected) not in range(0,n):
                raise ValueError
            else:
                return int(selected)
        except ValueError:
            print("\nNot an option! Try again.\n")

### Confirm deleting a match
def delete_listing(master,matches,selected):
    del master[matches[selected][1]]
    print(Fore.RED + Style.BRIGHT + "\nDELETED LISTING.\n")
    csv_functions.overwrite(master)
