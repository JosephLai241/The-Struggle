-- This file contains definitions for all tables in SQLite.

-- This table holds all job applications.
CREATE TABLE jobs (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    created TEXT NOT NULL,
    company_name TEXT NOT NULL,
    title_id INTEGER NOT NULL,
    status_id INTEGER NOT NULL,
    link TEXT,
    notes TEXT,
    sprint_id INTEGER NOT NULL,
    FOREIGN KEY (title_id) REFERENCES titles (id),
    FOREIGN KEY (status_id) REFERENCES statuses (id),
    FOREIGN KEY (sprint_id) REFERENCES sprints (id)
);

-- This table holds all job sprints and their start/end dates.
CREATE TABLE sprints (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    start_date TEXT NOT NULL,
    end_date TEXT,
    num_jobs INTEGER NOT NULL
);

-- This table holds all job statuses (ie. "PENDING", "IN PROGRESS", "OFFER RECEIVED",
-- "REJECTED", "NOT HIRING ANYMORE").
CREATE TABLE statuses (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

-- This table holds all unique job titles.
CREATE TABLE titles (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);
