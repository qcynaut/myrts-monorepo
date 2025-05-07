-- Your SQL goes here
create TABLE device (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    uid VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL
);