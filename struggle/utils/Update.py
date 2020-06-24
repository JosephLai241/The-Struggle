#===============================================================================
#                                Update a Job
#===============================================================================
from colorama import Fore, init, Style

from .Csv import ModifyCSV
from .Decorator import CleanExit
from .Global import job_categories, status_options, status_prompt
from .Search import Find, PrintMatches

### Automate sending reset sequences to turn off color changes at the end of 
### every print.
init(autoreset = True)

### Prompt that is shown for updating a job's attributes.
section_prompt = """
     UPDATE SECTION
-----------------------------
    0: COMPANY NAME
    1: JOB TITLE
    2: APPLICATION STATUS
    3: NOTES
-----------------------------
            """

class Update():
    """
    Methods for updating a job in the spreadsheet.
    """

    ### Select a match.
    @staticmethod
    @CleanExit.cleanup
    def select_job(matches, n):
        while True:
            try:
                selected = str(input("Select a job to update (number): "))
                if int(selected) not in range(0, n + 1):
                    raise ValueError
                else:
                    print((Style.BRIGHT + "\nUpdating %s job listing at %s\n") % 
                        (matches[int(selected)][0].title, 
                            matches[int(selected)][0].company))
                    return int(selected)
            except ValueError:
                print("\nNot an option! Try again.\n")

    ### Display update prompt.
    @staticmethod
    @CleanExit.cleanup
    def update_prompt():
        while True:
            try:
                section = str(input(section_prompt))
                if not section or int(section) not in range(0, 4):
                    raise ValueError
                else:
                    return int(section)
            except ValueError:
                print("\nNot an option!\n")

    ### Update job status.
    @staticmethod
    @CleanExit.cleanup
    def check_status():        
        while True:
            try:
                status = str(input(status_prompt).strip())
                if not status or int(status) not in range(0, 5):
                    raise ValueError
                else:
                    return status_options[int(status)]
            except ValueError:
                print("\nNot an option!\n")

    ### Pythonic switch to get the new value of whichever section is updated.
    @staticmethod
    def get_update(section):
        options_switch = {
            0: "\nWhat is the new company name? ",
            1: "\nWhat is the new job title? ",
            2: Update.check_status() if int(section) == 2 else None,
            3: "\nWhat are the new notes? "
        }

        update = str(input(options_switch.get(int(section)))).strip() \
            if int(section) != 2 else options_switch.get(int(section))

        return update
    
    ### Set the Job's attribute depending on which section is updated.
    @staticmethod
    @CleanExit.cleanup
    def update_section(section, matches, selected):
        update = Update.get_update(section)

        if section == 0:
            matches[selected][0].company = update
        elif section == 1:
            matches[selected][0].title = update
        elif section == 2:
            matches[selected][0].status = update
        elif section == 3:
            matches[selected][0].notes = update

        return matches[selected]

    ### Update the spreadsheet.
    @staticmethod
    def write_changes(job_listing, master):
        index = job_listing[1]
        master[index] = dict(zip(job_categories, [job_listing[0].date, 
            job_listing[0].company, job_listing[0].title, job_listing[0].status, 
            job_listing[0].notes]))

        ModifyCSV.overwrite(master)
        print(Fore.CYAN + Style.BRIGHT + "\nUPDATED LISTING.\n")

class UpdateJob():
    """
    Run Update methods.
    """

    ### Run update.
    @staticmethod
    def update(args, parser):
        master, matches = Find.find_job(args, parser)
        n = PrintMatches.print_matches(matches)

        selected = Update.select_job(matches, n)
        section = Update.update_prompt()
        job_listing = Update.update_section(section, matches, selected)
        
        PrintMatches.list_changes(args, matches, selected)
        PrintMatches.confirm_changes(parser)

        Update.write_changes(job_listing, master)