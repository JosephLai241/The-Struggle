```
                            _
      .::::::::::.        -(_)====u         .::::::::::.
    .::::''''''::::.                      .::::''''''::::.
  .:::'          `::::....          ....::::'          `:::.
 .::'             `:::::::|        |:::::::'             `::.
.::|               |::::::|_ ___ __|::::::|               |::.
`--'               |::::::|_()__()_|::::::|               `--'
 :::               |::-o::|        |::o-::|               :::
 `::.             .|::::::|        |::::::|.             .::'
  `:::.          .::\-----'        `-----/::.          .:::'
    `::::......::::'                      `::::......::::'
      `::::::::::'                          `::::::::::'
                           fetters
```

[![GitHub Workflow Status](https://github.com/JosephLai241/fetters/actions/workflows/rust.yml/badge.svg)](https://github.com/JosephLai241/fetters/actions/workflows/rust.yml)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/JosephLai241/fetters)

# Table of Contents

- [Introduction](#introduction)
  - [What Does It Do?](#what-does-it-do)
- [Installation](#installation)
  - [`cargo install`](#cargo-install)
  - [Compile From Source](#compile-from-source)
  - [Or Download a Binary](#or-download-a-binary)
- [Stored Attributes](#stored-attributes)
- [Job Sprints](#job-sprints)
- [Walkthrough](#walkthrough)
  - [Managing Job Sprints](#managing-job-sprints)
    - [Creating a New Sprint](#creating-a-new-sprint)
    - [Show Current Job Sprint](#show-current-job-sprint)
    - [Show All Job Sprints](#show-all-job-sprints)
    - [Switch to a Different Sprint](#switch-to-a-different-sprint)
  - [Adding a Job](#adding-a-job)
  - [Updating or Deleting a Job](#updating-or-deleting-a-job)
  - [Listing/Searching Jobs](#listingsearching-jobs)
  - [Display Job Insights](#showing-job-insights)
  - [Opening Links](#opening-links)
- [Releases](#releases)
- [Why Rust?](#why-rust)

# Introduction

> [!IMPORTANT]
> **Requires SQLite v3.35.0+ installed on your system.**

`fetters` is a command-line tool that helps you track your job applications all in one place. The process of adding, updating, searching, or deleting job applications is very simple and fast.

You can create different "job sprints" (phases when you are looking for a new job) with this program and track the number of applications you sent during each period, providing a singular tool and space for you to track them.

## What Does It Do?

This program stores job applications, job titles, and sprints into a local SQLite database. This SQLite database is automatically created for you on the initial run and is stored in the project directory on your machine (see the documentation for [`directories::ProjectDirs::data_dir()`][projectdirs documentation] for the exact path on your platform).

This program enables you to:

- Add, update, and delete job applications from the database.
- List/search for tracked job applications. Each entry is color-coded based on the application status.
- Display application insights (ie. How many applications are in progress, how many have been rejected, how many are in pending status, etc.).
- Group job applications by sprints.

# Installation

## `cargo install`

You can run `cargo install` to install this on your machine:

```
cargo install fetters
```

## Compile From Source

You can compile this program from source with the following commands:

```
$ git clone https://www.github.com/JosephLai241/fetters
$ cd fetters
$ cargo build --release
```

To check if it built correctly:

```
$ ./target/release/fetters -V
```

You can then move the `fetters` binary to another directory so you do not have to type that path to run it. Check if the binary was moved correctly:

```
$ mv target/release/fetters /some/directory/
$ cd /some/directory
$ ./fetters -V
```

## Or Download a Binary

If you do not want to compile `fetters`, you can also download a binary attached to a release in the [Releases] section.

# Stored Attributes

Each record stores the following fields:

- Created timestamp (`YYYY-MM-DD HH:MM:SS`)
- Company name
- Job title
- Application status
- [Optional] Link to the application
- [Optional] Notes
- Job Sprint

The job status is color-coded in the table. Here is a table mapping each status to its color:

| Status             | Color     |
| ------------------ | --------- |
| GHOSTED            | Gray      |
| HIRED              | Green     |
| IN PROGRESS        | Yellow    |
| NOT HIRING ANYMORE | Dark Gray |
| OFFER RECEIVED     | Magenta   |
| PENDING            | Blue      |
| REJECTED           | Red       |

# Job Sprints

A job sprint is a period of time when you are actively submitting job applications. `fetters` allows you to organize your job applications into sprints so that it is easy to tell during which time period an application was submitted.

See the [Managing Job Sprints](#managing-job-sprints) section for more details.

# Walkthrough

## Managing Job Sprints

You can configure different job sprints to group job applications based on periods of time in which you are actively submitting job applications.

**A new job sprint will be created for you on the initial run. You do not have to worry about managing sprints if you don't plan on grouping your job applications by sprint.**

Job sprints are labeled with the date on which they are created (`YYYY-MM-DD`) by default but can be overridden with a custom name when creating a new sprint.

### Creating a New Sprint

The default name for a sprint is the current date (`YYYY-MM-DD`). You can optionally override the name of the sprint by providing the `-n/--name` flag.

Run the following command to create a new sprint:

```
fetters sprint new (-n <NAME>)
```

An error will be raised if you try to create a new sprint but there is already another sprint with an identical name.

### Show Current Job Sprint

Run the following command to show the current job sprint:

```
fetters sprint current
```

This will display a table containing the sprint name, start date, end date (if applicable), and the total number of applications in the sprint.

### Show All Job Sprints

Run the following command to show all job sprints:

```
fetters sprint show-all
```

Like the `current` subcommand, this will display a table containing all sprints, start dates, end dates (if applicable), and the total number of applications tracked in each sprint.

### Switch to a Different Sprint

Run the following command to switch to or set a different sprint:

```
fetters sprint set
```

A select menu will appear and the selected sprint will be used to track applications until you decide to switch to a different sprint or create a new one.

## Adding a Job

> ![NOTE]
>
> If you are utilizing [different sprints](#managing-job-sprints), the job application will be added to your current sprint.

Run the following command to track a new job application:

```
fetters add <COMPANY_NAME>
```

> [!TIP]
> Use quotes around the company name if it is more than one word or contains special terminal characters. For example, `&` is used to run a command asynchronously (running in the background) in a Bash terminal. Running `fetters add H&M` will cause problems for you if you do not wrap `H&M` in quotes.

A series of `inquire` prompts will show to set the job title, application status, link, and any notes.

## Updating or Deleting a Job

> ![NOTE]
>
> If you are utilizing [different sprints](#managing-job-sprints), these subcommands will search for jobs within your current sprint that match your query.

Run the following commands to update or delete a tracked job application:

```
fetters update
fetters delete
```

These commands support querying all stored attributes. Here is an example using all of the query options:

```
fetters update [OPTIONS]
fetters delete [OPTIONS]

