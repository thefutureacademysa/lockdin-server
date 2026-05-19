-- Add migration script here
ALTER TABLE otps
ALTER COLUMN created_at TYPE TIMESTAMPTZ;