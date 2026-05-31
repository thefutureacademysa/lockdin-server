-- Add migration script here
-- Regular indexes
CREATE INDEX IF NOT EXISTS idx_rooms_grade ON rooms(grade);
CREATE INDEX IF NOT EXISTS idx_rooms_category ON rooms(category);
CREATE INDEX IF NOT EXISTS idx_rooms_is_live ON rooms(is_live);

-- Full text search
CREATE INDEX IF NOT EXISTS idx_rooms_search ON rooms
    USING GIN(to_tsvector('english', name || ' ' || subject));

-- Trigram for partial search
CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE INDEX IF NOT EXISTS idx_rooms_name_trgm ON rooms
    USING GIN(name gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_rooms_subject_trgm ON rooms
    USING GIN(subject gin_trgm_ops);