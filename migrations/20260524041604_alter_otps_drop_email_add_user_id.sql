-- Add migration script here
ALTER TABLE otps DROP COLUMN email, ADD COLUMN user_id VARCHAR REFERENCES users(id);