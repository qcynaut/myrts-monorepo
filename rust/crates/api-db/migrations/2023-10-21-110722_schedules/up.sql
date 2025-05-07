-- Your SQL goes here
alter TABLE schedules ADD COLUMN user_id INTEGER NOT NULL;
alter TABLE schedules ADD FOREIGN KEY (user_id) REFERENCES users(id);