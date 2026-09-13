# Implementation report: P0 source foundation

P0 source implementation and local validation are complete on `codex/rebuild-foundation`. No commit, remote, push, deployment, legacy-state mutation or release was performed. CI definitions exist; remote CI and Windows execution remain pending.

The [archived plan](../plans/completed/source-foundation.plan.md) is subordinate to the [master spec](../../../docs/rebuild-architecture.md). [Acceptance evidence](../../../tests/acceptance/foundation.md) records actual observations and remaining gates.

## Summary

Implemented an npm/Cargo/uv workspace with pinned dependencies, a credential-free React preview, a real Tauri desktop, a private Python diagnostic worker, a scoped Rust fixture API, PostgreSQL/private MinIO fixtures and CI definitions. JSON Schema is the single wire authority with generated TS/Rust/Python bindings and cross-language validation fixtures.

Clean architecture is applied at real boundaries: DesktopClient separates UI from native calls; a Rust actor owns process I/O and deadlines; Python lifecycle logic is pure; concrete API modules own auth/database/storage. There is no generic repository hierarchy, service locator, plugin framework, extra orchestration runtime or duplicate writer. Arbitrary parent environment variables are filtered before Python starts.

## Assessment versus plan

| Metric | Planned | Actual |
| --- | --- | --- |
| Complexity | XL | XL, three languages and native/database boundaries |
| Confidence | 8/10 before implementation | Local evidence passed; Windows/CI remains unproven |
| Files | 70–90 estimated | See exact inventory below; count includes generated bindings, locks and icon assets |
| Scope | P0-A–F | Source milestone and local checks delivered; external gates explicitly pending |

## Tasks completed

| # | Task | Status | Notes |
| --- | --- | --- | --- |
| 1 | Preserve and inventory | Complete | Legacy migration SHA-256 inventory and both version-order histories recorded |
| 2 | Scaffold and pin | Complete | npm/Cargo/uv locks; Node 24.14.1, Rust 1.95.0, Python 3.12.12 |
| 3 | Shared contracts | Complete | Schema authority, deterministic codegen, parity fixtures |
| 4 | Python worker | Complete | Bounded stdio and pure diagnostic lifecycle |
| 5 | Rust supervisor | Complete | Private actor, ordinary/control queues, correlation, deadlines, cleanup, environment filtering |
| 6 | Desktop and preview | Complete | Narrow commands, validated status, explicit preview, native smoke passed |
| 7 | Scoped fixture API | Complete | Server-derived identities and disabled-provider model capability contract |
| 8 | Reproducible test environment | Complete locally | Seeded teacher/two-student DB and private S3; hosted devices pending |
| 9 | Dev workflow and CI | Complete | Independent commands, source routing, platform build jobs, navigation guide |
| 10 | Review and validation | Local pass complete | CI/Windows execution pending; no claim of packaged P1 acceptance |

## Validation results

| Check | Final result | Evidence |
| --- | --- | --- |
| TypeScript typecheck / ESLint | Pass | Ajv generic validators corrected after initial dependency mismatch |
| Python Ruff / format / mypy | Pass | No reported errors |
| Rust format / full-workspace clippy | Pass | Includes native source; warnings treated as errors |
| Contract generation drift | Pass | Three language outputs reproducible |
| JavaScript command/routing tests | 11 pass | Scoped routing and owned-process termination |
| TypeScript unit/UI tests | 17 pass | Contract rejection, lifecycle status and sanitized errors |
| Python tests | 18 pass | Shared corpus, lifecycle, framing, EOF/private failures |
| Rust unit/contract/process tests | 14 pass | Includes 10 real subprocess/account lifecycle tests |
| DB/private S3 integration | 1 pass | Three identities, spoof rejection, expiry/revocation, repeatable seed, private objects |
| Chromium browser smoke | 1 pass | Lifecycle, profiles, no page errors, narrow viewport |
| UI production build | Pass | Latest corrected dependency set built |
| macOS native development build | Pass | Tauri host compiles; no installer claim |
| npm / Python package audits | Pass | No known third-party vulnerabilities after updates |
| Actual macOS native interaction | Pass | Start/health/stop, profile switch, HMR retains worker, app-close cleanup |
| Remote CI / Windows | Pending | No authorized remote/push or Windows execution in this session |

