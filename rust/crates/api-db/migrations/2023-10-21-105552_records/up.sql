-- Your SQL goes here
alter TABLE records DROP COLUMN duration;
alter TABLE records ADD COLUMN duration TEXT NOT NULL DEFAULT '';