
CREATE TABLE IF NOT EXISTS meta (
    owner_type TEXT NOT NULL,
    owner_id BIGINT NOT NULL,
    field TEXT NOT NULL,
    value JSONB NOT NULL DEFAULT '{}',
    PRIMARY KEY (owner_type, owner_id, field)
);
