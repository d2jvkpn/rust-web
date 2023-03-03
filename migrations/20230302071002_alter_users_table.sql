-- Add migration script here
ALTER TABLE users ADD COLUMN password varchar(64) DEFAULT NULL;
--- can't use "ALTER birthday"
