-- Your SQL goes here
UPDATE role SET name = 'Root' WHERE id = 1;
UPDATE role SET name = 'SuperAdmin' WHERE id = 2;
INSERT INTO role (id, name) VALUES (3, 'Admin');