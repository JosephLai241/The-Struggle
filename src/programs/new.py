#===============================================================================
#                               Add a New Job
#===============================================================================

def add_job(args,new_functions,parser):
    title = new_functions.new_title(args)
    status = new_functions.new_status()
    notes = new_functions.new_notes()
    job = new_functions.new_job(args,status,title,notes)
    master = new_functions.confirm_new_job(job,parser)
    new_functions.confirm_write(master)