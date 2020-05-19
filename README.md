# The Struggle

A command-line tool for tracking your job applications. 

Written in Python. Tested with Python 3.8.2.

Run `pip install -r requirements.txt` to install all project dependencies.

## Table of Contents

- [Introduction](#introduction)
- [How it works / Use Cases](#how-it-works-and-use-cases)
- [Stored Attributes](#stored-attributes)
- [Read This Before You Run the Program](#read-this-before-you-run-the-program)
- [Walkthrough](#walkthrough)
    - [Adding a Job](#adding-a-job)
    - [Updating or Deleting a Job](#updating-or-deleting-a-job)
    - [Listing All Jobs](#listing-all-jobs)
    - [Showing Job Insights](#showing-job-insights)
- [Releases](#releases)
 
## Introduction
 
I have been applying to *tons* of companies in attempt to secure a job before I graduate from college. The number is so high, it has become hard to keep track of every single place as well as where my application stands in the interview process (if I even get past the resume stage). I decided to make a Python command-line tool in attempt to help me keep track of my applications and the status of each one. I thought the tool was pretty good, so I put it on here. Thank you for trying this program and I hope it will help you keep track of your applications as well.

## How It Works and Use Cases

This program essentially makes it easier and faster to maintain a locally-stored spreadsheet of all the jobs applications you record. A spreadsheet will be created for you on the first run. See [Read This Before You Run the Program](#read-this-before-you-run-the-program) for more information.

**Adding a job:** The program will check if there is an existing CSV file of your job applications in the current working directory. If it does not exist, the program will create the file for you and add the first job you record.

**Updating a job:** The program will parse the existing spreadsheet, find the job you want to change, edit the stored details, then rewrite the spreadsheet to reflect the new changes. 

**Deleting a job:** The program will parse the existing spreadsheet, remove the job, then rewrite the spreadsheet. 

**Listing all saved jobs:** The program will parse the spreadsheet and then print all the jobs you have stored into a readable format within a terminal.

This program utilizes [Colorama](https://pypi.org/project/colorama/) and [PrettyTable](https://pypi.org/project/PrettyTable/) to add color and neatly print job listings in a table within the terminal.

## Stored Attributes

Date is automatically calculated based on Python's [datetime](https://docs.python.org/3/library/datetime.html).

Company Name, Job Title, and Notes are all based on user input.

Application Status has a few options you can choose from:

|Application Status|
|------------------|
|PENDING|
|IN PROGRESS|
|OFFER RECEIVED|
|HIRED|
|REJECTED|

Each Application Status is mapped to a color and will colorize your job listing within a terminal.

## Read This Before You Run the Program

You will have to add a job on the initial run of this program. Adding a job on the initial run will create a CSV spreadsheet titled `job_applications.csv` within the current working directory. All other functionality of the program will not work prior to adding the first job because there is no file to read from. 

**DO NOT** create `job_applications.csv`. The program will create the file for you. Creating an empty `job_applications.csv` before running the `-a` flag will cause issues for you later on. 

## Walkthrough

Use `-h` or `--help` if you forget the arguments or do not want to read this walkthrough.

### Adding a Job

Adding a job at Reddit:

`$ ./track.py -a Reddit`

Use quotes around the company name if it is more than one word:

`$ ./track.py -a "E Corp"`

### Updating or Deleting a Job

Updating an existing job at Reddit:

`$ ./track.py -u Reddit`

Deleting an existing job at E Corp:

`$ ./track.py -d "E Corp"`

***TIP:*** You do not have to type the exact company name when updating or deleting a job. The program uses regex to search for existing job listings. You can just type a letter that is present in the company name to return all job listings with company names that include that letter.

For example, if you have stored job applications from Uber, Hulu, and YouTube and search for just the letter `u`, the program will list all three of those companies. You can then choose which company you would like to update or delete from that list.

### Listing All Jobs

`$ ./track.py -l`

You can sort how jobs are listed within the terminal. The default sort method is by date (descending)

Sort By|Description
----|-----------
date|sort by date (descending) (default)
date_reverse|sort by date (ascending) 
company|sort by company name
title|sort by job title
status|sort by status
notes|sort by notes

Sort job applications by company name:

`$ ./track.py -l company`

### Showing Job Insights

`$ ./track.py -i`

You can display some insights about the jobs that are stored in the spreadsheet. The program will print how many jobs are in each job status as well as its percentage within the spreadsheet. The default insight is for all job statuses.

Sort By|Description
----|-----------
all|display all job application insights (default)
pending|only display insights for job applications that are pending a response
in_progress|only display insights for job applications that are currently in progress
offers|only display insights for job applications with an offer
hired|only display insights for jobs you were hired for
rejected|only display insights for rejected job applications

Only show job applications that are currently in progress:

`$ ./track.py -i in_progress`

## Releases
