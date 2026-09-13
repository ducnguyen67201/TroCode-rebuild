# Plan: P0 source foundation

## Summary

Build the P0-A–P0-F foundation from the canonical [master specification](../../../../docs/rebuild-architecture.md). Optimize for short edit/run cycles and growth through explicit ownership, a single schema source, and replaceable I/O boundaries. Keep one desktop application, one Python runtime and one modular Rust API; complete implementation and test code before the verification batch.

This is a subordinate execution packet, not a competing product roadmap. The master spec wins on scope and decisions. Status: implemented and locally verified; remote CI/Windows gates remain pending. See [implementation report](../../reports/source-foundation-report.md). Prepared 2026-09-13.

## User story and scope

As a Tro developer, I want a reproducible workspace with independently runnable UI, native host, teaching worker and API, so that I can implement lessons and classroom features without rebuilding or starting unrelated systems.

Problem → solution: four documentation files and no Git/code/tooling → a runnable desktop health round trip, credential-free UI preview, scoped test API and predictable development commands.

- Complexity: XL; approximately 70–90 small source/config/test/generated files, 10 ordered tasks.
- Source PRD: `docs/rebuild-architecture.md`, execution checklist and detailed P0 acceptance.
- Requirements: F07 (stack), F08 (batched verification), C08 groundwork (authority); prepares F01/F03/F04/F09 for P1.
- Milestone: finish P0-A through P0-E and their tests together; then P0-F.
- Interpretation of “abstract enough”: isolate actual side effects and public contracts, keep feature logic concrete, and extract shared code when a second real consumer needs it.

## UX design

Internal developer foundation; no classroom product flow yet.

| Touchpoint | Before | After |
| --- | --- | --- |
| UI edits | No application | `npm run dev:ui`: Vite preview with explicit fake runtime status |
| Desktop edits | No application | `npm run dev`: Tauri + editable Python; start, health, stop, retry after failure |
| API work | No replacement API | `npm run dev:api`: local fixture API with explicit database configuration |
| Verification | Written policy only | One explicit `npm run verify` after the milestone |
| Failure | No process supervision | Starting/ready/stopping/stopped/failed, sanitized reason, explicit restart |

Developer diagnostics show runtime version and worker status. Production UI features will consume a narrow desktop client later. Browser preview must visibly say “Preview — simulated runtime”; it cannot count as native acceptance.

## Mandatory reading and discovery

`LEGACY` below means `/Users/ducng/Desktop/workspace/TroCode`. All legacy references are read-only migration input; do not import the Electron runtime. Line references describe the inspected revision and may drift.

| Priority | File | Lines | Why |
| --- | --- | --- | --- |
| Critical | `docs/rebuild-architecture.md` | 1–35, 139–263, 264–420, 610–687 | Ownership, P0 batches, migration and verification |
| Critical | `AGENTS.md` | all | Implementation order and external boundaries |
| Critical | `docs/testing/ci-workflow.md` | all | CI is default; separate dev and verification |
| Reference | `LEGACY/services/agent-runtime/src/protocol.ts` | 5–95 | Version/digest, bounded validation, unknown outcomes |
| Reference | `LEGACY/services/api/src/error.rs` | 9–103 | Public stable errors/private source |
| Reference | `LEGACY/services/api/src/observability.rs` | 1–18 | Structured tracing and colocated Rust tests |
| Reference | `LEGACY/services/api/src/http/middleware.rs` | 11–42 | Correlation/timing metadata |
| Reference | `LEGACY/services/api/src/auth/sessions.rs` | 12–88 | Account/session ownership, token digests, SQL transactions |
| Reference | `LEGACY/services/api/src/db.rs` | all, especially 228–270 | Migration registration and alternate historical ordering |
| Reference | `LEGACY/src/main/auth/auth-session-store.ts` | 1–70 | OS-encrypted auth storage; incompatible with plain copying |
| Reference | `LEGACY/src/main/agent-runtime/encrypted-agent-state-store.ts` | 1–65 | Existing state/journal ownership |
| Reference | `LEGACY/services/agent-runtime/test/protocol-and-graph.test.ts` | 1–65 | Contract tests and deterministic fixtures |
| Reference | `LEGACY/services/api/src/http/classroom_test_fixture.rs` | 6–32 | Fixture access controlled by server config |
| Reference | `LEGACY/docs/testing/shared-test-environment.md` | 1–65 | Existing shared test environment, not automatically reusable |
| Reference | `LEGACY/package.json`, `LEGACY/services/api/Cargo.toml` | scripts/dependencies | npm, Vitest, Axum/Tokio/SQLx precedent |

The requested `docs/CODEX-NAVIGATION-GUIDE.md` is absent in both checkouts. Create a rebuild-specific guide as part of P0; do not invent inherited instructions.

### Source baseline observed

