-- Your SQL goes here
alter TABLE records ADD COLUMN user_id INTEGER NOT NULL REFERENCES users (id);