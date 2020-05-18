#===============================================================================
#                               Delete a Job
#===============================================================================

def delete_job(args,delete_functions,parser,search_functions):
    master,matches = search_functions.find_job(args)
    n = search_functions.print_matches(matches)
    selected = delete_functions.select_job(matches,n)
    search_functions.list_changes(args,matches,selected)
    search_functions.confirm_changes(parser)
    delete_functions.delete_listing(master,selected)