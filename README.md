# The Struggle

A command-line tool for tracking your job applications. 

Written in Python. Tested with Python 3.8.2.

Run `pip install -r requirements.txt` to install all project dependencies.

## Table of Contents

- [Introduction](#introduction)
- [How it works / Use Cases](#how-it-works-and-use-cases)
- [Stored Attributes](#stored-attributes)
- [Walkthrough](#walkthrough)
- [Releases](#releases)
 
## Introduction
 
I have been applying to *tons* of companies in attempt to secure a job before I graduate from college. The number is so high, it has become hard to keep track of every single place as well as where my application stands in the interview process. I decided to make a Python command-line tool in attempt to help me keep track of my applications and the status of each one. I thought the tool was pretty good, so I put it on here. Thank you for trying this program and I hope it will help you keep track of your applications as well.

## How It Works and Use Cases

This program essentially makes it easier and faster to maintain a spreadsheet of all the jobs applications you want to record.

**Adding a job:** The program will check if there is an existing CSV file of your job applications in the current working directory. If it does not exist, the program will create the file for you and add the first job you record.

**Updating a job:** The program will parse the existing spreadsheet, find the job you want to change, edit the stored details, then rewrite the spreadsheet to reflect the new changes. 

**Deleting a job:** The program will parse the existing spreadsheet, remove the job, then rewrite the spreadsheet. 

**Listing all saved jobs:** The program will parse the spreadsheet and then use [PrettyTable](https://pypi.org/project/PrettyTable/) to print all the jobs you have stored into a readable format within a terminal.

## Stored Attributes

Date is automatically calculated based on Python's [datetime](https://docs.python.org/3/library/datetime.html).

Company Name, Job Title, and Notes are all based on user input.

Application Status has a few options:

|Application Status|
|------------------|
|PENDING|
|IN PROGRESS|
|OFFER RECEIVED|
|HIRED|
|REJECTED|

## Walkthrough

## Releases
