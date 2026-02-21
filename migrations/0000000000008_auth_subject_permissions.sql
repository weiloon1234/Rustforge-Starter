
CREATE TABLE IF NOT EXISTS auth_subject_permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    guard TEXT NOT NULL,
    subject_id UUID NOT NULL,
    permission TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_auth_subject_permissions_guard_subject_permission
    ON auth_subject_permissions(guard, subject_id, permission);
CREATE INDEX IF NOT EXISTS idx_auth_subject_permissions_guard_subject
    ON auth_subject_permissions(guard, subject_id);
CREATE INDEX IF NOT EXISTS idx_auth_subject_permissions_permission
    ON auth_subject_permissions(permission);
