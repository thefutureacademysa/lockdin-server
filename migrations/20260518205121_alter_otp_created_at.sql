-- Add migration script here
ALTER TABLE otps
    ALTER COLUMN created_at TYPE TIMESTAMPTZ USING created_at::TIMESTAMPTZ,
    ALTER COLUMN expires_at TYPE TIMESTAMPTZ USING expires_at::TIMESTAMPTZ;