- New workspace: four files, no `.git`, manifests, scaffold or CI.
- Legacy checkout: `main`, HEAD `2a66b0bf84e1cbfc22cb8aabfba618285a909084`; untracked `.claude/PRPs/plans/agents-sdk-skill-architecture.plan.md`.
- Planning worktree `/Users/ducng/.codex/worktrees/9ee2/TroCode`: modified `AGENTS.md`, `README.md`, `docs/testing/ci-workflow.md`; untracked `docs/rebuild-architecture.md`. Preserve all.
- Legacy migration files extend through `037_classroom_lessons.sql`. The migrator recognizes an alternate ordering using version 34's checksum. A plain file copy into a new migrator is insufficient compatibility evidence.
- Hosted test API is documented in the legacy testing guide. Its current availability, deployed migration history and access are unverified. P0 uses an isolated local environment; hosted provisioning remains a separately recorded external dependency.

### Unified discovery table

| Category | Evidence | Retain or change |
| --- | --- | --- |
| Similar implementations | Legacy protocol and encrypted state store above | Retain bounded versioned transport and unknown-result rule; replace TS orchestration |
| Naming | `auth/sessions.rs:12–38`, `protocol.ts:62–80` | PascalCase types, camelCase wire properties, snake_case Rust/Python members |
| Errors | `error.rs:84–91` | Public stable code/message; private causal error; never send raw exception |
| Logging | `middleware.rs:38–42` | IDs, elapsed time and outcome; omit bodies/screens/secrets |
| Types | `protocol.ts:30–47,62–80` | Strict bounded schemas; replace Zod authority with language-neutral JSON Schema |
| Tests | `protocol-and-graph.test.ts:41–58`, `observability.rs:11–18` | Vitest behavior tests, colocated Rust unit tests; Python pytest is new |
| Configuration | Legacy manifests and shared-test guide | Keep npm familiarity; separate local fixture mode from hosted auth |
| Dependencies | Legacy API Cargo.toml | Axum/Tokio/Serde/tracing/SQLx; omit unrelated connectors, audio and agent service graph |

### Patterns to mirror (actual excerpts)

Naming, `LEGACY/services/api/src/auth/sessions.rs:26–29`:
```rust
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssuedSession {
    pub access_token: String,
```
Error handling, `LEGACY/services/api/src/error.rs:84–90`:
```rust
pub fn internal(error: impl Into<anyhow::Error>) -> Self {
    Self {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        code: None,
        message: "An internal error occurred.",
        retry_after_seconds: None,
        source: Some(error.into()),
```
Logging, `LEGACY/services/api/src/observability.rs:5–8`:
```rust
let _ = tracing_subscriber::registry()
    .with(filter)
    .with(tracing_subscriber::fmt::layer().json().flatten_event(true))
    .try_init();
```
Repository, `LEGACY/services/api/src/auth/sessions.rs:35–40`:
```rust
#[derive(Clone, Debug)]
pub struct SessionRepository {
    pool: PgPool,
    hmac_key: Vec<u8>,
    duration_days: u32,
}
```
Test style, `LEGACY/services/api/src/observability.rs:11–17`:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn repeated_initialization_is_safe() {
        super::init();
        super::init();
    }
