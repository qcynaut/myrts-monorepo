-- Your SQL goes here
alter TABLE schedules ADD COLUMN volumes TEXT[] NOT NULL DEFAULT '{}';