Total automated tests: **62**. The database test is intentionally excluded from the unit gate and runs explicitly with isolated infrastructure. No test was counted twice for reruns. A local editable project is not a published PyPI dependency and has no package-advisory entry.

Measured observations: Vite ready in 73 ms for preview, 84 ms for native mode; observed native incremental compilation 3.50 seconds. These are this machine's observations, not performance promises. The frontend-only path starts without Docker, credentials or a Python process.

## Issues and corrections

- npm initially selected Ajv 6; pinned Ajv 8.20.0 and used typed validator predicates.
- Tauri required icon metadata and a Result return for the async state-borrowing stop command; both corrected.
- Fixed two Clippy findings and made native exit cleanup explicitly complete before final exit.
- Upgraded Vitest to 4.1.11, pytest to 9.0.3 and datamodel-code-generator to 0.64.0 following audit findings; affected tests/codegen/audits passed afterward.
- Browser connector extension was unavailable. Used a pinned project Playwright browser test instead; no extension installation or browser-profile modification.
- Source review added account-selection tickets to reject superseded identity responses, filtered child environments and bounded database readiness.

## Deviations from plan

- MinIO replaces the LocalStack candidate to exercise actual signed access and anonymous-read denial with a small isolated fixture service.
- P0 diagnostic operations are immediate, so Python uses bounded synchronous stdio. P1 must add asynchronous cancellation before long-running SDK/CUA calls. Rust already provides priority control admission.
- Runtime status events use host revisions; request/response frames use correlation and generation IDs. No fake durable teaching event stream is created in P0.
- Restart is a native UI command. The CLI explains it rather than creating another local listener or arbitrary process killer.
- Small API handlers remain together in the entry module instead of creating one-file wrappers with no independent behavior.
- Generated Python models are available for typed consumers; current dict-based diagnostic dispatch validates against the canonical schema. No handwritten cross-language schema copies were introduced.
- The explicit prp-implement request supplied authorization for its final local validation pass. Remote CI remains pending without a remote/push authorization.

## Next gates

Run configured CI and Windows acceptance when available. P1 then bundles Python/CUA, proves real external-app observation and the teaching overlay, adds the durable action journal and validates native permission/installer behavior. Legacy upgrades and production auth remain separate work; the fixture API is development-only.

## File inventory

All paths below are relative to the rebuild root. Existing source documents were updated; other files were created. Generated assets and lockfiles explain much of the file/line count.

**122 files present: 117 created, 5 updated/archived.** No commit was created.

