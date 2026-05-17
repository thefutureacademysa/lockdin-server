-- Add migration script here
CREATE TABLE IF NOT EXISTS users
(
    id           VARCHAR PRIMARY KEY,
    full_name    VARCHAR        NOT NULL,
    school_name  VARCHAR        NOT NULL,
    grade        VARCHAR        NOT NULL,
    phone_number VARCHAR UNIQUE NOT NULL,
    avatar_url   VARCHAR,
    is_verified  BOOLEAN        NOT NULL DEFAULT FALSE,
    created_at   TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);