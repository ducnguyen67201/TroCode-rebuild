-- Separate identity domain; fixture tokens never authorize provider spending.
CREATE TABLE proof_accounts (
    id UUID PRIMARY KEY,
    role TEXT NOT NULL CHECK (role IN ('teacher', 'student'))
);
CREATE TABLE proof_sessions (
    token_digest BYTEA PRIMARY KEY CHECK (octet_length(token_digest) = 32),
    account_id UUID NOT NULL REFERENCES proof_accounts(id),
    expires_at TIMESTAMPTZ NOT NULL,
    revoked BOOLEAN NOT NULL DEFAULT FALSE
);
CREATE TABLE runtime_grants (
    token_digest BYTEA PRIMARY KEY CHECK (octet_length(token_digest) = 32),
    session_digest BYTEA NOT NULL REFERENCES proof_sessions(token_digest),
    account_id UUID NOT NULL REFERENCES proof_accounts(id),
    teaching_session_id UUID NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    remaining_calls INTEGER NOT NULL CHECK (remaining_calls BETWEEN 0 AND 4),
    revoked BOOLEAN NOT NULL DEFAULT FALSE
);
