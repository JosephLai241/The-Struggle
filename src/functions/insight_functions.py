#===============================================================================
#                                   Insight Functions
#===============================================================================
from colorama import init, Style
from prettytable import PrettyTable
from .. import global_vars

init(autoreset=True)

status_options = global_vars.status_options
insight_options = global_vars.insight_options

### Check insights arg
def check_insight_arg(args,parser):
    options = ", ".join(insight_options)
    try:
        if args.insights in insight_options:
            return
        else:
            raise ValueError
    except ValueError:
        print("\nNot an insight option!\n")
        print(Style.BRIGHT + "CHOOSE FROM: %s\n" % options)
        parser.exit()


### Calculate insights
class Calculate():
    def __init__(self, master, status_options):
        self.n_jobs = len(master)
        self.count_titles = {0: "\nALL INSIGHTS\n",
                             1: "\nPENDING JOBS\n",
                             2: "\nCURRENTLY IN PROGRESS\n",
                             3: "\nOFFERS RECEIVED\n",
                             4: "\nHIRES\n",
                             5: "\nREJECTIONS\n"
                             }

        self.n_pending, self.n_inprogress, self.n_offers, self.n_hired, self.n_rejected = 0, 0, 0, 0, 0

    def count_all(self, master):
        for job in master:
            if job[3] == status_options[0]:
                self.n_pending += 1
            elif job[3] == status_options[1]:
                self.n_inprogress += 1
            elif job[3] == status_options[2]:
                self.n_offers += 1
            elif job[3] == status_options[3]:
                self.n_hired += 1
            elif job[3] == status_options[4]:
                self.n_rejected += 1

        return len(master)

    def t_row(self,count,n_jobs):
        return "%s out of %s total jobs\n" % (count,n_jobs),"{:.0%} of all jobs".format(count/n_jobs)

    def make_table(self, n_jobs):
        table = PrettyTable()
        headers = [self.count_titles[i].strip("\n") for i in range(1,6)]
        all_counts = [self.n_pending, self.n_inprogress, self.n_offers, self.n_hired, self.n_rejected]
        for index,count in zip([i for i in range(0,5)],all_counts):
            row1,row2 = self.t_row(count,n_jobs)
            table.add_column(headers[index],[row1,row2])        
        
        return table

    def print_table(self,option,table):
        sort_index = 0
        if option == insight_options[0]:
            print(table)
            return
        elif option == insight_options[1]:
            sort_index = 1
        elif option == insight_options[2]:
            sort_index = 2
        elif option == insight_options[3]:
            sort_index = 3
        elif option == insight_options[4]:
            sort_index = 4
        elif option == insight_options[5]:
            sort_index = 5

        print(table.get_string(fields=[self.count_titles[sort_index].strip("\n")]))
