# Hosted authentication operations

This runbook describes source configuration and acceptance evidence. It does not
authorize Google Cloud changes, hosted migration, deployment, signing or release.

## Required inputs

Create a Google OAuth client of type **Desktop app** in the authorized Google
Cloud project. Configure the consent screen and permitted testers/domains there.
Supply the public client ID to both the hosted API and desktop as
`TRO_GOOGLE_CLIENT_ID`. If Google issued a client secret for the desktop client,
store it only in the hosted API secret manager as `TRO_GOOGLE_CLIENT_SECRET`;
never ship it to the desktop. The loopback redirect is chosen at runtime as
`http://127.0.0.1:<ephemeral>/oauth2/callback`.

The hosted API also requires:

| Variable                  | Requirement                                         |
| ------------------------- | --------------------------------------------------- |
| `DATABASE_URL`            | PostgreSQL database owned by the hosted environment |
| `TRO_API_BIND`            | Loopback address behind the trusted HTTPS proxy     |
| `TRO_AUTH_ISSUER`         | Public HTTPS API origin used as JWT issuer          |
| `TRO_AUTH_AUDIENCE`       | Fixed desktop API audience                          |
| `TRO_GOOGLE_CLIENT_SECRET`| Google client secret, when issued; hosted API only  |
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

The hosted service uses `WorkspaceService` as the canonical SeaORM membership
adapter. Its claim method runs in the auth transaction, matches only the
trim-and-lowercase verified email, preserves the owner-assigned role, is
idempotent and records the workspace audit event. `PendingWorkspaceMembershipStore`
remains only as an explicit test double; production startup must never select it.
An account without an active membership remains in `membershipRequired`.

The first workspace owner is provisioned by the approved operator/bootstrap
workflow; public self-service workspace creation is not part of this slice. Once
provisioned, an owner can list, add and remove non-owner memberships from the
desktop. Additions are pending until the matching Google account authenticates.
Removing a membership immediately excludes it from subsequent `/me` projections
and refresh responses. The native host revalidates `/me` before protected local
runtime and teaching commands, so cached client state cannot preserve removed
authority. The initial owner-management slice has a hard limit of 500 active
members per workspace; additions at capacity fail explicitly rather than creating
rows hidden by the bounded roster response.

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

| Code                             | Meaning                                    | Client behavior                                                       |
| -------------------------------- | ------------------------------------------ | --------------------------------------------------------------------- |
| `AUTH_INVALID_GOOGLE`            | Provider proof is invalid                  | End attempt; allow a new sign-in                                      |
| `AUTH_IDENTITY_CONFLICT`         | Email belongs to another provider identity | Gate access; operator reconciliation                                  |
| `AUTH_UNAVAILABLE`               | Provider/backend transport failed          | Offline gate; retain refresh credential and retry                     |
| `SESSION_EXPIRED`                | Refresh/access validity ended              | Clear local credential and sign out                                   |
| `SESSION_REVOKED`                | Account or device session is inactive      | Clear local credential and sign out                                   |
| `SESSION_REPLAYED`               | A replaced refresh credential was reused   | Revoke the device family and sign out                                 |
| `INVALID_REQUEST`                | Bounded request validation failed          | Do not retry the same payload                                         |
| `WORKSPACE_INVALID_REQUEST`      | Invalid email, role or workspace input     | Correct the owner-entered value                                       |
| `WORKSPACE_OWNER_REQUIRED`       | Active owner membership is absent          | Hide management UI; do not retry as authorization                     |
| `WORKSPACE_MEMBERSHIP_CONFLICT`  | Email has a different active role          | Reconcile the existing assignment                                     |
| `WORKSPACE_MEMBERSHIP_NOT_FOUND` | Membership is absent or removed            | Refresh the member list                                               |
| `WORKSPACE_MEMBER_LIMIT_REACHED` | Workspace already has 500 active members   | Remove access or use a separately designed paging/seat expansion flow |

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
