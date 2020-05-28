#!/usr/bin/python3
"""
Created on Sat May 16 17:24:56 2020

Struggle Tracker - A program that helps you track your job applications

@author: Joseph Lai
"""
import src.cli as cli
import src.global_vars as global_vars
import src.programs.delete as delete
import src.programs.insight as insight
import src.programs.list as listings
import src.programs.new as new
import src.programs.update as update

from src.functions import (delete_functions, insight_functions, list_functions,
                           new_functions, search_functions, update_functions)

def main():
    parser, args = cli.parse_args()
    if args.add:
        ### Add new job to spreadsheet
        new.add_job(args, new_functions, parser)
    if args.update:
        ### Update an existing job in the spreadsheet
        update.update_job(args, update_functions, parser, search_functions)
    if args.delete:
        ### Delete an existing job in the spreadsheet
        delete.delete_job(args, delete_functions, parser, search_functions)
    if args.list:
        ### List all existing jobs in the spreadsheet
        listings.list_jobs(args, list_functions, parser)
    if args.insights:
        ### Return job application insights
        insight.print_insights(args, insight_functions, parser)


if __name__ == "__main__":
    main()
