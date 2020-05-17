#===============================================================================
#                               Export Functions
#===============================================================================
import csv
import re
from .. import global_vars, model

job_categories = global_vars.job_categories

### Regex search class
class reSearch():
    def __init__(self,args):
        self.args = args
        self.matches = []
    
    def search(self,args,line,row):
        if re.search("%s" % str(args[0]),"%s" % row[1],re.I):
            match = [model.Job(row[0],row[1],row[2],row[3],row[4]),line]
            self.matches.append(match)

    def return_matches(self):
        return self.matches

### Search spreadsheet for existing job listings. Create master list of all listings
def find_job(args):
    re_search = reSearch(args)
    master = []
    with open(global_vars.f_name,"r") as spreadsheet:
        csv_file = csv.reader(spreadsheet,delimiter=",")
        line = 0
        next(csv_file)
        for row in csv_file:
            if args.update:
                re_search.search(args.update,line,row)
            elif args.delete:
                re_search.search(args.delete,line,row)
            job = dict(zip(job_categories,row))
            master.append(job)
            line += 1
    
    matches = re_search.return_matches()
    return master,matches

### Set formatting for header
def set_print_format(matches):
    c_len = 0
    t_len = 0
    n_len = 0
    for match in matches:
        if len(match[0].company) > c_len:
            c_len = len(match[0].company) + 2
        if len(match[0].title) > t_len:
            t_len = len(match[0].title) + 2 if len(match[0].title) > 10 else len(job_categories[2]) + 2
        if len(match[0].notes) > n_len:
            n_len = len(match[0].notes) + 2

    return c_len,t_len,n_len

### Print all matches
def print_matches(matches):
    c_len,t_len,n_len = set_print_format(matches)
    found_header = f"\n{'='*61} EXISTING JOB LISTINGS {'='*62}"
    table_header = f"\nNumber {job_categories[0]:<{19}} {job_categories[1]:<{c_len}} {job_categories[2]:<{t_len}} {job_categories[3]:<{16}} {job_categories[4]:<{n_len}}"
    print(found_header)
    print(table_header)
    print("-"*len(table_header))
    
    n = 0
    for match in matches:
        details = [match[0].date,match[0].company,match[0].title,match[0].status,match[0].notes]
        description = f"{n:<6} {details[0]:<{19}} {details[1]:<{c_len}} {details[2]:<{t_len}} {details[3]:<{16}} {details[4]:<{n_len}}\n"
        global_vars.set_color(description,details,c_len,t_len,n_len)

        n += 1

    return n