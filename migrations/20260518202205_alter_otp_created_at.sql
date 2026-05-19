-- Add migration script here
ALTER TABLE otps
ALTER COLUMN created_at TYPE TEXT,
ALTER COLUMN expires_at TYPE TEXT;