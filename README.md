```
      _         _         _        _        _        _         _
   __/\\___  __/\\___  __/\\__  __/\\__  __/\\___  _/\\___    /\\__
  (_  ____))(_  ____))(__  __))(__  __))(_  ____))(_   _  )) /    \\
   / ||__    /  ._))    /  \\    /  \\   /  ._))   /  |))// _\  \_//
  /:. ._))  /:. ||___  /:.  \\  /:.  \\ /:. ||___ /:.    \\// \:.\
  \  _))    \  _____)) \__  //  \__  // \  _____))\___|  //\\__  /
   \//       \//          \//      \//   \//           \//    \\/

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
```

> `fetters` - A command-line tool for tracking your job hunt struggles.

# Table of Contents

- [Introduction](#introduction)
- [What Does It Do?](#what-does-it-do)
  - [Features](#features)
- [How It Works](#how-it-works)

# Introduction

`fetters` exists because I wanted something that makes it faster to track all the countless job applications I send out whenever I'm on the job hunt _without_ touching any spreadsheets. This program is for people who love the command-line and hate touching spreadsheets as much as I do. I used to work at an insurance company and can't look at a spreadsheet without feeling a little [_dusty_](https://www.youtube.com/watch?v=n887wZHV3OY).

![fuck Excel](https://i.imgur.com/9EHgRwO.jpg)

# What Does It Do?

`fetters` is an overbuilt, customizable CLI tool that performs CRUD (**c**reate, **r**ead, **u**pdate, **d**elete) operations on a SQLite database that resides in the application data directory on your machine<sub><small>1</small></sub> and simplifies the process of tracking your job applications. Once you become familiar with the CLI, it is **_much_** faster than opening and editing a spreadsheet.

## Features

Here's a list of all the features implemented in `fetters`:

- Create, read, update, or delete job applications from the SQLite instance.
- Assign job applications to a `stint` -- a period of time when you are actively applying for jobs.
- Display/search for tracked job applications in your terminal.
  - `fetters` implements [`term-table`](https://crates.io/crates/term-table) to neatly display your job application data in tables (similar to, but still _not_ a spreadsheet 😉).
  - [`ansi_term`](https://crates.io/crates/ansi_term) is also implemented to paint job applications according to their status or for highlighting pattern matches when querying tracked jobs.
- Display insights for tracked applications, such as:
  - **Add something here**
- Customizable prompt and display options:
  - Override preset job titles to select from when interactively adding a new job (instead of typing the same title out each time).
  - Override preset job application status options.
  - Map custom colors to each job application status.
  - Override the default pattern match color and table width.



<small><sub>1</sub> _Refer to the table for [the `data_dir()` method in the `directories` crate](https://docs.rs/directories/5.0.1/directories/struct.ProjectDirs.html#method.data_dir) to find the path to the SQLite instance on your machine._</small>


# How It Works

