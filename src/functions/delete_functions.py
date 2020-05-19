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
                print((Style.BRIGHT + "\nDeleting %s listing at %s\n") % (matches[int(selected)][0].title,matches[int(selected)][0].company))
                return int(selected)
        except ValueError:
            print("\nNot an option! Try again.\n")

### Confirm deleting a match
def delete_listing(master,selected):
    print(Fore.RED + Style.BRIGHT + "\nDELETED LISTING.\n")
    del master[selected]
    csv_functions.overwrite(master)
