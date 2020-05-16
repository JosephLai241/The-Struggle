#===============================================================================
#                       New Job Profile Functions
#===============================================================================
from . import global_vars

### Enter a job to track
def job_title(parser):
    while True:
        try:
            title = str(input("What is the title of the position you are applying for at %s? " % parser.new)).strip()
            if not title:
                raise ValueError
            
            while True:
                try:
                    confirm = input("\nConfirm selection? [Y/N] ").strip().lower()
                    if confirm == global_vars.options[0]:
                        return title
                except ValueError:
                    print("Not an option! Try again.")
        except ValueError:
            print("No job title was entered!")