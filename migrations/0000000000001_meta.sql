
CREATE TABLE IF NOT EXISTS meta (
    id BIGINT PRIMARY KEY,
    owner_type TEXT NOT NULL,
    owner_id BIGINT NOT NULL,
    field TEXT NOT NULL,
    value JSONB NOT NULL DEFAULT '{}',
    UNIQUE (owner_type, owner_id, field)
);
CREATE INDEX IF NOT EXISTS idx_meta_owner ON meta(owner_type, owner_id);
