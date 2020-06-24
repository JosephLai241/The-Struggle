#===============================================================================
#                              Deleting a Job
#===============================================================================
from colorama import Fore, init, Style

from .Csv import ModifyCSV
from .Decorator import CleanExit
from .Search import Find, PrintMatches

### Automate sending reset sequences to turn off color changes at the end of 
### every print.
init(autoreset = True)

class Delete():
    """
    Methods for deleting a job from the spreadsheet.
    """

    ### Select a match from the spreadsheet.
    @staticmethod
    @CleanExit.cleanup
    def select_job(matches, n):
        while True:
            try:
                selected = str(input("Select a job to delete (number): "))
                if int(selected) not in range(0, n + 1):
                    raise ValueError
                else:
                    return int(selected)
            except ValueError:
                print("\nNot an option! Try again.\n")

    ### Delete the match from the spreadsheet.
    @staticmethod
    def delete_listing(master, matches, selected):
        del master[matches[selected][1]]
        ModifyCSV.overwrite(master)

        print(Fore.RED + Style.BRIGHT + "\nDELETED LISTING.\n")

class DeleteJob():
    """
    Run Delete methods.
    """

    ### Run delete.
    @staticmethod
    def delete(args, parser):
        master, matches = Find.find_job(args, parser)
        n = PrintMatches.print_matches(matches)

        selected = Delete.select_job(matches, n)
        PrintMatches.list_changes(args, matches, selected)
        PrintMatches.confirm_changes(parser)

        Delete.delete_listing(master, matches, selected)
