# Implementation report: P1 visual teaching foundation

**Branch:** `codex/plan-native-teaching-foundation`

**Status:** source implementation and local macOS packaging completed; **P1 acceptance remains open**. The plan is not archived and the master specification is not marked complete.

## Result

Tro can discover an external window, obtain bounded accessibility/screenshot evidence,
produce a grounded visual cue, refresh its geometry, and check fresh evidence after a
learner reports trying it. The learner performs every external-app action (F10).

The architecture keeps React on presentation, Rust on native windows/consent/shortcuts,
private account access and process supervision, and Python on session/observation/model
logic. CUA is hidden behind a read-only interface and an immutable bounded native manifest.
The model has **zero callable tools**: it returns a structured visual proposal that Python
validates against a fresh observation. Images remain inside Python/model access and never
enter the frontend protocol or SQLite evidence store.

## Assessment versus plan

| Item | Planned | Actual |
| --- | --- | --- |
| Complexity | XL | XL; native packaging exposed additional shutdown/metadata issues |
| Files | 50–70 | 86 including planning, generated contracts, tests and this report |
| Verification | Consolidated checks, then fix failures | Initial batch, failure-focused reruns and scoped checks for final review corrections |
| Native proof | Windows x64 and macOS arm64 | macOS build/relocation/native-policy load proven; interactive/device acceptance pending |

## Tasks

| # | Task | Outcome |
| --- | --- | --- |
| 1 | Dependencies, manual fixture, policy | Implemented: CUA 0.28.1, Agents SDK 0.22.2, PyInstaller 6.22.3 and locked distributions |
| 2 | Closed v2 contracts | Implemented and generated across TypeScript, Rust and Python; mutation and malformed-cue corpus cases rejected |
| 3 | Cancellable transport/controller | Implemented; reserved control admission, bounded frames/queues, bootstrap cancellation, clean EOF/shutdown and stale-result fences |
| 4 | Read-only CUA | Implemented; exact selected-window manifest, bounded AX/image data, inherited native policy preserved; real native denial tests pass |
| 5 | Permissions and overlay geometry | Implemented for source proof; host consent, click-through/nonactivating windows and grounding checks. Secondary macOS displays withheld pending calibration |
| 6 | Point/click UI and Stop | Implemented; preview, real window selection, observation, repeat and host emergency shortcut. Live click-through/focus proof pending |
| 7 | Session evidence | Implemented; account UUID directories, SQLite transactions, exclusive OS locks, idempotent facts and interrupted-check labels |
| 8 | Drag/type/scroll guidance | Implemented; visual path/ghost pointer, direction/field hints and reduced motion. No input injection |
| 9 | Proof identity/model gateway | Implemented; separate tables/mode, expiring scoped grants, atomic four-call budget, fixed upstream, bounded requests/responses and private desktop configuration |
| 10 | Agents SDK | Implemented; bounded structured-output run, selected-window image/AX input, tracing disabled, fresh grounding. Real provider acceptance pending |
| 11 | Learner reports/checks | Implemented; confirmed/mismatch/unknown with observation source/time. Rendering does not establish an attempt or mastery |
| 12 | Frozen runtime | Proven locally on macOS arm64: relocated path with spaces, no development PATH, private handshake and native read-only policy initialization |
| 13 | Installers | macOS app/DMG built locally. Windows NSIS configured in CI, not executed here. Distribution signing/notarization remains pending |
| 14 | CI/artifact evidence | Existing workflow extended with gated native bundles, pinned artifact upload and no duplicate feature-branch push jobs. Evidence schema/runbook added; remote CI has not run on these uncommitted changes |
| 15 | Installed acceptance | Pending. No manual learner gestures, clean-machine installation, Windows run, signed distribution or paid model run is claimed |

## Validation

All reported source failures were corrected and the affected checks rerun.

