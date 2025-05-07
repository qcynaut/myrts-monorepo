-- Your SQL goes here
alter TABLE schedules DROP COLUMN hour, DROP COLUMN minute;
alter TABLE schedules ADD COLUMN device_ids INTEGER [] NOT NULL;
alter TABLE schedules ADD COLUMN kind INTEGER NOT NULL;
alter TABLE schedules ADD COLUMN weeks INTEGER [] NOT NULL;
alter TABLE schedules ADD COLUMN dates INTEGER [] NOT NULL;
alter TABLE schedules ADD COLUMN times TEXT[] NOT NULL;