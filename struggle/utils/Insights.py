#===============================================================================
#                             Displaying Insights
#===============================================================================
from colorama import init, Style
from prettytable import PrettyTable

from .Csv import GetCSV
from .Global import status_options

### Automate sending reset sequences to turn off color changes at the end of 
### every print.
init(autoreset = True)

### Global variables.
insight_options = ["all", "pending", "in_progress", "offers", "hired", "rejected"]

class CheckArgs():
    """
    Method to check insights args.
    """

    ### Check insights arg.
    @staticmethod
    def check_insight_arg(args, parser):
        try:
            if args.insights in insight_options:
                return
            else:
                raise ValueError
        except ValueError:
            print("\nNot an insight option!\n")
            print(Style.BRIGHT + "CHOOSE FROM: %s\n" % ", ".join(insight_options))
            
            parser.exit()

class Calculate():
    """
    Methods to calculate job insights.
    """

    ### Initialize objects that will be used in class methods.
    def __init__(self, master, status_options):
        self.n_jobs = len(master)
        self.count_titles = {
            0: "\nALL INSIGHTS\n",
            1: "\nPENDING JOBS\n",
            2: "\nCURRENTLY IN PROGRESS\n",
            3: "\nOFFERS RECEIVED\n",
            4: "\nHIRES\n",
            5: "\nREJECTIONS\n"
        }

        self.n_pending, self.n_inprogress, self.n_offers, self.n_hired, self.n_rejected = \
            0, 0, 0, 0, 0

    ### Get the total count of jobs for each status.
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

    ### Get the ratio of jobs in a status to total number of tracked jobs.
    def _ratio_row(self, count, n_jobs):
        return "%s out of %s total jobs\n" % \
            (count, n_jobs),"{:.0%} of all jobs".format(count/n_jobs)

    ### Make the PrettyTable of job insights.
    def make_table(self, n_jobs):
        insights_table = PrettyTable()
        headers = [self.count_titles[i].strip("\n") for i in range(1,6)]
        
        all_counts = [self.n_pending, self.n_inprogress, self.n_offers, 
            self.n_hired, self.n_rejected]

        for index, count in enumerate(all_counts):
            row1, row2 = self._ratio_row(count, n_jobs)
            insights_table.add_column(headers[index], [row1, row2])           
        
        return insights_table

    ### Print the insights PrettyTable.
    def print_table(self, option, table):
        sort_index = insight_options.index(option)
        if sort_index == 0:
            print(table)
            return

        print(table.get_string(fields = [self.count_titles[sort_index].strip("\n")]))

class ShowInsights():
    """
    Run Insights methods.
    """

    ### Run insights.
    @staticmethod
    def insights(args, parser):
        CheckArgs.check_insight_arg(args, parser)

        master = GetCSV.get_jobs()

        insights = Calculate(master, status_options)
        n_jobs = insights.count_all(master)
        table = insights.make_table(n_jobs)

        insights.print_table(args.insights, table)
