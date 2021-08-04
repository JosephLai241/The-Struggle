     __             
    /\ \__     👎   
    \ \ ,_\   ____  
     \ \ \/  /',__\ 
      \ \ \_/\__, `\
       \ \__\/\____/
        \/__/\/___/... The Struggle

![GitHub top language](https://img.shields.io/github/languages/top/JosephLai241/The-Struggle?color=yellow&logo=Rust)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/JosephLai241/The-Struggle/Rust?logo=github)][Github Actions]
![GitHub release (latest by date)](https://img.shields.io/github/v/release/JosephLai241/The-Struggle)

A command-line tool that helps you track your job applications. 

Written in Rust. Tested with Rust 1.54.0.

This program is also available in Python, located on the [Python branch][Python branch].

# Table of Contents

* [Introduction](#introduction)
    + [What Does It Do?](#what-does-it-do)
    + [Why?](#why)
* [Installation](#installation)
    + [Compile From Source](#compile-from-source)
    + [Or Download a Binary](#or-download-a-binary)
* [Stored Attributes](#stored-attributes)
* [Read This Before You Run the Program](#read-this-before-you-run-the-program)
* [Walkthrough](#walkthrough)
    + [Adding a Job](#adding-a-job)
    + [Updating or Deleting a Job](#updating-or-deleting-a-job)
    + [Listing Stored Jobs](#listing-stored-jobs)
    + [Display Job Insights](#showing-job-insights)
* [Releases](#releases)
* [Why Rust?](#why-rust)
 
# Introduction

## What Does It Do?

`The-Struggle` performs CRUD operations on a local spreadsheet and simplifies the process of tracking your applications. It is very fast because:

1) It is written in Rust - it is so fast it feels as if the output was hardcoded.
2) Doing these operations from the terminal is much faster than opening up a spreadsheet and manually editing it.

Its features include:

* Add, update, or delete job applications from the spreadsheet
* Display tracked job applications, using [ANSI Terminal][ANSI Terminal] and [PrettyTable][PrettyTable] to color-code and neatly display applications in a table within your terminal.
* Display insights for tracked applications, such as:
    + The total number of tracked applications
    + The total number and percentage of applications:
        * That are pending a response
        * That are currently in progress
        * Where you have received an offer
        * Where you have been rejected from
        * Where you have been hired at

## Why?

I graduated from college in June 2020 and have been applying to ***tons*** of companies in attempt to kick off my developer career. It has been very difficult for me to break into the industry because I am a self-taught developer (no CS degree) and I graduated in the midst of the COVID-19 pandemic.

The number of applications that I have sent out is so high, it has become hard to keep track of every company as well as where my application stands in the interview process (if I even pass the resume stage). I decided to make a command-line tool in attempt to help me keep track of my applications and the status of each one. I thought the tool was pretty useful so I put it on here. 

Thank you for trying this program and I hope it will help you keep track of your applications as well. It is tough out there, man.

# Installation

> ***NOTE:*** This program initializes and reads from files in your current working directory. Run `The-Struggle` in a directory in which you would like all your records to be stored.
> 
> The complimentary spreadsheet `job_applications.csv` will be created in whichever directory you move the `ts` binary to.

## Compile From Source

First, you will need a [Rust installation][Rust Official Site] in order to compile The Struggle.

Then run these commands to build The Struggle:

```
$ git clone https://www.github.com/JosephLai241/The-Struggle
$ cd The-Struggle
$ cargo build --release
```

To check if The Struggle built correctly:

```
$ ./target/release/ts -V
```

You can then move the `ts` binary to another directory so you do not have to type that path to run The Struggle. Check if the binary was moved correctly:

```
$ mv target/release/ts /some/directory/
$ cd /some/directory
$ ./ts -V
```

## Or Download a Binary

If you do not want to compile `The-Struggle`, you can also download a binary attached to a release in the [Releases][Releases] section.

# Stored Attributes

Each application will store the following information:

* `DATE ADDED`
* `COMPANY`
* `JOB TITLE`
* `STATUS`
* `NOTES`

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

You have to add a job on the initial run of this program. Adding a job on the initial run will create a CSV spreadsheet titled `job_applications.csv` within your current working directory. All other functionality of the program will not work prior to adding the first job because there is no valid spreadsheet to read from. 

**DO NOT** create `job_applications.csv` manually. The program will create the file for you. Creating an empty `job_applications.csv` before running the `-a` flag will cause issues for you later on. 

# Walkthrough

Use `-h` or `--help` if you forget the arguments or do not want to read this walkthrough.

## Adding a Job

As stated before, **this has to be the first command you run.** Doing so will create `job_applications.csv` in your current working directory.

```
$ ./ts -a COMPANY_NAME
```

> ***NOTE:*** Use quotes around the company name if it is more than one word or contains special terminal characters. For example, `&` is used to run a command asynchronously (running in the background) in a Bash terminal. Running `$ ./ts -a H&M` will cause problems for you if you do not wrap `H&M` in quotes.

You will then enter the job title at the company, select the status of the job listing, then enter any notes on the job listing. You can just enter through the notes prompt to leave it blank.

The job listing will be written to `job_applications.csv` after you confirm.

## Updating or Deleting a Job

> ***TIP:*** You do not have to type the exact company name when updating or deleting a job. The program uses regex to search for existing job listings. You can just type a letter or pattern of letters present in the company name. This will return all job listings with company names that include that letter or pattern.
>
> For example, if you have stored job applications from Uber, Hulu, and YouTube and search for just the letter `u`, the program will list all three of those companies. You can then choose which company you would like to update or delete from that list.

**Updating an existing job**

```
$ ./ts -u COMPANY_NAME
```

Use the `NUMBER` in the far left column to pick the job you want to delete:

Choose the section you want to update. You can modify the job's company name, job title, application status, or notes.

The job listing will be updated in `job_applications.csv` after you confirm.

**Deleting an existing job**

```
$ ./ts -d COMPANY_NAME
```

Identical to updating, you can just enter a letter or pattern in the company name and use the `NUMBER` in the far left column to choose the job you want to delete.

The job listing will then be deleted from `job_applications.csv` after you confirm.

## Listing Stored Jobs

```
$ ./ts -l
```

Job applications are sorted by date (descending) and are colorized based on the application status. See the application status and color table in the [Stored Attributes](#stored-attributes) section for details.

## Display Job Insights

```
$ ./ts -i
```

You can display some insights about the jobs that are stored in the spreadsheet. The program will print how many jobs are listed under each job status as well as its percentage within the spreadsheet.

Each cell is also colorized based on the table described in the [Stored Attributes](#stored-attributes) section.

# Releases

* **May 18, 2020:** v1.0.0 (Python Edition). Features include:
    + Add a new job
    + Update an existing job
    + Delete an existing job
    + List all stored jobs
    + Print job application insights

* **July 4, 2020:** v2.0.0 (Rust Edition).
    + Unfortunately had to remove the optional list sorting method since Rust's PrettyTable currently does not have some kind of `sort()` method.
    + Insights will now only display all job status insights rather than including options to only display a specific status. I figure most people would not bother using any of the other options anyways.
    + Deploying with Travis CI.
    + *Scary fast*.
* **August 4, 2021:** v2.0.1.
    + Prompts would end with a newline character, so user-entered data would appear on a new line underneath the prompt. This release modifies the prompts so that they are inline with user-entered data.

# Why Rust?

I chose Rust because one of my best friends [Luke Schenk][Luke] told me Rust is amazing and encouraged me to try it. Also, I have finished too many projects in Python and wanted to add some variety to my portfolio. Rust and Python are two very different languages, so Rust is just the perfect choice for me since I am looking to get better at programming in a language besides Python.

<!-- BADGES -->
[Github Actions]: https://github.com/JosephLai241/The-Struggle/actions?query=workflow%3ARust

<!-- REPO LINKS -->
[Python branch]: https://github.com/JosephLai241/The-Struggle/tree/python
[Releases]: https://github.com/JosephLai241/The-Struggle/releases

<!-- A BROTHER -->
[Luke]: https://github.com/LukeDSchenk

<!-- RUST LINKS -->
[Rust Official Site]: https://www.rust-lang.org/

[ANSI Terminal]: https://docs.rs/ansi_term/0.12.1/ansi_term/
[chrono]: https://docs.rs/chrono/0.4.11/chrono/
[PrettyTable]: https://docs.rs/prettytable-rs/0.8.0/prettytable/
