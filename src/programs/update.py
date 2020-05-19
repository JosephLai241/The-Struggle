#===============================================================================
#                                   Update Job
#===============================================================================

def update_job(args,update_functions,parser,search_functions):
    master,matches = search_functions.find_job(args,parser)
    n = search_functions.print_matches(matches)
    selected = update_functions.select_job(matches,n)
    section = update_functions.update_prompt()
    job_listing = update_functions.update_section(section,matches,selected)
    search_functions.list_changes(args,matches,selected)
    search_functions.confirm_changes(parser)
    update_functions.write_changes(job_listing,master)