#!/usr/bin/python
"""
Created on Friday May 15 22:15:23 2020

Struggle Tracker - A program that helps you track your job applications

@author: Joseph Lai
"""
import src.cli as cli
import src.global_vars as global_vars
from src.model import Job

def main():
    cli.parse_args()


if __name__ == "__main__":
    main()