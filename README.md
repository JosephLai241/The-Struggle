     __             
    /\ \__     👎   
    \ \ ,_\   ____  
     \ \ \/  /',__\ 
      \ \ \_/\__, `\
       \ \__\/\____/
        \/__/\/___/... The Struggle

![GitHub top language](https://img.shields.io/github/languages/top/JosephLai241/the-struggle?logo=rust&style=for-the-badge)
[![Travis (.org)](https://img.shields.io/travis/JosephLai241/The-Struggle?logo=travis&style=for-the-badge)][Travis CI Build Status]
![GitHub release (latest by date)](https://img.shields.io/github/v/release/JosephLai241/The-Struggle?style=for-the-badge)

A command-line tool for tracking your job applications. 

Written in Rust. Tested with Rust 1.43.1.

# Table of Contents

- [Introduction](#introduction)
- [Installation](#installation)
- [Why Rust?](#why-rust)
- [How it works / Use Cases](#how-it-works-and-use-cases)
- [Stored Attributes](#stored-attributes)
- [Read This Before You Run the Program](#read-this-before-you-run-the-program)
- [Walkthrough](#walkthrough)
    - [Adding a Job](#adding-a-job)
    - [Updating or Deleting a Job](#updating-or-deleting-a-job)
    - [Listing Stored Jobs](#listing-stored-jobs)
    - [Showing Job Insights](#showing-job-insights)
- [Releases](#releases)
 
# Introduction
 
I have been applying to *tons* of companies in attempt to secure a job before I graduate from college. The number is so high, it has become hard to keep track of every single place as well as where my application stands in the interview process (if I even get past the resume stage). I decided to make a command-line tool in attempt to help me keep track of my applications and the status of each one. I thought the tool was pretty useful so I put it on here. Thank you for trying this program and I hope it will help you keep track of your applications as well.

# Installation

First, you will need a [Rust installation][Rust Official Site] in order to compile The Struggle.

Then run these commands to build The Struggle:

```bash
$ git clone https://www.github.com/JosephLai241/The-Struggle
$ cd The-Struggle
$ cargo build --release
```

To check if The Struggle built correctly:

```bash
$ ./target/release/ts -V
```

You can then move the `ts` binary to another directory so you do not have to type that path to run The Struggle. Check if the binary was moved correctly:

```bash
$ mv target/release/ts /some/directory/
$ cd /some/directory
$ ./ts -V
```

The complimentary spreadsheet `job_applications.csv` will be created in whichever directory you move the `ts` binary to.

# Why Rust?

I chose Rust because one of my best friends [Luke Schenk][Luke] told me Rust is amazing and encouraged me to try it. Also, I have finished too many projects in Python and wanted to add some variety to my portfolio, so Rust, a low-level programming language, was the way to go.

# How It Works and Use Cases

This program essentially makes it easier and faster to maintain a locally-stored spreadsheet of all the jobs applications you record. A spreadsheet will be created for you on the first run. See [Read This Before You Run the Program](#read-this-before-you-run-the-program) for more information.

This program utilizes [ANSI Terminal][ANSI Terminal] and [PrettyTable][PrettyTable] to add color and neatly print job listings in a table within the terminal.

## Adding a Job 

The program will check if there is an existing CSV file of your job applications in the current working directory. If it does not exist, the program will create the file for you and add the first job you record. If it does exist, the program will append the job to the spreadsheet.

## Updating a Job

The program will parse the existing spreadsheet, find the job you want to change, edit the stored details, then rewrite the spreadsheet to reflect the new changes. 

## Deleting a Job 

The program will parse the existing spreadsheet, remove the job, then rewrite the spreadsheet. 

## Listing All Saved Jobs 

The program will parse the spreadsheet and then print all the jobs you have stored into a readable format within a terminal.

## Job Application Insights 

The program will parse the spreadsheet, count how many jobs are under each job status, and calculate the percentage of each job status.

# Stored Attributes

`DATE ADDED` is automatically calculated based on Rust's [chrono][chrono].

`COMPANY`, `JOB TITLE`, and `NOTES` are all based on user input.

`STATUS` has a few options you can choose from. Each status is mapped to a color and will colorize your job listing within the terminal:

| Application Status | Color   |
|--------------------|---------|
| PENDING            | Blue    |
| IN PROGRESS        | Yellow  |
| OFFER RECEIVED     | Magenta |
| HIRED              | Green   |
| REJECTED           | Red     |

# Read This Before You Run the Program

You have to add a job on the initial run of this program. Adding a job on the initial run will create a CSV spreadsheet titled `job_applications.csv` within the current working directory. All other functionality of the program will not work prior to adding the first job because there is no file to read from. 

**DO NOT** create `job_applications.csv` manually. The program will create the file for you. Creating an empty `job_applications.csv` before running the `-a` flag will cause issues for you later on. 

# Walkthrough

Use `-h` or `--help` if you forget the arguments or do not want to read this walkthrough.

## Adding a Job

As stated before, **this has to be the first command you run.** Doing so will create `job_applications.csv` in your current working directory.

`$ ./ts -a COMPANY_NAME`

***NOTE:*** Use quotes around the company name if it is more than one word or contains special terminal characters. For example, `&` is used to run a command asynchronously (running in the background) in a Bash terminal. Running `$ ./ts -a H&M` will cause problems for you if you do not wrap `H&M` in quotes.

You will then enter the job title at the company, select the status of the job listing, then enter any notes on the job listing. You can just enter through the notes prompt to leave it blank.

The job listing will be written to `job_applications.csv` after you confirm.

## Updating or Deleting a Job

***TIP:*** You do not have to type the exact company name when updating or deleting a job. The program uses regex to search for existing job listings. You can just type a letter or pattern of letters present in the company name. This will return all job listings with company names that include that letter or pattern.

For example, if you have stored job applications from Uber, Hulu, and YouTube and search for just the letter `u`, the program will list all three of those companies. You can then choose which company you would like to update or delete from that list.

**Updating an existing job**

`$ ./ts -u COMPANY_NAME`

Use the `NUMBER` in the far left column to pick the job you want to delete:

Choose the section you want to update. You can modify the job's company name, job title, application status, or notes.

The job listing will be updated in `job_applications.csv` after you confirm.

**Deleting an existing job**

`$ ./ts -d COMPANY_NAME`

Identical to updating, you can just enter a letter or pattern in the company name and use the `NUMBER` in the far left column to choose the job you want to delete.

The job listing will then be deleted from `job_applications.csv` after you confirm.

## Listing Stored Jobs

`$ ./ts -l`

Job applications are sorted by date (descending) and are colorized based on the application status. Here is a table of how each is colorized:

| Job Status     | Color   |
|----------------|---------|
| PENDING        | Blue    |
| IN PROGRESS    | Yellow  |
| OFFER RECEIVED | Magenta |
| HIRED          | Green   |
| REJECTED       | Red     |

## Showing Job Insights

`$ ./ts -i`

You can display some insights about the jobs that are stored in the spreadsheet. The program will print how many jobs are listed under each job status as well as its percentage within the spreadsheet.

Each cell is also colorized based on the table described in the section above.

# Releases

- **May 18, 2020:** The Struggle v1.0 (Python). Features include:
    - Add a new job
    - Update an existing job
    - Delete an existing job
    - List all stored jobs
    - Print job application insights

- **TBD:** The Struggle v2.0 (Rust).
    - Unfortunately had to remove the optional list sorting method since Rust's PrettyTable does not currently have some kind of `sort()` method.
    - Insights will now only display all job status insights rather than including options to only display a specific status.
    - Deploying 2.0 with Travis CI.

<!-- BADGES -->
[Travis CI Build Status]: https://travis-ci.org/github/JosephLai241/The-Struggle

<!-- A BROTHER -->
[Luke]: https://github.com/LukeDSchenk

<!-- RUST LINKS -->
[Rust Official Site]: https://www.rust-lang.org/

[ANSI Terminal]: https://docs.rs/ansi_term/0.12.1/ansi_term/
[chrono]: https://docs.rs/chrono/0.4.11/chrono/
[PrettyTable]: https://docs.rs/prettytable-rs/0.8.0/prettytable/
