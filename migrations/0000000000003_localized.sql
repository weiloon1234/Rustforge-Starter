
CREATE TABLE IF NOT EXISTS localized (
    id BIGINT PRIMARY KEY,
    owner_type TEXT NOT NULL,
    owner_id BIGINT NOT NULL,
    field TEXT NOT NULL,
    locale TEXT NOT NULL,
    value TEXT NOT NULL,
    UNIQUE (owner_type, owner_id, field, locale)
);
CREATE INDEX IF NOT EXISTS idx_localized_owner ON localized(owner_type, owner_id);
