#===============================================================================
#                                   Job Model
#===============================================================================
class Job():
    ### Initialize objects that will be used in class methods
    def __init__(self,company,date,position,status):
        self.company = company
        self.position = position
        self.date = date
        self.status = status

    