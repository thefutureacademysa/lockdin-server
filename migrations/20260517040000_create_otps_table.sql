-- Create OTPs table for storing one-time passwords
CREATE TABLE IF NOT EXISTS otps (
    phone_number VARCHAR(20) PRIMARY KEY,
    code VARCHAR(6) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index on expires_at for efficient cleanup queries
CREATE INDEX IF NOT EXISTS idx_otps_expires_at ON otps(expires_at);
