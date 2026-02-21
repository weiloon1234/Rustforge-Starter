CREATE TABLE IF NOT EXISTS personal_access_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tokenable_type TEXT NOT NULL,
    tokenable_id TEXT NOT NULL,
    name TEXT NOT NULL,
    token TEXT NOT NULL UNIQUE,
    token_kind TEXT NOT NULL CHECK (token_kind IN ('access', 'refresh')),
    family_id UUID NOT NULL,
    parent_token_id UUID,
    abilities JSONB,
    last_used_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_pat_tokenable_kind ON personal_access_tokens(tokenable_type, tokenable_id, token_kind);
CREATE INDEX IF NOT EXISTS idx_pat_token ON personal_access_tokens(token);
CREATE INDEX IF NOT EXISTS idx_pat_family_id ON personal_access_tokens(family_id);
CREATE INDEX IF NOT EXISTS idx_pat_parent_token_id ON personal_access_tokens(parent_token_id);
CREATE INDEX IF NOT EXISTS idx_pat_active_refresh ON personal_access_tokens(tokenable_type, tokenable_id, token_kind, revoked_at, expires_at);