```
Service composition: no replacement implementation exists to mirror. New convention: constructors receive concrete dependencies; introduce a trait/interface only for a real I/O substitution. Do not reproduce legacy repository fields or formatting merely to resemble old code.

### Five traces

1. Entry: React `DesktopClient` → named Tauri command → Rust `WorkerSupervisor` → inherited pipes → Python entry point.
2. Data: canonical schema → generated types plus runtime validation → correlated response → small UI status projection. API identity comes from a validated credential, never a body account ID.
3. State: Rust owns child generation and pending requests; Python owns its runtime lifecycle. P0 stores no learner content. API owns fixture account records. P1 adds Python-owned durable sessions/journal.
4. Contracts: protocol version/schema digest/capabilities; typed errors; account-generation invalidation; model-proxy boundary; separate readiness/liveness.
5. Patterns: direct composition, pure lifecycle transitions, narrow adapters, feature modules and explicit ownership. No shared mutable service locator.

## External documentation and research

Checked 2026-09-13. These establish design constraints, not tested compatibility of a pinned dependency set.

| Topic/source | KEY_INSIGHT | APPLIES_TO | GOTCHA |
| --- | --- | --- | --- |
| [Tauri sidecars](https://v2.tauri.app/develop/sidecar/) | Bundled binaries use target-specific naming and can be launched from Rust | P1 package boundary | A working dev interpreter is not a bundled runtime proof |
| [Tauri commands](https://v2.tauri.app/develop/calling-rust/) and [capabilities](https://v2.tauri.app/security/capabilities/) | Native functions have explicit command/capability boundaries | Desktop client | Validate authorization inside commands; plugin permissions do not substitute for domain policy |
| [uv locking](https://docs.astral.sh/uv/concepts/projects/sync/) | Editable project installs support source iteration; `--locked` checks lock consistency | Python dev/CI | `--frozen` skips freshness checks; prefer `--locked` for CI |
| [json-schema-to-typescript](https://github.com/bcherny/json-schema-to-typescript) | Generate TS declarations from schema | Contracts | Types do not validate input; current docs require Node >=22.19 |
| [typify](https://github.com/oxidecomputer/typify) | Generate Rust types from JSON Schema | Contracts crate | Keep schema constructs conservative; runtime checks still required |
| [datamodel-code-generator](https://github.com/datamodel-code-generator/datamodel-code-generator) | Generate Pydantic v2 models | Python contracts | Disable timestamps and coercion in boundary use; generated types aren't the wire authority |
| [Ajv](https://ajv.js.org/json-schema.html), [Python jsonschema](https://python-jsonschema.readthedocs.io/en/stable/validate/) | Validate the same schema in both consumers | Validation parity | Format checking/coercion defaults differ; use explicit constraints and one golden corpus |
| [CUA process model](https://cua.ai/docs/reference/cua-driver/process-model) | Direct SDK runs in importing process; app owns lifecycle | P1 seam in Python | macOS permissions depend on app identity; do not add a second driver daemon by default |

## Strategic design

### Package and dependency strategy

Use npm workspaces (root lockfile), one Cargo workspace (root lockfile, resolver 3), and one uv Python project with its own lockfile. Start with Node 24 LTS and Python 3.12 compatibility lines; resolve and record exact available patch/tool versions during P0-C. Select a supported stable Rust toolchain compatible with Tauri and Rust edition 2024; do not blindly copy legacy `1.97.1`. No manifest may retain `latest` or unbounded dependency versions after P0.

| Area | Dependencies to pin in implementation |
| --- | --- |
| Desktop | React, react-dom, Vite, TypeScript, Tauri 2 API/CLI, Vitest, Testing Library, ESLint and Prettier |
| Contracts | json-schema-to-typescript, Ajv; Rust typify/serde/serde_json/jsonschema; Python datamodel-code-generator/pydantic/jsonschema |
| Python | uv, pytest, Ruff, mypy; stdlib asyncio/logging; SDK/CUA dependencies enter in P1/P2 when consumed |
| Native host | tauri/tauri-build, Tokio process/io/time/sync, serde, thiserror, tracing, uuid; no renderer shell plugin grants |
| API | Axum, Tokio, serde, tracing, thiserror, SQLx Postgres/migrate, SHA-256/token RNG, uuid; private object-store adapter |
| Test infra | Docker Compose, PostgreSQL and an S3-compatible test service (LocalStack S3), exact image digests recorded |

P0 pins what it consumes. P1 must select and validate the Python bundler against native CUA assets before claiming installation support. `package` in P0 exposes a clear prerequisite error until a real bundled runtime is available; no installer with an implicit system-Python dependency.

### Fast development contract

- `npm run dev:ui`: frontend-only, no Rust/Python/Docker/auth/cloud calls. React edits use Vite HMR.
- `npm run dev`: real Tauri host and one `.venv` Python child; default diagnostic session uses no model/API access.
- `npm run dev:api`: API plus explicit local infrastructure start, migrations and fixture seeding. No cloud credentials needed.
- `npm run dev:full`: explicit composition of API and native development. Never required for a CSS change.
- Python edits restart only the worker after cancelling the diagnostic session; no auto-resume. Use explicit `dev:runtime:restart` initially rather than a second watcher framework.
- Rust host changes use Tauri's native rebuild path; API changes use a separate explicit restart command initially. No global process-kill commands.
- Node orchestration scripts use `spawn` argument arrays and OS-specific executable resolution; no Unix-only shell command chains in npm scripts.
- Resolve/install on setup or explicit dependency changes, not on every start. Dev invokes `.venv` Python directly after setup; do not launch a package-manager parent instead of the owned worker.
- Generated bindings are committed. Generate only after schema edits; CI checks drift once. Cargo reuses one target cache. No full lint/typecheck/tests/audits in HMR, save hooks, build hooks or normal startup.
- Record cold/warm start and UI edit latency on a named machine at P0-F. Structural acceptance: UI edits never restart Python/Rust, native startup performs no installs or verification, and API can remain stopped during worker health tests. Do not claim unmeasured speed targets.

### Abstraction budget and growth path

| Boundary | Implement now | Grow when needed |
| --- | --- | --- |
| UI ↔ host | `DesktopClient` with health/start/stop/status subscription; native + preview adapters | Add typed teaching commands in P1/P2 |
| Host ↔ worker | `WorkerSupervisor` and small pure lifecycle reducer, fixed executable resolver | Bundled resolver in P1; same message transport |
| Python | `main`, `protocol`, `runtime`, `config`; injected clock for timeouts | SDK planner, CUA adapter and journal inside this one runtime |
| API | `auth`, `health`, `model_access`, `db`, `storage` modules in one binary | Add classroom/material/submission feature modules in P3/P6 |
| Shared code | Wire contracts and conformance fixtures only | Extract real shared UI/domain code after two consumers exist |

No generic repository base, dependency-injection container, plugin registry, universal event bus, microservice per entity, Nx/Turborepo/Bazel, or speculative classroom tables. Use `domain → boundary → concrete adapter` where a side effect exists, not a mandatory stack of classes for every function.

Alternatives considered: a local HTTP worker introduces listener/auth/reconnect complexity without a P0 consumer; three hand-maintained schemas drift; a TS-owned schema makes other runtimes secondary; copying the old backend imports unrelated orchestration. Selected: private pipes, JSON Schema authority, minimal API with migration inventory rather than production cutover.

### Wire protocol (new design)

Canonical source: `packages/contracts/schema/protocol.schema.json`, JSON Schema draft-07, closed object variants using `oneOf` and `kind` constants. All required fields explicit; no implicit defaults, remote `$ref`, coercion or extensible arbitrary tool payload.

Envelope fields: `protocolVersion` (integer 1), `kind`, `requestId` (UUID string), `correlationId` (UUID), `generationId` (UUID). Session events additionally require `sessionId`; event `sequence` is monotonic per generation and bounded to JS-safe integers. A digest is SHA-256 over canonical recursively key-sorted schema JSON serialized as compact UTF-8 without trailing newline. Use the same generated digest in each language.

P0 variants: `runtime.initialize`/`runtime.ready`, `runtime.health`/`runtime.healthResult`, `runtime.start`/`runtime.started`, `runtime.stop`/`runtime.stopped`, `runtime.shutdown`/`runtime.shutdownComplete`, `runtime.error`. Start means a diagnostic runtime session, not a teaching task. Health works without auth; authenticated account binding is host-owned and immutable for a worker generation. Initialize carries expected digest/version; ready returns actual values and implemented capabilities. Reject mismatch before start.

Private stdin/stdout use newline-delimited UTF-8 JSON. Stdout contains protocol only; structured sanitized logs use stderr. Enforce 256 KiB per frame before JSON parsing, max 32 pending ordinary requests, and bounded read buffers. Reserve a control path for stop/shutdown. Initial deadlines: handshake 5 seconds, health 2 seconds, graceful shutdown 3 seconds then kill/wait on the owned child. These are engineering defaults, not classroom latency promises.

Reject malformed/unknown kinds, duplicate outstanding request IDs, stale generations and invalid sequences. A timed-out response cannot fulfill another request. EOF fails pending requests without replay. Do not promise cancelling a future native action undoes its side effect.

Errors: `{code, message, retryable, correlationId}` with stable codes `INVALID_MESSAGE`, `PROTOCOL_MISMATCH`, `NOT_READY`, `BUSY`, `TIMEOUT`, `WORKER_EXITED`, `UNAUTHORIZED`, `FORBIDDEN`, `INTERNAL`. Retryable is UI guidance, never permission for automatic mutation replay.

Rust validates JSON before deserialization; TS validates received values with Ajv; Python validates using Draft7Validator before generated model construction with strict validation. Use a restricted UUID regex and integer limits in schema rather than relying on different format/coercion defaults. Same valid/invalid fixtures must pass identically in all languages.

### Account and model access boundary

P0 fixture API: `GET /healthz` (liveness), `GET /readyz` (database/migration/object-store reachability), `GET /v1/me` (authenticated account) and `POST /v1/model-access/check` (authenticated provider-access capability, providerEnabled=false in fixture mode). This last endpoint is an explicit stub contract, not an OpenAI Responses implementation.

Prepare `model-access.schema.json` for capability/error responses and document the P1 streaming requirement: Python SDK calls an authenticated Rust API model endpoint; API owns provider credentials, authorization, quotas and accounting. Actual Responses streaming, tool-call fidelity and cancellation must be proven with SDK integration in P1/P2, not simulated by returning arbitrary text in P0.

Fixture credentials: opaque random tokens generated locally, hashed in the test database, bound server-side to teacher/student account IDs. No caller-supplied role/account header. Bind API to loopback by default. Fixture mode is explicit and refused in release/hosted mode; `/v1/me` provides server identity. Rust holds selected credential in memory, queries identity and binds worker generation. Credential is never sent to React, logged, placed in a process argument or persisted in browser storage. P0 Python needs no credential because it performs no model calls.

Changing account: stop and reap old child, clear pending events/credential, resolve new identity, then create new generation. Failed identity resolution leaves no authenticated session. Python persistence will use account-specific storage in P1 with OS-backed protection; P0 only verifies identity isolation in memory. No placeholder plaintext persistence.

### Database, object storage and migration safety

Create an isolated database namespace `tro_rebuild_test`, unique Compose project/volumes and private fixture bucket. P0 baseline contains only fixture accounts/sessions with explicit expiry/revocation; no replacement classroom schema yet. New migration namespace starts at `0001_foundation.sql`; never run it against the legacy database. Guard commands with configured environment and expected database identity. Seed repeatably; never reset volumes implicitly.

Legacy compatibility inventory records authentication, roles, materials, assignments, submissions, task history, settings, permissions, SDK checkpoints and updater identity; list owner, source path, current format, disposition and future acceptance. Record checksums of all 37 migration files and both legacy migrator histories. Preserve readable history; opaque Electron encryption/checkpoints require a designed migration and cannot be copied as resumable Python state.

Test environments: local reproducible fixture backend fulfills independent P0 engineering work; real multi-device shared backend/device credentials remain explicitly pending if unavailable. Document existing hosted environment as a candidate only. Never label a local seed as completed shared-device acceptance.

## Files to change

All paths below are planned, except existing documentation. Grouped entries explicitly identify the source/config families to create; generated outputs are never hand-edited.

| Path | Action | Purpose |
| --- | --- | --- |
| `README.md`, `AGENTS.md`, `docs/rebuild-architecture.md`, `docs/testing/ci-workflow.md` | UPDATE | Commands, ownership, linked packet, evidence |
| `docs/CODEX-NAVIGATION-GUIDE.md` | CREATE | Actual module/import map and feature recipe |
| `docs/migration-inventory.md`, `docs/testing/foundation-environment.md` | CREATE | Evidence inventory and reproducible environments |
| `.gitignore`, `.gitattributes`, `.editorconfig`, `.env.example` | CREATE | Ignore private state/build outputs and consistent text |
| `package.json`, `package-lock.json`, `.node-version`, `rust-toolchain.toml`, `Cargo.toml`, `Cargo.lock` | CREATE | Workspace/tool pins |
| `eslint.config.mjs`, `.prettierrc.json`, `tsconfig.base.json` | CREATE | Shared lightweight tooling |
| `apps/desktop/{package.json,index.html,vite.config.ts,tsconfig.json,AGENTS.md}` | CREATE | Frontend workspace |
| `apps/desktop/src/{main.tsx,App.tsx,styles.css}` | CREATE | Diagnostic UI |
| `apps/desktop/src/platform/{desktop-client.ts,tauri-client.ts,preview-client.ts}` | CREATE | Single frontend/native seam |
| `apps/desktop/src/features/runtime/{RuntimeStatus.tsx,runtime-state.ts,runtime-state.test.ts,RuntimeStatus.test.tsx}` | CREATE | Lifecycle view and tests |
| `apps/desktop/src-tauri/{Cargo.toml,build.rs,tauri.conf.json,capabilities/main.json}` | CREATE | Host build and narrow capabilities |
| `apps/desktop/src-tauri/src/{main.rs,lib.rs,commands.rs,worker.rs,lifecycle.rs,config.rs}` | CREATE | Host supervision and command boundary |
| `apps/desktop/src-tauri/tests/worker_lifecycle.rs` | CREATE | Real subprocess tests |
| `services/teaching-runtime/{pyproject.toml,uv.lock,.python-version,AGENTS.md}` | CREATE | One editable Python project |
| `services/teaching-runtime/src/tro_runtime/{__init__.py,__main__.py,protocol.py,runtime.py,config.py,generated.py}` | CREATE | Worker and generated models |
| `services/teaching-runtime/tests/{test_protocol.py,test_runtime.py}` | CREATE | Python boundary tests |
| `packages/contracts/{package.json,Cargo.toml,README.md}` | CREATE | Shared package/crate metadata |
| `packages/contracts/schema/{protocol.schema.json,model-access.schema.json}` | CREATE | Canonical wire schemas |
| `packages/contracts/src/{index.ts,generated.ts,validate.ts,lib.rs,generated.rs}` | CREATE | TS/Rust bindings and wrappers |
| `packages/contracts/tests/{conformance.test.ts,conformance.rs}` | CREATE | Same corpus across consumers |
| `services/api/{Cargo.toml,AGENTS.md}` | CREATE | Modular backend |
| `services/api/src/{main.rs,lib.rs,config.rs,error.rs,health.rs,auth.rs,model_access.rs,db.rs,storage.rs}` | CREATE | Minimal scoped API |
| `services/api/migrations/0001_foundation.sql`, `services/api/tests/foundation.rs` | CREATE | Isolated baseline and integration tests |
| `tests/fixtures/{contracts,materials,workers}/` | CREATE | Valid/invalid frames, public Markdown lesson, adversarial fake workers |
| `tests/acceptance/foundation.md` | CREATE | Native/manual evidence checklist |
| `scripts/rebuild/{cli.mjs,processes.mjs,contracts.mjs,ci-plan.mjs,ci-plan.test.mjs}` | CREATE | Thin cross-platform command dispatcher/codegen/routing |
| `infra/compose.test.yml` | CREATE | Isolated Postgres/S3 services |
| `.github/workflows/foundation.yml` | CREATE | Source, integration, native build gates; no deployment |

## Step-by-step tasks

VALIDATE items below describe test code/review evidence to prepare during implementation. Execute checks together in task 10, not after each task.

### Task 1 — Preserve and inventory (P0-A)
- ACTION: Initialize local Git on `codex/rebuild-foundation`; preserve all current files and both legacy checkouts.
- IMPLEMENT: Record observed SHA/dirty paths; add migration inventory with all domains above, migration checksum manifest and unresolved deployed-state evidence. Add ignore rules before staging. Do not copy private data or modify old migrations.
- MIRROR: Legacy migrator/history protection and master migration discipline.
- IMPORTS: Node `node:fs/promises`, `node:crypto` for inventory if scripted.
- GOTCHA: Version 34 checksum selects alternate history; production schema cannot be inferred from file names.
- VALIDATE: Review inventory completeness and diff; no changed legacy files or deleted state.

### Task 2 — Scaffold and pin (P0-B/C)
- ACTION: Create the file/package layout above and exact tool/dependency locks.
- IMPLEMENT: npm workspaces for desktop/contracts; Cargo members desktop host, API and contracts; uv project `tro-runtime`. Explicit dependency features and dev profiles; TypeScript strict mode; Ruff/mypy config. Document version resolution and native prerequisites in setup.
- MIRROR: Existing npm/Vitest and Rust idioms; new Python conventions explicitly stated.
- IMPORTS: Dependencies table; Rust aliases `tro_contracts`, `tro_api`, `tro_desktop`; TS `@tro/contracts`; Python `tro_runtime`.
- GOTCHA: Check tool versions are published/available; legacy manifest versions are not authority. Separate fixture data/app ID from installed Tro.
- VALIDATE: Source-review manifests and locks, no legacy imports or unused framework packages; later clean locked install.

### Task 3 — Canonical contracts and conformance (P0-D)
- ACTION: Write minimal schemas and deterministic binding generation.
- IMPLEMENT: Protocol and capability schemas above; TS generator, Rust typify generator, Python generated Pydantic v2 models. Expose narrow `parseMessage`/`parse_message` wrappers. Include digest and common golden corpus; generation check writes to a temporary directory and compares bytes.
- MIRROR: Bounded legacy schemas and unknown-outcome principle, not protocol v7 or old tool catalog.
- IMPORTS: `json-schema-to-typescript`, `ajv`; Rust `typify`, `serde_json`, `jsonschema`; Python `jsonschema`, `pydantic`.
- GOTCHA: Validators must agree on extra fields, integers, null and Unicode; validators cannot fetch schema references over the network. Generators run explicitly, not on every Rust edit.
- VALIDATE: Positive/negative corpus in three languages; generated drift fails the gate.

### Task 4 — Python diagnostic worker (P0-D)
- ACTION: Implement entry point and bounded protocol loop.
- IMPLEMENT: Fixed diagnostic lifecycle, hello/health/start/stop/shutdown, stderr metadata logs, input EOF cleanup. Single writer to stdout; asynchronous reader/control handling so stop isn't queued behind ordinary work. No SDK/CUA side effects.
- MIRROR: Pure state transitions and structured logs.
- IMPORTS: `asyncio`, `sys`, `logging`, `tro_runtime.generated`, `tro_runtime.protocol`.
- GOTCHA: Don't use Unix-only event-loop stdin APIs as the sole Windows transport; use a portable reader thread with a bounded handoff and deterministic shutdown.
- VALIDATE: Invalid initialization, repeated stop, oversized/partial input, EOF and timeout tests using the actual executable.

### Task 5 — Rust worker supervisor (P0-D)
- ACTION: Own exactly one child per active account/generation.
- IMPLEMENT: Resolve fixed `.venv` executable in dev; fixed bundle path in release (fail if missing). Spawn with pipes using Tokio; read bounded frames, track requests/generation, reserve stop, reject mismatch, drain/close/reap. Concurrent starts serialize. Child EOF on parent pipe close triggers exit. No automatic restart/replay.
- MIRROR: Explicit lifecycle/typed error patterns.
- IMPORTS: `tokio::process::Command`, `tokio::io`, `tokio::sync`, `tokio::time`, `tro_contracts`, `uuid`, `tracing`.
- GOTCHA: Do not await a response while holding the supervisor mutex. P0 child launches no grandchildren; P1 must extend cleanup proof for native components if needed.
- VALIDATE: Real subprocess tests for crash, hung shutdown, stale response, duplicate start, parent disconnect and paths containing spaces on Windows/macOS.

### Task 6 — Minimal desktop and preview (P0-B/D)
- ACTION: Connect diagnostic UI through `DesktopClient`.
- IMPLEMENT: Named `runtime_start`, `runtime_health`, `runtime_stop`, `runtime_status` commands plus scoped events. React receives sanitized status only; owns no worker/auth authority. Preview adapter uses same interface and fixtures. Unsubscribe on unmount; state snapshot closes event-subscription races.
- MIRROR: Narrow desktop API principle; PascalCase components/camelCase functions.
- IMPORTS: `react`, `@tauri-apps/api/core`, `@tauri-apps/api/event`, `@tro/contracts` only inside platform adapter where appropriate.
- GOTCHA: Configure custom command permissions/window scope explicitly; do not grant raw shell/fs/process/network plugin APIs. Missing native bridge must not silently fall back to preview in a packaged build.
- VALIDATE: UI transitions and actionable failure tests; source review of capability allowlist; real host round trip in acceptance.

### Task 7 — Scoped test API and model-access contract (P0-D/E)
- ACTION: Implement minimal endpoints and account identity boundary.
- IMPLEMENT: Axum routing, validated config, public errors/private logs, fixture token verifier with expiry/revocation. `me` derives identity from token digest; capability endpoint is explicit disabled-provider stub. Host loads selected local fixture token without exposing it to the UI. Account change invalidates old worker generation.
- MIRROR: Legacy error, tracing and session repository principles.
- IMPORTS: `axum`, `sqlx`, `sha2`, `uuid`, `tracing`, `tro_contracts`; `crate::{auth,config,error}`.
- GOTCHA: Health is not authorization; fixture-role headers are forbidden. No real provider call or fake production authentication.
- VALIDATE: Missing/invalid/expired credential, spoofed identity, account switch and release fixture-mode refusal tests.

### Task 8 — Reproducible test infrastructure (P0-E)
- ACTION: Add isolated Compose services, migration and fixtures.
- IMPLEMENT: Database baseline with fixture-account/session tables, repeatable seed for teacher/student-A/student-B, private bucket and sample lesson digest. Generate secrets to ignored local files with restricted permissions; never overwrite existing developer configuration. Readiness checks explicit dependencies with timeouts. Document native device/shared-hosted evidence still required.
- MIRROR: Server-controlled fixture enablement and immutable SQL history.
- IMPORTS: SQLx runtime queries/migrations; test S3 client; Node filesystem/crypto.
- GOTCHA: Database identity guard before migration/seed. Do not point new migrations at legacy/staging. No automatic `down -v` or real account provisioning.
- VALIDATE: Seed twice preserves account identities; unauthenticated object access fails; readiness fails accurately; existing-data rows survive repeated startup.

### Task 9 — Fast commands, CI and contributor guide (P0-C/F preparation)
- ACTION: Wire one thin root CLI to npm scripts and CI jobs.
- IMPLEMENT: `setup`, `dev`, `dev:ui`, `dev:api`, `dev:full`, `dev:runtime:restart`, `build`, `verify`, `package`, `contracts:generate`, `contracts:check`, `db:migrate`, `db:seed`, `env:check`. Root JS scripts own their child processes and forward exit codes/signals. Add dependency-aware CI routing and cache keys from lockfiles/toolchain/OS. Unknown/root/contracts changes run all affected gates; docs-only runs link/routing checks; UI-only skips Python/API checks; native changes build both target OSes. CI job definitions do not authorize a push.
- MIRROR: Founder implementation-first workflow.
- IMPORTS: Node built-ins, pinned project CLIs; no custom task graph engine.
- GOTCHA: Commands are proposed until implemented. Package must not claim P1 success. CI unavailable means verification pending, not silent local full-suite fallback.
- VALIDATE: Routing tests cover root, docs, UI, contracts, Python, API and unknown paths; source-review scripts for hidden install/typecheck chains and unsafe process killing.

### Task 10 — Milestone review and verification (P0-F)
- ACTION: Review full implementation and test source, collect corrections, then execute applicable verification via CI once an authorized remote exists.
- IMPLEMENT: Record exact commands/results, missing external/native evidence and timings in foundation acceptance document; update master phase status accurately. Group failures and fix them together. If CI cannot run, report pending; local focused diagnosis remains allowed for concrete failures.
- MIRROR: `docs/testing/ci-workflow.md`.
- IMPORTS: None beyond pinned verification tools.
- GOTCHA: No phase complete claim on mocks alone; P0 native health proof differs from P1 installer/CUA proof.
- VALIDATE: All applicable checks on final revision; no provider calls, external provisioning or release actions hidden in verification.

## Testing strategy

| Behavior | Input | Expected |
| --- | --- | --- |
| Schema parity | Golden frames in TS/Rust/Python | Same accept/reject decisions |
| Hostile payload | Extra property, boolean integer, null, bad UUID, oversized frame | Rejected before dispatch |
| Framing | Split UTF-8, multiple frames, unterminated oversized line | Correct buffering or bounded failure |
| Handshake | Wrong version/digest | Start blocked with mismatch |
| Concurrent start | Two calls | One child and consistent response |
| Stop priority | Full ordinary queue | Control admitted, bounded shutdown |
| Generation isolation | Late response from old account/worker | Discarded; never updates new UI/session |
| Process failure | Crash or EOF before reply | Pending requests fail, no replay |
| Child cleanup | Normal quit, parent disconnect, hung worker | Exit/reap or owned-child kill; no orphan |
| Auth | Missing/expired/revoked token, role/account spoof | 401/403, identity derived server-side |
| Environment | Missing DB/bucket, permission denied | Actionable readiness/config error |
| Seed | Run twice with existing fixtures | Stable records, preserved unrelated data |
| Logging | Error includes token-like/private payload | No body/token in public error or logs |
| UI | Preview/native failures, retry, unmount | Accurate state, no listener leaks |
| CI routing | Path change matrix | Required gates selected, unknown paths conservative |

Edge coverage: empty input, maximum size and size+1, invalid types, concurrency, network failure, permission denial, shutdown races, ports in use, Windows paths, account switches. Write tests for these boundaries rather than snapshots of empty scaffold files. Add a fake worker fixture for wrong digest, hanging shutdown, stale reply and exit-before-response.

## Validation commands

These are interfaces to implement, not commands already available. Run the aggregate once after milestone implementation/source review, in CI by default. Individual commands exist for CI routing and focused diagnosis; don't run both individually and again as an unchanged full batch.

```bash
npm ci
uv sync --project services/teaching-runtime --locked
npm run contracts:check
npm run lint
npm run typecheck
npm run test:typescript
uv run --project services/teaching-runtime --locked ruff check .
uv run --project services/teaching-runtime --locked ruff format --check .
uv run --project services/teaching-runtime --locked mypy src
uv run --project services/teaching-runtime --locked pytest
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
npm run db:migrate
npm run db:seed
npm run env:check
npm run test:integration
npm run build
```

Expected: locked reproducible dependencies, no generated drift/type/lint errors, boundary and database tests pass, development artifacts build. API integration commands use the isolated fixture environment provisioned by the CI job. Native build jobs run on Windows/macOS with required Tauri system dependencies; Linux API/contracts jobs need not compile the desktop host. Scope Python commands to the project cwd in CLI wrappers so `.`/`src` resolve correctly. Include dependency audits in the post-implementation gate; before any commit, run required npm/Python audits under contributor guidance, and report network/tool failures honestly.

Aggregate interface: `npm run verify -- --scope all`; scopes `ui`, `runtime`, `api`, `native`, `contracts`, `docs` expand to their dependencies. `build` compiles, without calling `verify`. `package -- --target <triple>` remains a prerequisite-reporting interface until P1 implements bundled artifacts.

Manual acceptance:
- [ ] Clean setup documents required dev tools and starts UI preview with no credentials/Docker.
- [ ] `dev` launches real host/worker; health matches protocol; start/stop works.
- [ ] Edit UI and observe HMR without worker restart; record warm startup measurements.
- [ ] Kill Python; error appears; explicit restart produces fresh generation, no replay.
- [ ] Quit host during diagnostic work; child exits. Repeat on both OS targets when devices available.
- [ ] Switch fixture accounts; old response and state cannot enter new session.
- [ ] Start local API, seed twice, authenticate three distinct identities.
- [ ] No raw native tools or credentials available through browser/renderer APIs.
- [ ] Hosted environment/device/packaging evidence is clearly marked pending where absent.

## Acceptance and completion checklist

- [ ] P0-A–E delivered and reviewed; no legacy edits lost.
- [ ] One private host/worker round trip, bounded failure and clean shutdown.
- [ ] One schema source, generated types, runtime validation and shared negative corpus.
- [ ] Fast UI-only/native/API paths; no installs or full gates during normal restarts.
- [ ] API account boundary and isolated repeatable database/object-store fixtures.
- [ ] Exact consumed tools/dependencies/lockfiles and documented commands.
- [ ] Named remaining D01–D07 effects; no invented customer decisions.
- [ ] Applicable P0-F checks pass on final revision or verification explicitly pending.
- [ ] Developer guide includes a concrete recipe: add schema variant → generate → handler/module → adapter/UI → boundary test; no new service required.
- [ ] Public errors and structured logs contain no private bodies/credentials.
- [ ] No unused framework abstractions, empty future subsystems or duplicated writers.
- [ ] Master spec remains canonical; this packet is linked as P0 detail.

## Not building

Full planner/SDK model loop, CUA native actions, overlay, action journal or installer proof (P1/P2); real login migration or provider streaming implementation; classroom dashboard/material ingestion (P3); broadcasts, interventions, submissions, voice, analytics, gamification; production migrations, remote provisioning, push/deploy/release. Define the next seams but do not advertise these as working capabilities.

## Risks and unresolved evidence

| Risk | Likelihood | Impact | Mitigation |
| --- | --- | --- | --- |
| Codegen/validator disagreement | Medium | High | Conservative draft-07 subset and shared corpus; schema remains authority |
| Over-abstraction slows progress | Medium | High | Four real boundaries; no framework/extraction without consumer |
| Legacy migration history differs from file order | Confirmed design case | High | Preserve both migrators/checksums; no legacy DB writes in P0 |
| macOS TCC/overlay/bundler incompatibility | Unproven | High in P1 | Packaged P1 gate before broader UI reconstruction |
| Windows subprocess handling differs | Medium | High | Cross-platform pipes, path tests and native CI; manual evidence tracked |
| Real SDK streaming contract remains unproven | Expected P1 work | High in P1 | Explicit disabled stub and subsequent SDK integration gate |
| Hosted credentials/devices unavailable | Unknown | Medium | Independent local fixtures; shared-device evidence pending |
| Version resolution changes | Medium | Medium | Resolve once during implementation, record exact pins and lockfiles |

Planning confidence: 8/10 for P0 implementation from this packet; no confidence claim for P1 native packaging. Remaining package-version resolution is an explicit implementation task, not a claimed tested dependency set. Pilot decisions D01–D07 remain in the master register and do not block independent P0 work.