| File | Action | Lines / asset |
| --- | --- | --- |
| `.claude/PRPs/plans/completed/source-foundation.plan.md` | Updated/archived | 441 |
| `.claude/PRPs/reports/source-foundation-report.md` | Created | 213 |
| `.editorconfig` | Created | 9 |
| `.env.example` | Created | 8 |
| `.gitattributes` | Created | 1 |
| `.github/workflows/foundation.yml` | Created | 113 |
| `.gitignore` | Created | 16 |
| `.node-version` | Created | 1 |
| `.prettierignore` | Created | 8 |
| `.prettierrc.json` | Created | 4 |
| `AGENTS.md` | Updated/archived | 42 |
| `Cargo.lock` | Created | 6374 |
| `Cargo.toml` | Created | 22 |
| `README.md` | Updated/archived | 30 |
| `apps/desktop/AGENTS.md` | Created | 5 |
| `apps/desktop/index.html` | Created | 1 |
| `apps/desktop/package.json` | Created | 17 |
| `apps/desktop/src-tauri/Cargo.toml` | Created | 28 |
| `apps/desktop/src-tauri/build.rs` | Created | 14 |
| `apps/desktop/src-tauri/capabilities/main.json` | Created | 16 |
| `apps/desktop/src-tauri/icons/128x128.png` | Created | 1537 bytes |
| `apps/desktop/src-tauri/icons/128x128@2x.png` | Created | 3068 bytes |
| `apps/desktop/src-tauri/icons/32x32.png` | Created | 495 bytes |
| `apps/desktop/src-tauri/icons/64x64.png` | Created | 771 bytes |
| `apps/desktop/src-tauri/icons/icon.icns` | Created | 37864 bytes |
| `apps/desktop/src-tauri/icons/icon.ico` | Created | 6786 bytes |
| `apps/desktop/src-tauri/icons/icon.png` | Created | 6104 bytes |
| `apps/desktop/src-tauri/icons/source.svg` | Created | 1 |
| `apps/desktop/src-tauri/permissions/autogenerated/account_select.toml` | Created | 11 |
| `apps/desktop/src-tauri/permissions/autogenerated/runtime_health.toml` | Created | 11 |
| `apps/desktop/src-tauri/permissions/autogenerated/runtime_restart.toml` | Created | 11 |
| `apps/desktop/src-tauri/permissions/autogenerated/runtime_start.toml` | Created | 11 |
| `apps/desktop/src-tauri/permissions/autogenerated/runtime_status.toml` | Created | 11 |
| `apps/desktop/src-tauri/permissions/autogenerated/runtime_stop.toml` | Created | 11 |
| `apps/desktop/src-tauri/src/commands.rs` | Created | 80 |
| `apps/desktop/src-tauri/src/config.rs` | Created | 22 |
| `apps/desktop/src-tauri/src/lib.rs` | Created | 58 |
| `apps/desktop/src-tauri/src/lifecycle.rs` | Created | 33 |
| `apps/desktop/src-tauri/src/main.rs` | Created | 3 |
| `apps/desktop/src-tauri/src/manager.rs` | Created | 154 |
| `apps/desktop/src-tauri/src/worker.rs` | Created | 320 |
| `apps/desktop/src-tauri/tauri.conf.json` | Created | 26 |
| `apps/desktop/src-tauri/tests/worker_lifecycle.rs` | Created | 135 |
| `apps/desktop/src/App.tsx` | Created | 29 |
| `apps/desktop/src/features/runtime/RuntimeStatus.test.tsx` | Created | 34 |
| `apps/desktop/src/features/runtime/RuntimeStatus.tsx` | Created | 117 |
| `apps/desktop/src/features/runtime/runtime-state.test.ts` | Created | 23 |
| `apps/desktop/src/features/runtime/runtime-state.ts` | Created | 22 |
| `apps/desktop/src/main.tsx` | Created | 16 |
| `apps/desktop/src/platform/desktop-client.ts` | Created | 12 |
| `apps/desktop/src/platform/preview-client.ts` | Created | 39 |
| `apps/desktop/src/platform/tauri-client.ts` | Created | 18 |
| `apps/desktop/src/styles.css` | Created | 173 |
| `apps/desktop/tsconfig.json` | Created | 14 |
| `apps/desktop/vite.config.ts` | Created | 8 |
| `docs/CODEX-NAVIGATION-GUIDE.md` | Created | 33 |
| `docs/legacy-migration-checksums.json` | Created | 48 |
| `docs/migration-inventory.md` | Created | 22 |
| `docs/rebuild-architecture.md` | Updated/archived | 747 |
| `docs/testing/ci-workflow.md` | Updated/archived | 17 |
| `docs/testing/foundation-environment.md` | Created | 25 |
| `eslint.config.mjs` | Created | 34 |
| `infra/compose.test.yml` | Created | 40 |
| `package-lock.json` | Created | 4455 |
| `package.json` | Created | 58 |
| `packages/contracts/Cargo.toml` | Created | 21 |
| `packages/contracts/codegen/main.rs` | Created | 9 |
| `packages/contracts/package.json` | Created | 10 |
| `packages/contracts/schema/digest.txt` | Created | 1 |
| `packages/contracts/schema/model-access.schema.json` | Created | 29 |
| `packages/contracts/schema/protocol.schema.json` | Created | 533 |
| `packages/contracts/schema/status.schema.json` | Created | 40 |
| `packages/contracts/src/generated-model-access.ts` | Created | 7 |
| `packages/contracts/src/generated-status.ts` | Created | 8 |
| `packages/contracts/src/generated.rs` | Created | 6021 |
| `packages/contracts/src/generated.ts` | Created | 114 |
| `packages/contracts/src/index.ts` | Created | 3 |
| `packages/contracts/src/lib.rs` | Created | 26 |
| `packages/contracts/src/validate.ts` | Created | 20 |
| `packages/contracts/tests/conformance.rs` | Created | 21 |
| `packages/contracts/tests/conformance.test.ts` | Created | 15 |
| `playwright.config.ts` | Created | 15 |
| `rust-toolchain.toml` | Created | 4 |
| `scripts/rebuild/ci-plan.mjs` | Created | 21 |
| `scripts/rebuild/ci-plan.test.mjs` | Created | 26 |
| `scripts/rebuild/cli.mjs` | Created | 159 |
| `scripts/rebuild/contracts.mjs` | Created | 115 |
| `scripts/rebuild/environment.mjs` | Created | 46 |
| `scripts/rebuild/processes.mjs` | Created | 53 |
| `scripts/rebuild/processes.test.mjs` | Created | 15 |
| `scripts/rebuild/verify.mjs` | Created | 106 |
| `services/api/AGENTS.md` | Created | 5 |
| `services/api/Cargo.toml` | Created | 23 |
| `services/api/migrations/0001_foundation.sql` | Created | 12 |
| `services/api/src/auth.rs` | Created | 52 |
| `services/api/src/config.rs` | Created | 92 |
| `services/api/src/db.rs` | Created | 42 |
| `services/api/src/error.rs` | Created | 37 |
| `services/api/src/lib.rs` | Created | 78 |
| `services/api/src/main.rs` | Created | 51 |
| `services/api/src/storage.rs` | Created | 34 |
| `services/api/tests/foundation.rs` | Created | 166 |
| `services/teaching-runtime/.python-version` | Created | 1 |
| `services/teaching-runtime/AGENTS.md` | Created | 5 |
| `services/teaching-runtime/pyproject.toml` | Created | 27 |
| `services/teaching-runtime/src/tro_runtime/__init__.py` | Created | 1 |
| `services/teaching-runtime/src/tro_runtime/__main__.py` | Created | 28 |
| `services/teaching-runtime/src/tro_runtime/digest.txt` | Created | 1 |
| `services/teaching-runtime/src/tro_runtime/generated.py` | Created | 309 |
| `services/teaching-runtime/src/tro_runtime/protocol.py` | Created | 33 |
| `services/teaching-runtime/src/tro_runtime/runtime.py` | Created | 65 |
| `services/teaching-runtime/src/tro_runtime/schema.json` | Created | 533 |
| `services/teaching-runtime/tests/test_protocol.py` | Created | 26 |
| `services/teaching-runtime/tests/test_runtime.py` | Created | 70 |
| `services/teaching-runtime/uv.lock` | Created | 858 |
| `tests/acceptance/foundation.md` | Created | 31 |
| `tests/acceptance/ui.spec.ts` | Created | 37 |
| `tests/fixtures/contracts/corpus.json` | Created | 122 |
| `tests/fixtures/materials/example.md` | Created | 9 |
| `tests/fixtures/workers/adversarial.py` | Created | 34 |
| `tsconfig.base.json` | Created | 19 |
| `vitest.config.ts` | Created | 10 |
