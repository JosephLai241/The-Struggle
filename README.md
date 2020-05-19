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
    - [Listing Stored Jobs](#listing-stored-jobs)
    - [Showing Job Insights](#showing-job-insights)
- [Releases](#releases)
 
## Introduction
 
I have been applying to *tons* of companies in attempt to secure a job before I graduate from college. The number is so high, it has become hard to keep track of every single place as well as where my application stands in the interview process (if I even get past the resume stage). I decided to make a Python command-line tool in attempt to help me keep track of my applications and the status of each one. I thought the tool was pretty useful so I put it on here. Thank you for trying this program and I hope it will help you keep track of your applications as well.

## How It Works and Use Cases

This program essentially makes it easier and faster to maintain a locally-stored spreadsheet of all the jobs applications you record. A spreadsheet will be created for you on the first run. See [Read This Before You Run the Program](#read-this-before-you-run-the-program) for more information.

**Adding a job:** The program will check if there is an existing CSV file of your job applications in the current working directory. If it does not exist, the program will create the file for you and add the first job you record. If it does exist, the program will append the job to the spreadsheet.

**Updating a job:** The program will parse the existing spreadsheet, find the job you want to change, edit the stored details, then rewrite the spreadsheet to reflect the new changes. 

**Deleting a job:** The program will parse the existing spreadsheet, remove the job, then rewrite the spreadsheet. 

**Listing all saved jobs:** The program will parse the spreadsheet and then print all the jobs you have stored into a readable format within a terminal.

**Job application insights:** The program will parse the spreadsheet, count how many jobs are under each job status, and calculate the percentage of each job status.

This program utilizes [Colorama](https://pypi.org/project/colorama/) and [PrettyTable](https://pypi.org/project/PrettyTable/) to add color and neatly print job listings in a table within the terminal.

## Stored Attributes

`Date` is automatically calculated based on Python's [datetime](https://docs.python.org/3/library/datetime.html).

`Company Name`, `Job Title`, and `Notes` are all based on user input.

`Application Status` has a few options you can choose from:

|Application Status|
|------------------|
|PENDING|
|IN PROGRESS|
|OFFER RECEIVED|
|HIRED|
|REJECTED|

Each status is mapped to a color and will colorize your job listing within a terminal.

## Read This Before You Run the Program

You have to add a job on the initial run of this program. Adding a job on the initial run will create a CSV spreadsheet titled `job_applications.csv` within the current working directory. All other functionality of the program will not work prior to adding the first job because there is no file to read from. 

**DO NOT** create `job_applications.csv` manually. The program will create the file for you. Creating an empty `job_applications.csv` before running the `-a` flag will cause issues for you later on. 

## Walkthrough

Use `-h` or `--help` if you forget the arguments or do not want to read this walkthrough.

### Adding a Job

As stated before, **this has to be the first command you run.** Doing so will create `job_applications.csv` in your current working directory.

`$ ./track.py -a COMPANY_NAME`

***NOTE:*** Use quotes around the company name if it is more than one word or contains special terminal characters. For example, `&` is used to run a command asynchronously (running in the background) in a Bash terminal. Running `$ ./track.py -a H&M` will cause problems for you if you do not wrap `H&M` in quotes.

Enter the job title at the company:

![Add new company to track](https://github.com/JosephLai241/The-Struggle/blob/master/.github/screenshots/add_1.png)

Then select the status of the job listing:

![Add job status](https://github.com/JosephLai241/The-Struggle/blob/master/.github/screenshots/add_2.png)

If you do not have any notes on the job listing, you can just enter through the notes prompt:

![Add notes then submit](https://github.com/JosephLai241/The-Struggle/blob/master/.github/screenshots/add_3.png)

The job listing will be written to `job_applications.csv` after you confirm.

### Updating or Deleting a Job

***TIP:*** You do not have to type the exact company name when updating or deleting a job. The program uses regex to search for existing job listings. You can just type a letter or pattern of letters present in the company name. This will return all job listings with company names that include that letter or pattern.

For example, if you have stored job applications from Uber, Hulu, and YouTube and search for just the letter `u`, the program will list all three of those companies. You can then choose which company you would like to update or delete from that list.

**Updating an existing job**

`$ ./track.py -u COMPANY_NAME`

Use the number in the far left column to pick the job you want to delete:

![Updating 1](https://github.com/JosephLai241/The-Struggle/blob/master/.github/screenshots/update_1.png)

Choose the section you want to update:

![Updating 2](https://github.com/JosephLai241/The-Struggle/blob/master/.github/screenshots/update_2.png)

I chose to update the application status. This option will display another menu to update the job status. All other options will prompt you to type in a new company name, job title, or notes.

![Updating 3](https://github.com/JosephLai241/The-Struggle/blob/master/.github/screenshots/update_3.png)

The job listing will be updated in `job_applications.csv` after you confirm.

**Deleting an existing job**

`$ ./track.py -d COMPANY_NAME`

Identical to updating, you can just enter a letter or pattern in the company name and use the number in the far left column to choose the job you want to delete.

![Deleting a job](https://github.com/JosephLai241/The-Struggle/blob/master/.github/screenshots/delete_1.png)

The job listing will be deleted from `job_applications.csv` after you confirm.

### Listing Stored Jobs

`$ ./track.py -l OPTIONAL_SORT_METHOD`

Job applications are sorted by date (descending) if no optional sort methods are provided:

![Table of job applications default](https://github.com/JosephLai241/The-Struggle/blob/master/.github/screenshots/list_1.png)

This is a table of sort options and the changes that will be reflected within a terminal:

Sort By|Description
----|-----------
date|sort by date (descending) (default)
date_reverse|sort by date (ascending) 
company|sort by company name
title|sort by job title
status|sort by status
notes|sort by notes

Sort job applications by job status:

![Table sorted](https://github.com/JosephLai241/The-Struggle/blob/master/.github/screenshots/list_2.png)

The program will print the acceptable options if you enter an invalid sort method:

![Invalid sort method](https://github.com/JosephLai241/The-Struggle/blob/master/.github/screenshots/list_3.png)

### Showing Job Insights

`$ ./track.py -i OPTIONAL_DISPLAY_METHOD`

You can display some insights about the jobs that are stored in the spreadsheet. The program will print how many jobs are listed under each job status as well as its percentage within the spreadsheet. The default insight is for all job statuses.

![Insights default](https://github.com/JosephLai241/The-Struggle/blob/master/.github/screenshots/insights_1.png)

This is a table of display options and the changes that will be reflected within a terminal:

Display|Description
----|-----------
all|display all job application insights (default)
pending|only display insights for job applications that are pending a response
in_progress|only display insights for job applications that are currently in progress
offers|only display insights for job applications with an offer
hired|only display insights for jobs you were hired for
rejected|only display insights for rejected job applications

Only show job applications that are pending a response:

![Insights pending](https://github.com/JosephLai241/The-Struggle/blob/master/.github/screenshots/insights_2.png)

The program will print the acceptable options if you enter an invalid display method:

![Invalid display method](https://github.com/JosephLai241/The-Struggle/blob/master/.github/screenshots/insights_3.png)

## Releases

- **May 18, 2020:** The Struggle v1.0 completed. Features include:
    - Add a new job
    - Update an existing job
    - Delete an existing job
    - List all stored jobs
    - Print job application insights
