-- Your SQL goes here
alter TABLE records DROP COLUMN device_ids;
alter TABLE records ADD COLUMN hash VARCHAR(255) NOT NULL DEFAULT '';