| Check | Result |
| --- | --- |
| Contract generation drift | Passed |
| ESLint / TypeScript | Passed |
| TypeScript tests | 33 passed |
| Script tests | 13 passed |
| Browser smoke | 1 passed; preview only |
| Ruff / Python format / mypy | Passed |
| Python tests | 56 covered by the final suite and targeted cancellation regression run |
| Rust format / Clippy | Passed, including full desktop feature on macOS |
| Rust contracts | 2 passed |
| Native geometry | 1 passed |
| Native process/lifecycle | 12 passed, including concurrent start, bootstrap Stop and identity-override rejection |
| API unit / unauthenticated HTTP | 3 passed |
| Database/storage integration | Passed against the isolated fixture services |
| Fake-provider gateway integration | Passed: authentication, duplicate grants, six concurrent requests capped at four calls, expiry/revocation, timeout, oversized response/request and error redaction |
| Dependency audits | npm and Python audits reported no known vulnerabilities; local `tro-runtime` is not a PyPI package |
| UI build / macOS release build | Passed |
| Frozen runtime relocation / native policy load | Passed |

The automated checks cover 123 test cases across the languages/suites. Shared corpus
cases intentionally run in more than one language. Browser/source tests do not establish
native overlay correctness or learning outcomes.

## Local artifact

- App: `target/release/bundle/macos/Tro Rebuild Development.app`
- DMG: `target/release/bundle/dmg/Tro Rebuild Development_0.1.0_aarch64.dmg`
- Runtime manifest: `apps/desktop/src-tauri/resources/runtime/manifest.json`
- Signing inspection: linker ad-hoc signature, no TeamIdentifier, no sealed resources.
  This is a local proof artifact, not a signed/notarized distribution.
- DMG SHA-256: `adf0615c18d3f9e680e7375a30413debd3dffb2c8d9d7b09aeec6aab9b83289c`.

## Implementation refinements

1. Teaching requests return complete revisioned projections instead of several partial
   event types. Rust uses a 250 ms read-only refresh loop while a cue is active; generation
   and presentation epochs prevent a stopped/superseded request from restoring an overlay.
2. CUA uses bounded mode with a selected-window manifest rather than replacing a user's
   policy file. The SDK's canonical tool inventory is not its authorization result;
   native dispatch denial is tested with impossible process targets.
3. The SDK proposes structured output with **no callable tools**. Observation, grounding,
   presentation and checks stay in the controller, making the F10 boundary smaller.
4. Screenshots are bounded and private to Python/model access. Heartbeat refresh uses AX
   only; images and raw screen content are excluded from durable session facts.
5. Existing CI owns native package jobs rather than adding a second competing workflow.
6. Secondary macOS display overlays are disabled until coordinate calibration is verified;
   reconnecting displays currently requires restarting Tro.

## Issues found and fixed

- A daemon reading buffered stdin could abort Python at interpreter shutdown. Raw OS-pipe
  reads and a regression keeping parent stdin open now prove clean exit.
- Concurrent Start calls regressed during lifecycle-lock removal. A separate bootstrap
  lock preserves idempotent Start while Stop remains independent.
- Request payloads could otherwise replace protocol identity fields. The worker rejects
  those fields before writing to the child.
- Frozen Agents SDK imports require transitive package metadata (including MCP). The
  PyInstaller specification now collects it, and relocation/self-check succeeds.
- Overlay native operations are scheduled on the main thread; hidden WebViews are created
  during setup to avoid the documented Windows synchronous-handler creation deadlock.
- Native text is truncated by UTF-8 bytes, keeping worst-case Unicode projections inside
  the protocol frame budget without corrupting characters.

## Remaining acceptance work

- Interactive macOS and Windows observation/click-through/focus-retention tests using the
  manually operated fixture and one ordinary native app.
- Mixed-DPI/multiple-monitor calibration, move/minimize/close behavior and measured Stop
  latency on installed builds; secondary macOS display support remains incomplete.
