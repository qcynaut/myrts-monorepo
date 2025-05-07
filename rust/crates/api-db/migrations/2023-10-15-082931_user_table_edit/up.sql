-- Your SQL goes here

ALTER TABLE users DROP COLUMN user_group_id;

ALTER TABLE users
ADD
    COLUMN user_group_ids INTEGER [] NOT NULL DEFAULT '{}';