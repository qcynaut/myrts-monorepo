-- Your SQL goes here
create TABLE schedules (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    sid INTEGER NOT NULL,
    name TEXT NOT NULL,
    days TEXT NOT NULL,
    record_url TEXT NOT NULL,
    kind INTEGER NOT NULL,
    weeks TEXT NOT NULL,
    dates TEXT NOT NULL,
    times TEXT NOT NULL,
    month INTEGER,
    year INTEGER,
    volume DOUBLE
);