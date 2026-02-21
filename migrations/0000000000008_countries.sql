
CREATE TABLE IF NOT EXISTS countries (
    iso2 TEXT PRIMARY KEY,
    iso3 TEXT NOT NULL UNIQUE,
    iso_numeric TEXT,
    name TEXT NOT NULL,
    official_name TEXT,
    capital TEXT,
    capitals TEXT[] NOT NULL DEFAULT '{}',
    region TEXT,
    subregion TEXT,
    currencies JSONB NOT NULL DEFAULT '[]',
    primary_currency_code TEXT,
    calling_code TEXT,
    calling_root TEXT,
    calling_suffixes TEXT[] NOT NULL DEFAULT '{}',
    tlds TEXT[] NOT NULL DEFAULT '{}',
    timezones TEXT[] NOT NULL DEFAULT '{}',
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    independent BOOLEAN,
    status TEXT NOT NULL DEFAULT 'disabled',
    assignment_status TEXT,
    un_member BOOLEAN,
    flag_emoji TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (char_length(iso2) = 2),
    CHECK (char_length(iso3) = 3),
    CHECK (iso_numeric IS NULL OR char_length(iso_numeric) = 3),
    CHECK (status IN ('enabled', 'disabled'))
);
CREATE INDEX IF NOT EXISTS idx_countries_name ON countries(name);
CREATE INDEX IF NOT EXISTS idx_countries_status ON countries(status);
CREATE INDEX IF NOT EXISTS idx_countries_region ON countries(region);
CREATE INDEX IF NOT EXISTS idx_countries_primary_currency_code ON countries(primary_currency_code);
CREATE INDEX IF NOT EXISTS idx_countries_currencies_gin ON countries USING GIN (currencies);
