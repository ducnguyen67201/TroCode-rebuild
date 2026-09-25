# Hosted authentication operations

This runbook describes source configuration and acceptance evidence. It does not
authorize Google Cloud changes, hosted migration, deployment, signing or release.

## Required inputs

Create a Google OAuth client of type **Desktop app** in the authorized Google
Cloud project. Configure the consent screen and permitted testers/domains there;
do not create or ship a client secret. Supply the public client ID to both the
hosted API and desktop as `TRO_GOOGLE_CLIENT_ID`. The loopback redirect is chosen
at runtime as `http://127.0.0.1:<ephemeral>/oauth2/callback`.

The hosted API also requires:

| Variable                  | Requirement                                         |
| ------------------------- | --------------------------------------------------- |
| `DATABASE_URL`            | PostgreSQL database owned by the hosted environment |
| `TRO_API_BIND`            | Loopback address behind the trusted HTTPS proxy     |
| `TRO_AUTH_ISSUER`         | Public HTTPS API origin used as JWT issuer          |
| `TRO_AUTH_AUDIENCE`       | Fixed desktop API audience                          |
| `TRO_JWT_KEY_B64`         | At least 32 random bytes, base64 encoded            |
| `TRO_REFRESH_KEY_B64`     | Different 32+ random bytes, base64 encoded          |
| `TRO_ACCESS_TTL_SECONDS`  | 300–3600; production default 900                    |
| `TRO_REFRESH_TTL_SECONDS` | 86400–2592000; production default 2592000           |

The desktop requires the same public Google client ID and a fixed HTTPS
`TRO_AUTH_API_ORIGIN`. Loopback HTTP is accepted only in debug builds. Keep keys
in the deployment secret manager; never put them in `.env`, bundles, logs or CI
artifacts.

## Migration and workspace integration

Run `npm run db:migrate:hosted` only against an approved database with the hosted
variables loaded. The command sets the explicit migration guard and uses the
SeaORM migration ledger. Take and verify a recoverable backup first, record the
source/target versions, and do not edit published migrations.

Before deployment, replace `PendingWorkspaceMembershipStore` with the workspace
branch's SeaORM repository. Its claim method must run in the auth transaction,
match only trim-and-lowercase verified email, preserve the owner-assigned role,
be idempotent, and record the workspace audit event. Without that adapter, login
is intentionally limited to `membershipRequired`.

Legacy Electron sessions are not imported. Supported upgrades require a fresh
Google sign-in; do not decrypt or copy old refresh material into the new keychain
entry.

## Key rotation and session cleanup

The current JWT implementation has one active HMAC signing key. Rotating
`TRO_JWT_KEY_B64` invalidates outstanding access JWTs (at most 15 minutes) while
valid refresh sessions can immediately obtain tokens under the new key. Coordinate
desktop availability and monitor terminal/retryable outcomes during the change.

Rotating `TRO_REFRESH_KEY_B64` invalidates every stored refresh digest and forces
all devices to sign in again. Treat this as a planned global logout or incident
response action, communicate it before rollout, and rotate the JWT key separately.
Never retain retired plaintext keys in application configuration.

Expired/revoked session-row cleanup is an operator-owned maintenance job. Delete
only rows past the approved retention window and preserve replay/audit evidence
for the required incident-retention period. Use a typed SeaORM maintenance
repository; do not add an ad-hoc SQL script.

## Public error taxonomy

| Code                     | Meaning                                    | Client behavior                                   |
| ------------------------ | ------------------------------------------ | ------------------------------------------------- |
| `AUTH_INVALID_GOOGLE`    | Provider proof is invalid                  | End attempt; allow a new sign-in                  |
| `AUTH_IDENTITY_CONFLICT` | Email belongs to another provider identity | Gate access; operator reconciliation              |
| `AUTH_UNAVAILABLE`       | Provider/backend transport failed          | Offline gate; retain refresh credential and retry |
| `SESSION_EXPIRED`        | Refresh/access validity ended              | Clear local credential and sign out               |
| `SESSION_REVOKED`        | Account or device session is inactive      | Clear local credential and sign out               |
| `SESSION_REPLAYED`       | A replaced refresh credential was reused   | Revoke the device family and sign out             |
| `INVALID_REQUEST`        | Bounded request validation failed          | Do not retry the same payload                     |

Logs may contain the stable code, correlation ID, account UUID after successful
identity establishment and workspace count. They must not contain email, Google
responses, OAuth code, verifier, nonce, ID/access/refresh token or key material.

## Packaged acceptance evidence

Run the source gate first, then test signed macOS and Windows packages against an
authorized hosted test environment. Capture cold-start gating, system-browser
handoff, exact-email auto-claim, membership-required retry, restart renewal,
offline recovery, expiry/revocation/replay logout, runtime cancellation, keychain
contents and redacted logs. Source tests do not substitute for these packaged OS
checks.
