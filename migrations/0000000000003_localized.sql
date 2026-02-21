
CREATE TABLE IF NOT EXISTS localized (
    owner_type TEXT NOT NULL,
    owner_id BIGINT NOT NULL,
    field TEXT NOT NULL,
    locale TEXT NOT NULL,
    value TEXT NOT NULL,
    PRIMARY KEY (owner_type, owner_id, field, locale)
);
