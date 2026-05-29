-- Add migration script here
CREATE TABLE IF NOT EXISTS refresh_tokens
(
    id         VARCHAR PRIMARY KEY,
    user_id    VARCHAR        NOT NULL REFERENCES users (id),
    token      TEXT        NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked    BOOLEAN              DEFAULT FALSE
);