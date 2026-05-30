-- Add migration script here
CREATE TABLE IF NOT EXISTS rooms
(
    id                VARCHAR     NOT NULL PRIMARY KEY,
    name              VARCHAR     NOT NULL,
    subject           VARCHAR     NOT NULL,
    grade             INT         NOT NULL,
    category          VARCHAR     NOT NULL,
    participant_count INT         NOT NULL,
    is_live           BOOLEAN     NOT NULL DEFAULT FALSE,
    created_at        timestamptz NOT NULL DEFAULT now()
)