-- Add migration script here
ALTER TABLE users
ADD COLUMN email varchar(255) UNIQUE;