- Clean installation/reinstallation without development tools, permission attribution and
  revocation/relaunch behavior, nested signing and notarization as applicable.
- Provision an authorized proof backend/account and run real model guidance through it.
- Run remote CI on the final submitted revision and attach actual platform evidence using
  `tests/acceptance/native-proof-evidence.schema.json`.

No branch was pushed, no PR was created, no release was published, and no provider calls
or user configuration changes were made by this implementation run.

## Changed files

Generated bindings are maintained through the schema generator. The list includes the
pre-existing planning changes in this feature branch.

| File | Change | Lines |
| --- | --- | --- |
| `.github/workflows/foundation.yml` | Updated | +22 / -2 |
| `.gitignore` | Updated | +2 / -0 |
| `AGENTS.md` | Updated | +4 / -2 |
| `Cargo.lock` | Updated | +83 / -0 |
| `README.md` | Updated | +7 / -0 |
| `apps/desktop/src-tauri/Cargo.toml` | Updated | +4 / -2 |
| `apps/desktop/src-tauri/build.rs` | Updated | +4 / -0 |
| `apps/desktop/src-tauri/capabilities/main.json` | Updated | +5 / -4 |
| `apps/desktop/src-tauri/src/commands.rs` | Updated | +40 / -1 |
| `apps/desktop/src-tauri/src/config.rs` | Updated | +43 / -0 |
| `apps/desktop/src-tauri/src/lib.rs` | Updated | +29 / -5 |
| `apps/desktop/src-tauri/src/manager.rs` | Updated | +119 / -5 |
| `apps/desktop/src-tauri/src/worker.rs` | Updated | +24 / -2 |
| `apps/desktop/src-tauri/tauri.conf.json` | Updated | +2 / -1 |
| `apps/desktop/src-tauri/tests/worker_lifecycle.rs` | Updated | +41 / -0 |
| `apps/desktop/src/App.tsx` | Updated | +3 / -1 |
| `apps/desktop/src/main.tsx` | Updated | +6 / -1 |
| `apps/desktop/src/platform/desktop-client.ts` | Updated | +2 / -0 |
| `apps/desktop/src/platform/preview-client.ts` | Updated | +2 / -0 |
| `apps/desktop/src/platform/tauri-client.ts` | Updated | +28 / -1 |
| `apps/desktop/src/styles.css` | Updated | +102 / -0 |
| `docs/CODEX-NAVIGATION-GUIDE.md` | Updated | +5 / -5 |
| `docs/rebuild-architecture.md` | Updated | +147 / -54 |
| `package.json` | Updated | +3 / -1 |
| `packages/contracts/schema/digest.txt` | Updated | +1 / -1 |
| `packages/contracts/schema/protocol.schema.json` | Updated | +1256 / -11 |
| `packages/contracts/src/generated.rs` | Updated | +14024 / -2058 |
| `packages/contracts/src/generated.ts` | Updated | +244 / -12 |
| `packages/contracts/src/index.ts` | Updated | +8 / -2 |
| `packages/contracts/src/lib.rs` | Updated | +1 / -1 |
| `packages/contracts/src/validate.ts` | Updated | +10 / -1 |
| `scripts/rebuild/cli.mjs` | Updated | +40 / -4 |
| `services/api/src/db.rs` | Updated | +11 / -8 |
| `services/api/src/lib.rs` | Updated | +1 / -0 |
| `services/api/src/main.rs` | Updated | +3 / -0 |
| `services/teaching-runtime/pyproject.toml` | Updated | +19 / -2 |
| `services/teaching-runtime/src/tro_runtime/__main__.py` | Updated | +214 / -15 |
| `services/teaching-runtime/src/tro_runtime/digest.txt` | Updated | +1 / -1 |
| `services/teaching-runtime/src/tro_runtime/generated.py` | Updated | +578 / -2 |
| `services/teaching-runtime/src/tro_runtime/runtime.py` | Updated | +2 / -2 |
| `services/teaching-runtime/src/tro_runtime/schema.json` | Updated | +1256 / -11 |
| `services/teaching-runtime/tests/test_runtime.py` | Updated | +86 / -1 |
| `services/teaching-runtime/uv.lock` | Updated | +472 / -0 |
| `tests/fixtures/contracts/corpus.json` | Updated | +3097 / -9 |
| `tests/fixtures/workers/adversarial.py` | Updated | +3 / -0 |
| `.claude/PRPs/plans/native-teaching-foundation.plan.md` | Created | +491 |
| `apps/desktop/src-tauri/capabilities/overlay.json` | Created | +10 |
| `apps/desktop/src-tauri/permissions/autogenerated/observation_permissions.toml` | Created | +11 |
| `apps/desktop/src-tauri/permissions/autogenerated/overlay_current.toml` | Created | +11 |
| `apps/desktop/src-tauri/permissions/autogenerated/proof_connect.toml` | Created | +11 |
| `apps/desktop/src-tauri/permissions/autogenerated/teaching_request.toml` | Created | +11 |
| `apps/desktop/src-tauri/src/account.rs` | Created | +147 |
| `apps/desktop/src-tauri/src/geometry.rs` | Created | +86 |
| `apps/desktop/src-tauri/src/overlay.rs` | Created | +234 |
| `apps/desktop/src-tauri/src/permissions.rs` | Created | +43 |
| `apps/desktop/src-tauri/tauri.package.conf.json` | Created | +20 |
| `apps/desktop/src/features/teaching/OverlayWindow.tsx` | Created | +88 |
| `apps/desktop/src/features/teaching/TeachingOverlay.test.tsx` | Created | +35 |
| `apps/desktop/src/features/teaching/TeachingOverlay.tsx` | Created | +81 |
| `apps/desktop/src/features/teaching/TeachingPanel.tsx` | Created | +301 |
| `apps/desktop/src/platform/preview-teaching.ts` | Created | +130 |
| `apps/desktop/src/platform/teaching-client.ts` | Created | +19 |
| `docs/native-teaching-runbook.md` | Created | +57 |
| `scripts/rebuild/package-check.mjs` | Created | +120 |
| `scripts/rebuild/package-runtime.mjs` | Created | +72 |
| `services/api/migrations/0002_proof_model_access.sql` | Created | +20 |
| `services/api/src/model_gateway.rs` | Created | +494 |
| `services/teaching-runtime/packaging/entry.py` | Created | +30 |
| `services/teaching-runtime/packaging/runtime.spec` | Created | +19 |
| `services/teaching-runtime/src/tro_runtime/agent.py` | Created | +79 |
| `services/teaching-runtime/src/tro_runtime/guidance.py` | Created | +101 |
| `services/teaching-runtime/src/tro_runtime/model_client.py` | Created | +28 |
| `services/teaching-runtime/src/tro_runtime/observation_source.py` | Created | +199 |
| `services/teaching-runtime/src/tro_runtime/observations.py` | Created | +68 |
| `services/teaching-runtime/src/tro_runtime/resources/read-only.json` | Created | +8 |
| `services/teaching-runtime/src/tro_runtime/session_store.py` | Created | +134 |
| `services/teaching-runtime/src/tro_runtime/teaching.py` | Created | +228 |
| `services/teaching-runtime/tests/test_agent.py` | Created | +52 |
| `services/teaching-runtime/tests/test_guidance.py` | Created | +66 |
| `services/teaching-runtime/tests/test_read_only_policy.py` | Created | +58 |
| `services/teaching-runtime/tests/test_session_store.py` | Created | +47 |
| `services/teaching-runtime/tests/test_teaching.py` | Created | +113 |
| `tests/acceptance/native-foundation.md` | Created | +26 |
| `tests/acceptance/native-proof-evidence.schema.json` | Created | +27 |
| `tests/fixtures/native/gestures.html` | Created | +19 |
