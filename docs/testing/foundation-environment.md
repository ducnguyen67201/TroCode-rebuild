# P0 fixture environment

`npm run setup` installs locked development dependencies and creates ignored `.local/environment.json` / `.local/profiles.json` with random credentials (owner-only POSIX modes). It does not modify global config, inspect provider secrets or provision hosted resources.

`npm run dev:api` starts Compose project `tro-rebuild-foundation`, PostgreSQL at `127.0.0.1:55439`, private S3-compatible MinIO at `127.0.0.1:19000`, creates the private bucket, applies the isolated migration and seeds three identities. The API listens on `127.0.0.1:4318`. MinIO is used instead of the plan's LocalStack candidate to exercise actual signed requests and anonymous-access denial. Images are pinned by digest in `infra/compose.test.yml`.

Database: `tro_rebuild_test`; migration: `0001_foundation.sql`; tables: fixture_accounts/fixture_sessions. Migration refuses a database containing legacy `users`. This is not a legacy upgrade. Re-seeding preserves account IDs and does not revive expired/revoked sessions. No command deletes volumes or resets user data. Credentials expire after 30 days; removing a local credential file is not a supported renewal workflow. Deliberate session renewal is future operator work.

Profiles: teacher, student-a, student-b. These tokens represent isolated engineering fixtures, not production authentication. The API refuses fixture mode in release builds; commands never deploy it. Teacher/student roles are stored by the API. `GET /v1/me` ignores claimed role/account headers and reports the credential's account. `POST /v1/model-access/check` explicitly reports providerEnabled=false. Real authenticated SDK Responses/streaming integration is a P1/P2 gate.

Sample material: `tests/fixtures/materials/example.md`, object `tro-rebuild-fixtures/materials/example.md`. An unsigned read must return 401/403. Readiness verifies the expected migration checksum and fixture object; liveness only reports that HTTP is alive.

## Acceptance environments

Local fixture environment is reproducible engineering infrastructure. The legacy hosted test service in `TroCode/docs/testing/shared-test-environment.md` has not been contacted or changed. Its availability, credentials and deployed migration versions remain unverified. Hosted teacher/two-device acceptance and Windows/macOS interactive device evidence remain separate gates. D01–D07 stay unresolved where the master spec marks them unresolved.

## Commands

- `npm run db:up`: start only owned fixture infrastructure and create private bucket.
- `npm run db:migrate`: apply isolated migrations; DB must already be running.
- `npm run db:seed`: idempotent account/material seed.
- `npm run env:check`: inspect running API readiness.
- `npm run test:integration`: prepare fixture environment and run explicit integration tests.

Port collisions fail visibly. Existing containers and volumes belonging to other projects are never stopped. Fixture teardown may be performed explicitly with Docker Compose after providing the local environment; ordinary application shutdown does not stop databases.
