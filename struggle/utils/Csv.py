#===============================================================================
#                                   Exporting
#===============================================================================
import csv

from os import path

from .Global import f_name, job_categories

# # ### Global filename.
# f_name = Global.f_name

class Check():
    """
    Method for checking if the file exists.
    """

    ### Check if spreasheet exists.
    @staticmethod
    def existence():
        return path.exists("job_applications.csv")

class ModifyCSV():
    """
    Methods to modify the jobs spreadsheet.
    """

    ### Write job to spreadsheet.
    @staticmethod
    def add_job(master):
        exists = Check.existence()
        write_mode = "a" if exists else "w"
        with open(f_name, write_mode, encoding = "utf-8") as addjob:
            if write_mode == "w":
                writer = csv.DictWriter(addjob, fieldnames = job_categories)
                writer.writeheader()
                writer.writerow(master) 
            else:
                writer = csv.writer(addjob)
                writer.writerow(list(master.values()))

    ### Rewrite spreadsheet after changes are made (updating or deleting a job 
    ### listing).
    @staticmethod
    def overwrite(master):
        with open(f_name, "w", encoding = "utf-8") as updatejob:
            writer = csv.DictWriter(updatejob, fieldnames = job_categories)
            writer.writeheader()
            writer.writerows(master)

class GetCSV():
    """
    Method for getting jobs from the spreadsheet.
    """

    ### Get all jobs in spreadsheet.
    @staticmethod
    def get_jobs():
        master = []
        with open(f_name, "r") as listing:
            csv_file = csv.reader(listing, delimiter = ",")
            next(csv_file)
            for row in csv_file:
                master.append(row)
        
        return master
