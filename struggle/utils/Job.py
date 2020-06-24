#===============================================================================
#                                   Job Model
#===============================================================================

class Job():
    """
    Schema for a Job object.
    """

    ### Creating a Job object.
    def __init__(self, date, company, title, status, notes):
        self.date = date
        self.company = company
        self.title = title
        self.status = status
        self.notes = notes
    