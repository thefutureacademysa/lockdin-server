-- Add migration script here
ALTER TABLE otps
    DROP COLUMN phone_number,
    ADD COLUMN email varchar(255);
