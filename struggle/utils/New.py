#===============================================================================
#                           New Job Profile Functions
#===============================================================================
from colorama import Fore, init, Style

from .Csv import ModifyCSV
from .Decorator import CleanExit
from .Job import Job

from .Global import date, job_categories, options, set_color, status_prompt

### Automate sending reset sequences to turn off color changes at the end of 
### every print.
init(autoreset = True)

class GetDetails():
    """
    Methods for getting job details.
    """

    ### Enter job title to track.
    @staticmethod
    @CleanExit.cleanup
    def _new_title(args):
        while True:
            try:
                title = str(input(
                    "\nWhat is the title of the position you are applying for at %s? " % 
                        args.add[0])).strip()
                if not title:
                    raise ValueError
                else:
                    return title
            except ValueError:
                print("\nNo job title was entered!\n")

    ### Enter job status.
    @staticmethod
    @CleanExit.cleanup
    def _new_status():
        status_options = ["PENDING", "IN PROGRESS", "OFFER RECEIVED", "HIRED", "REJECTED"]

        while True:
            try:
                status = str(input(status_prompt).strip())
                if not status or int(status) not in range(0, 5):
                    raise ValueError
                else:
                    return status_options[int(status)]
            except ValueError:
                print("\nNot an option!\n")

    ### Enter notes about the job.
    @staticmethod
    @CleanExit.cleanup
    def _new_notes():
        notes = str(input("\nEnter notes regarding this position: ")).strip()
        return notes

    ### Make new Job.
    @staticmethod
    def new_job(args):
        title = GetDetails._new_title(args)
        status = GetDetails._new_status()
        notes = GetDetails._new_notes()

        return Job(date, args.add[0], title, status, notes)

class ConfirmWrite():
    """
    Methods for printing job details and writing to spreadsheet.
    """

    ### Print job details.
    @staticmethod
    def _print_details(job):
        c_len = len(job.company) + 2
        t_len = len(job.title) + 2 if len(job.title) > 10 else len(job_categories[2]) + 2
        n_len = len(job.notes) + 2

        table_header = f"\n{job_categories[0]:<{19}} {job_categories[1]:<{c_len}} {job_categories[2]:<{t_len}} {job_categories[3]:<{16}} {job_categories[4]:<{n_len}}"
        print(table_header)
        print("-"*len(table_header))

        details = [job.date, job.company, job.title, job.status, job.notes]
        description = f"{details[0]:<{19}} {details[1]:<{c_len}} {details[2]:<{t_len}} {details[3]:<{16}} {details[4]:<{n_len}}\n"
        
        set_color(description, details)

        return details

    ### Confirm job details.
    @staticmethod
    @CleanExit.cleanup
    def confirm_new_job(job, parser):
        details = ConfirmWrite._print_details(job)
        master = dict(zip(job_categories, details))

        while True:
            try:
                confirm = input("\nConfirm? [Y/N] ").strip().lower()

                if confirm == options[0]:
                    return master
                elif confirm == options[1]:
                    print("\nEXITING.\n")
                    parser.exit()
                elif confirm not in options:
                    raise ValueError
            except ValueError:
                print("\nNot an option! Try again.\n")

    ### Write job to spreadsheet.
    @staticmethod
    def confirm_write(master):
        ModifyCSV.add_job(master)
        print(Fore.WHITE + Style.BRIGHT + "\nADDED NEW LISTING.\n")

class AddJob():
    """
    Run New methods.
    """

    ### Run add.
    @staticmethod
    def add(args, parser):
        job = GetDetails.new_job(args)
        master = ConfirmWrite().confirm_new_job(job, parser)

        ConfirmWrite.confirm_write(master)