Options:
  -c, --company <COMPANY_NAME>   Filter results by company name.
  -l, --link <LINK>              Filter results by links.
  -n, --notes <NOTES>            Filter results by notes.
      --sprint <SPRINT>          Filter results by sprint name.
  -s, --status <STATUS>          Filter results by application status.
  -t, --title <TITLE>            Filter results by job title.
```

> [!TIP]
>
> All query options support partial text searching via the SQL `LIKE` operator.

The `delete` subcommand is very fast. A table of job applications (matching the query parameters or all applications if no query is provided) will be displayed, followed by an `inquire` prompt to select the job to delete.

The `update` subcommand will display a `MultiSelect` `inquire` prompt to select all the fields you want to update. `inquire` prompts will only show depending on the fields you have selected.

## Listing/Searching Jobs

> ![NOTE]
>
> If you are utilizing [different sprints](#managing-job-sprints), this subcommand will search for jobs within your current sprint that match your query.

Run the following command to list or search job applications:

```
fetters list
```

Like the [`update` and `delete` subcommands](#updating-or-deleting-a-job), this also supports the same query options:

```
fetters list [OPTIONS]

Options:
  -c, --company <COMPANY_NAME>   Filter results by company name.
  -l, --link <LINK>              Filter results by links.
  -n, --notes <NOTES>            Filter results by notes.
      --sprint <SPRINT>          Filter results by sprint name.
  -s, --status <STATUS>          Filter results by application status.
  -t, --title <TITLE>            Filter results by job title.
```

> [!TIP]
>
> All query options support partial text searching via the SQL `LIKE` operator.

Jobs matching your query parameters will be displayed in a table.

## Display Job Insights

> ![NOTE]
>
> If you are utilizing [different sprints](#managing-job-sprints), this subcommand will display insights for your current sprint.

Run the following command to show job application insights:

```
fetters insights
```

## Opening Links

> ![NOTE]
>
> If you are utilizing [different sprints](#managing-job-sprints), this subcommand will search for jobs within your current sprint that match your query.

Each record provides an optional link field. This field can be a URL to the job application (ie. `https://linkedin.com/jobs/view/...`) or a path to a local file (ie. a PDF or Word document).

Run the following command to open the URL or file:

```
fetters open
```

Like the `update`, `delete`, and `list` subcommands, this also supports the same query options:

```
fetters open [OPTIONS]

Options:
  -c, --company <COMPANY_NAME>   Filter results by company name.
  -l, --link <LINK>              Filter results by links.
  -n, --notes <NOTES>            Filter results by notes.
      --sprint <SPRINT>          Filter results by sprint name.
  -s, --status <STATUS>          Filter results by application status.
  -t, --title <TITLE>            Filter results by job title.
```

Jobs matching your query parameters will be displayed in a table. Once a job is selected, the link will be opened in your default browser or document viewer based on the file type.

[projectdirs documentation]: https://docs.rs/directories/6.0.0/directories/struct.ProjectDirs.html#method.data_dir
[releases]: https://github.com/JosephLai241/fetters/releases
