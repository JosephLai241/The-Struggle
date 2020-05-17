#===============================================================================
#                                   Update Job
#===============================================================================

def update_job(args,update_functions,parser):
    master,matches = update_functions.find_job(args)
    n = update_functions.print_matches(matches)
    selected = update_functions.select_job(matches,n)
    section = update_functions.update_prompt()
    job_listing = update_functions.update_section(section,matches,selected)
    update_functions.list_changes(matches,selected)
    update_functions.confirm_update(parser)
    update_functions.write_changes(job_listing,master)

    