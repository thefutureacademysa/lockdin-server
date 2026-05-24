-- Add migration script here
ALTER TABLE otps ADD CONSTRAINT email_unique UNIQUE (email);