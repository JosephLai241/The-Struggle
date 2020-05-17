#!/usr/bin/python
"""
Created on Friday May 15 22:15:23 2020

Struggle Tracker - A program that helps you track your job applications

@author: Joseph Lai
"""
import src.cli as cli
import src.global_vars as global_vars
import src.programs.new as new
import src.programs.update as update
from src.functions import new_functions, update_functions

def main():
    parser,args = cli.parse_args()
    if args.new:
        ### Add new job to spreadsheet
        new.add_job(args,new_functions,parser)
    if args.update:
        ### Update an existing job in the spreadsheet
        update.update_job(args,update_functions,parser)
    if args.delete:
        pass
    elif args.list:
        pass


if __name__ == "__main__":
    main()