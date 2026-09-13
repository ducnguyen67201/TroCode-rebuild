-- Isolated fixture namespace. This migration is never applied to legacy Tro.
CREATE TABLE fixture_accounts (
    id UUID PRIMARY KEY,
    profile TEXT NOT NULL UNIQUE CHECK (profile IN ('teacher', 'student-a', 'student-b')),
    role TEXT NOT NULL CHECK (role IN ('teacher', 'student'))
);
CREATE TABLE fixture_sessions (
    token_digest BYTEA PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES fixture_accounts(id),
    expires_at TIMESTAMPTZ NOT NULL,
    revoked BOOLEAN NOT NULL DEFAULT FALSE
);
