#===============================================================================
#                                   Job Model
#===============================================================================
class Job():
    ### Initialize objects that will be used in class methods
    def __init__(self,date,company,title,status,notes):
        self.date = date
        self.company = company
        self.title = title
        self.status = status
        self.notes = notes
    