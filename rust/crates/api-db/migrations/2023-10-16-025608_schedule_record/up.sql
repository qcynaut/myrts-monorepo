-- Your SQL goes here

create TABLE
    records (
        id SERIAL PRIMARY KEY,
        name TEXT NOT NULL,
        created_at TIMESTAMP NOT NULL,
        description TEXT,
        file_url TEXT NOT NULL,
        device_ids INTEGER [] NOT NULL
    );

create TABLE
    schedules (
        id SERIAL PRIMARY KEY,
        name TEXT NOT NULL,
        hour INTEGER NOT NULL,
        minute INTEGER NOT NULL,
        days INTEGER [] NOT NULL,
        records_id INTEGER NOT NULL,
        FOREIGN KEY (records_id) REFERENCES records (id)
    );