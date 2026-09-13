# Plan: P1 packaged visual teaching foundation

## Summary

Build a Windows/macOS app that observes a selected external window, shows where and how to click, drag, scroll or type, waits for the learner to act, and checks fresh evidence. **Tro never performs the external-app action.** CUA is read-only; the OpenAI Agents SDK produces grounded explanations and visual cues through a small observation/presentation interface.

This revised execution plan is subordinate to `docs/rebuild-architecture.md`, especially founder clarification **F10**. It supersedes this plan's earlier automated-demonstration/dispatcher/journal proposal. Those features are removed, not hidden behind an approval button or deferred feature flag. No corresponding action implementation exists in P0.

## User Story

As a learner, I want Tro to show me where and how to act in my application, so that I can perform the task myself and receive guidance based on what I actually did.

## Problem → Solution

P0 offers diagnostic process start/health/stop. P1 adds read-only native observation, a teaching pointer independent of the real mouse, visual gesture guidance, a bounded agent explanation loop, account-scoped session evidence and packaged runtime support. The full adaptive multi-step teaching journey remains P2.

## Metadata

- **Complexity:** XL, 15 tasks across three complete milestones.
- **Source PRD:** `docs/rebuild-architecture.md`; F01/F03/F04/F07/F09/**F10**, F08 verification order and C08 account isolation groundwork.
- **Phase:** P1-A observe/point; P1-B visual gestures/agent/learner checks; P1-C packaged proof.
- **Baseline:** merged PR #1, commit `675df3eb30a2e1b175692dd6bfc05e6e375f4852`; final PR revision `1917680` passed source, integration and Windows/macOS build/worker CI. 64 local automated tests. Packaged acceptance remains unproven.
- **Estimated implementation files:** 50–70 including tests, generated files and lockfiles; do not create empty modules to meet the estimate.
- **Confidence:** 7/10 overall. Native permission attribution, geometry and installed runtime loading need real-device evidence.
- **Status:** implementation in progress; source validation and local packaged-runtime proof are recorded in the implementation report. Full native acceptance remains open.
- **Execution boundary:** local implementation and validation; no provider spending, publishing, signing credentials or manual learner input performed by the agent.

## Agreed Boundary

The learner performs every external-app click, drag, scroll and keystroke. Tro may highlight controls, draw arrows, animate a ghost pointer/path and explain the gesture. These effects render only inside a transparent click-through overlay. They cannot move the real pointer, focus another app, inject input, open resources automatically or complete a task for the learner.

Do not add raw `call_tool`, CUA action methods, SDK ComputerTool, arbitrary shell, app launching, browser automation/CDP attachment, an execute/demonstrate command or a native mutation journal. Enforce observation-only access in both the application interface and CUA native policy. Teacher resource delivery remains possible inside Tro, but later classroom functionality cannot bypass F10 to control another app.

Ordinary explicit operations in Tro's own UI—select a lesson, request help, change account or submit selected work—remain governed by their normal application permissions. F10 is a boundary on operating the learner's external application, not a ban on all writes inside Tro.

## UX Design

### Before

```text
Select development profile → diagnostic Start / Health / Stop
```

### After

```text
Connect account → permissions → select external window → Observe
                                                           ↓
                                       Grounded explanation + visual cue
                                                           ↓
                              Learner clicks / drags / scrolls / types
                                                           ↓
                           I tried it → fresh observation → feedback

Repeat = replay visual guidance only       Stop = hide cues and stop observing
```

| Touchpoint | Behavior | Boundary |
| --- | --- | --- |
| Click cue | Highlight the target and optionally pulse a ghost click | No OS click or real pointer movement |
| Drag cue | Mark source/destination, draw a path and animate a ghost pointer | Learner holds and drags their own mouse; no injected drag |
| Type cue | Highlight the field and explain what to enter | No typing, paste or clipboard write |
| Scroll cue | Show direction and the relevant region | No wheel/scroll event |
| Repeat | Replay the current visual cue if geometry is still valid | No native action; stale cue requires a fresh observation |
| I tried it | Record a learner report and obtain a fresh selected-window observation | Report is not proof of completion |
| Feedback | Distinguish confirmed change, mismatch and insufficient evidence | Animation completion/model completion never proves learning |
| Stop | Hide overlay immediately and cancel observation/model work | Always available during a pending request |

Captions are plain text, 1–3 short sentences, maximum 400 characters, in selected `en` or `vi`. Include reduced motion, readable contrast and adequate caption wrapping. Keep all controls in the main Tro window; overlay remains fully click-through and nonactivating. No voice dependency.

Use an offline fixture opened **manually** in an external browser: uniquely titled page with an accessible click counter, drag source/drop destination, labeled text field and scroll area. Include explicit reset controls operated by the tester. Do not automate its interactions through the product. Record a second manual observation/cue example in a normal native app; a fixture does not establish arbitrary-app compatibility.

## Mandatory Reading and Discovery

Line references describe the merged P0 baseline; revised instructions/spec take precedence.

| Category | File:lines | Pattern / implication |
| --- | --- | --- |
| Requirements | `docs/rebuild-architecture.md`; root/nested `AGENTS.md` | F10 observation-only boundary, F08 milestone-first checks |
| Ownership | `docs/CODEX-NAVIGATION-GUIDE.md:1–33` | Concrete modules, explicit dependencies, no second orchestrator |
| Entry/lifecycle | `apps/desktop/src-tauri/src/lib.rs:8–57` | Targeted events and guarded asynchronous exit cleanup |
| State/concurrency | `apps/desktop/src-tauri/src/manager.rs:10–154` | Account tickets and host status revisions; startup currently holds lifecycle lock |
| Transport | `apps/desktop/src-tauri/src/worker.rs:53–175,203–320` | 32 ordinary/4 control slots, deadlines, 256 KiB frames; matched replies only |
| Native commands | `apps/desktop/src-tauri/src/commands.rs:6–80` | Narrow command surface and host-only credentials |
| Release config | `apps/desktop/src-tauri/src/config.rs:1–22`; `tauri.conf.json` | Debug `.venv` path; release refused and bundle inactive |
| Python entry/state | `services/teaching-runtime/src/tro_runtime/__main__.py:1–28`; `runtime.py:8–65` | Synchronous diagnostic input, pure transitions, protocol-only stdout |
| Shared types | `packages/contracts/schema/{protocol,status,model-access}.schema.json` | Closed variants, diagnostic handshake and disabled model capability |
| Codegen | `scripts/rebuild/contracts.mjs:36–110`; `packages/contracts/src/lib.rs:1–26` | Protocol generates TS/Rust/Python and digest; other schemas currently TS-only |
| Presentation | `apps/desktop/src/features/runtime/RuntimeStatus.tsx:11–49,58–105` | Client injection, latest revisions, stop remains accessible |
| Naming/adapters | `apps/desktop/src/platform/{desktop-client,tauri-client,preview-client}.ts` | Native/preview implementations; TS/wire camelCase, Rust/Python snake_case |
| Errors | `worker.rs:13–25`; `services/api/src/error.rs:8–36`; `runtime.py:22–29` | Stable public codes and sanitized messages |
| Logs | `services/api/src/lib.rs:35–48` | Correlation ID, elapsed time/status, no content |
| Auth/data | `services/api/src/auth.rs:16–50`; `db.rs:1–42`; `lib.rs:18–34,62–78` | Bound SQL, token hashes, server-derived identity, concrete handlers |
| API config | `services/api/src/config.rs:11–57`; `migrations/0001_foundation.sql` | Debug fixture-only guard; three constrained account profiles |
| Tests | `apps/desktop/src-tauri/tests/worker_lifecycle.rs:1–135`; Python `tests/test_runtime.py:12–70` | Pure tests plus real/adversarial subprocesses with deadlines |
| Integration | `services/api/tests/foundation.rs`; `tests/acceptance/ui.spec.ts` | Isolated DB/private storage; browser preview is not native-overlay proof |
| Tooling | root/component manifests, `scripts/rebuild/{cli,verify,ci-plan}.mjs`, `.github/workflows/foundation.yml` | Pinned dependencies, independent dev paths, package currently refuses |

### Five traces

1. **Entry:** React intent → DesktopClient → named Rust command → RuntimeManager/Worker → validated Python command. New commands only select/observe/explain/check/present.
2. **Data:** images and AX content remain bounded in Python; only target geometry, captions and session state reach React. Model input goes through an authenticated backend.
3. **State:** Rust owns process generation/presentation; Python owns teaching session and evidence; account switch invalidates both before resolving new credentials.
4. **Contracts:** generate types/digests from JSON Schema. Events need explicit direction and sequence validation. A render acknowledgement is only a display fact.
5. **Patterns:** one Python session controller with real/fake observation and model boundaries. Small modules, no native mutation dispatcher, generic repository hierarchy or DI framework.

## External Documentation and Version Evidence

Checked during P1 planning on 2026-09-13; dependencies have not been installed for this task.

| Topic | Source | Implication |
| --- | --- | --- |
| CUA topology | [Integration choices](https://cua.ai/docs/concepts/choose-a-cua-driver-integration) | Same-process CUA inside existing Python worker; no extra daemon |
| Window observation | [In-process guide](https://cua.ai/docs/how-to-guides/driver/use-sdk-in-process), [SDK reference](https://cua.ai/docs/reference/cua-driver/sdk-reference) | Use typed read methods and explicit shutdown; do not expose action APIs |
| Published CUA | [0.28.1 package metadata](https://pypi.org/pypi/cua-driver/0.28.1/json) | Published wheel source contains typed window APIs; matching dylib/DLL is included |
| Policy/privacy | [Permission policies](https://cua.ai/docs/reference/cua-driver/permission-policies), [Telemetry](https://cua.ai/docs/reference/cua-driver/telemetry) | Native read-only allowlist plus preserved administrator restrictions; process-local telemetry/update controls |
| Native permissions | [macOS permissions](https://cua.ai/docs/reference/cua-driver/macos-permissions), [Platform support](https://cua.ai/docs/reference/cua-driver/platform-support) | Host owns prompts/relaunch; capture needs installed proof |
| Agent/model | [Tools](https://openai.github.io/openai-agents-python/tools/), [Models](https://openai.github.io/openai-agents-python/models/), [Tracing](https://openai.github.io/openai-agents-python/tracing/), [0.22.2 metadata](https://pypi.org/pypi/openai-agents/0.22.2/json) | Product observation/presentation tools, injected client, tracing off |
| Overlay | [Tauri config](https://v2.tauri.app/reference/config/#windowconfig), [Shortcut plugin](https://v2.tauri.app/plugin/global-shortcut/) | Nonactivating click-through windows and Rust-owned stop shortcut |
| Packaging | [Tauri resources](https://v2.tauri.app/develop/resources/), [PyInstaller operation](https://pyinstaller.org/en/stable/operating-mode.html), [ctypes assets](https://pyinstaller.org/en/stable/feature-notes.html#ctypes-dependencies), [6.22.3 metadata](https://pypi.org/pypi/pyinstaller/6.22.3/json) | Preserve onedir assets/layout; build per target OS |
| Signing | [Tauri macOS signing](https://v2.tauri.app/distribute/sign/macos/) | Nested native assets and outer bundle require signing evidence |

**KEY_INSIGHT:** CUA docs include a warning about typed APIs after 0.25, but inspected published 0.28.1 bindings expose `list_apps`, `list_windows` and `get_window_state`. **APPLIES_TO:** exact version pin. **GOTCHA:** wrapper and native library must remain matched.

**KEY_INSIGHT:** `_native.py` loads `libcua_driver_sdk.dylib`/`cua_driver_sdk.dll` adjacent to the package source. **APPLIES_TO:** PyInstaller hook. **GOTCHA:** venv import success does not prove installed loading.

**KEY_INSIGHT:** permission preflight and actual capture readiness differ. **APPLIES_TO:** onboarding and installed tests. **GOTCHA:** do not assume terminal permission covers the signed app.

**KEY_INSIGHT:** a model can call only the product tools it is given, while CUA policy provides a second boundary. **APPLIES_TO:** observation-only enforcement. **GOTCHA:** a model request or visual animation must never reach a native input API.

Verified read imports: `CuaDriver`, `ListAppsInput`, `ListWindowsInput`, `GetWindowStateInput`, configured authorization options and `DriverError`. `GetWindowStateInput` requires explicit keyword arguments for pid/window_id/session/query/accessibility/screenshot/output path/element/depth/dimension limits. `WindowStateOutput` provides snapshot ID, degraded/truncated/elements-complete flags, screenshot dimensions/scale/validity, bounds and bounded elements with frames. Pass `screenshot_out_file=None`; no screenshot persistence by default. No `ClickInput`, action token executor or other mutation imports belong in product code.

Planning pins: `cua-driver==0.28.1`, `openai-agents==0.22.2`, `pyinstaller==6.22.3`. Inspected CUA wheel hashes: macOS universal2 `6578fd45fb710394751736c7231f69ee428aa5d23b41b72090e97a0c9291dd2d`; Windows x64 `c61aaa28592df04308ed41c52318da825c739ba47128c64e9018dd118200ab43`. Resolve remaining transitive/platform dependencies into locks during implementation. No research cache is required to use this plan.

## Patterns to Mirror

### Naming and error handling

`services/teaching-runtime/src/tro_runtime/runtime.py:17–20` preserves identity fields:

```python
reply = {
    key: request[key]
    for key in ("protocolVersion", "requestId", "correlationId", "generationId")
}
```

Use stable public codes/messages like `WorkerError::new(code, message)` (`worker.rs:19–25`); never forward private upstream exception strings. Keep snake_case internally and camelCase on the wire.

### Logging

Actual pattern from `services/api/src/lib.rs:47`:

```rust
tracing::info!(event = "request.completed", correlation_id = %id, elapsed_ms = start.elapsed().as_millis() as u64, status = response.status().as_u16());
```

Add content-free session/cue/check IDs, never screenshots, AX text, captions, tokens or provider bodies. Python stdout remains protocol only; SDK tracing is disabled.

### Data/service boundary

Use `sqlx::query(...).bind(...).execute(&mut *tx)` as in `auth.rs:46–48`. Python SQLite also uses bound parameters. DesktopClient has actual native/preview implementations; define an ObservationSource test boundary with fake/real implementations, not a generic computer-control abstraction.

### Tests

Mirror Python `test_runtime.py:53–63`: encoded frames through a real subprocess, parsed outputs and a deadline. Mirror `worker_lifecycle.rs`: adversarial child modes, generation/account races, hangs and EOF. Add a read-only policy integration test and an observer fake that fails immediately on any mutation method access.

## Architecture Decisions

```text
React main controls ── DesktopClient ── Rust host / worker actor
React overlay ← validated cue ← Rust per-monitor OverlayController
                                      │ private pipes
                                      ▼
                       Python session + Agents SDK
                         │                    │
               read-only CUA adapter     session/evidence SQLite
                         │
                  external-window observation

Python model client → authenticated Rust gateway → model provider
Learner's real mouse/keyboard → external application (never via Tro)
```

### Protocol and cancellation

Upgrade the private protocol to v2 and regenerate all bindings/digests. Reject incompatible pairs before loading CUA. Keep process RuntimeStatus separate from session states `preparing`, `observing`, `explaining`, `waiting`, `checking`, `stopping`, `stopped`, `failed`.

| Command → reply | Payload/meaning |
| --- | --- |
| `initialize → ready` | Host-derived account/storage namespace and optional private model grant; negotiate observation/presentation capabilities |
| `start → started` | sessionId and locale; observation-only session |
| `listTargets → targetsResult` | Bounded native window identities mapped to opaque targetIds |
| `selectTarget → targetSelected` | Current targetId; clear old observations/cues/checks |
| `observe → observationResult` | Fresh selected-window metadata; no image bytes over desktop IPC |
| `explain → explanationResult` | observationId + bounded request; grounded visual cue |
| `check → checkResult` | cueId + learner report; fresh observation, confirmed/mismatch/unknown result |
| `presentationAck → presentationAckResult` | cueId + rendered status; display evidence only |
| `health`, `stop`, `shutdown` | Preserve lifecycle pairs and reserved control admission |
| Events | `runtime.sessionChanged`, `runtime.cueChanged`, `runtime.checkChanged`, `runtime.permissionChanged` |

All command names use the `runtime.` prefix added by Worker.request. Replies retain request/correlation IDs; events instead carry current generationId/sessionId/eventSeq. No execute, click, drag, type or demonstrate command exists. Gesture kinds are **display data**, not executable commands.

Keep 256 KiB frames, 100 targets, 200 projected elements, 256-character window titles and 400-character captions. Images stay in bounded Python memory (4 MiB decoded cap) and are resized for bounded model input. Empty/invalid geometry produces explicit unavailable status.

Replace synchronous stdin dispatch with one asyncio controller, portable daemon input-reader thread, bounded ordinary admission and reserved stop/shutdown/output capacity. One writer serializes frames. EOF revokes observation/model work and exits. Rust handles events separately from pending replies; stale generations/sequences cannot revive a cue. Release lifecycle locks before long observation/model waits. Change a cancellation epoch immediately on stop/account switch; bootstrap and late results must check it before publication.

Rust hides overlays immediately on stop, then cancels worker/model work, performs bounded graceful shutdown and force-kills/reaps if needed. Register `CommandOrControl+Shift+Escape` in Rust; expose shortcut conflicts and keep main-window Stop usable. Proposed measured targets: cue hidden within 250 ms, worker reaped within existing 8-second lifecycle budget. Do not claim these targets already passed.

### Read-only CUA enforcement

Expose an `ObservationSource` with list_windows/observe_selected/close behavior. Never expose driver objects, generic tool calls or input methods to agent functions. Construct configured STANDARD-only CUA, with no unrestricted acknowledgement; cap session TTL to 600 seconds and idle TTL to 120 seconds.

Install a product native policy allowing only `list_apps`, `list_windows`, `get_window_state` and any separately verified read-only permission/lifecycle operation needed by the pinned adapter. Everything else is denied by default. Validate policy/schema/tool names against the pinned runtime during implementation. A denial must remain a denial; no fallback to desktop control, automation scripting or browser attachment.

Preserve administrator/user `CUA_DRIVER_MANAGED_POLICY_FILE` and `CUA_DRIVER_POLICY_FILE` restrictions. Compose the product ceiling with inherited restrictions rather than overwriting them; use the native runtime's bounded configuration or policy intersection when necessary. Missing/unreadable configured restrictions fail closed. No policy paths/overrides come from model/UI messages.

Set `CUA_DRIVER_RS_TELEMETRY_ENABLED=false` and `CUA_DRIVER_RS_UPDATE_CHECK=false` before driver construction; do not call recording/history tools or alter global CUA preferences. Enforce read-only behavior beyond UI hiding: tests must prove click/drag/type/scroll/focus/launch requests are denied at the adapter and native policy boundaries.

### Geometry and visual gestures

Observation records bind exact PID/window ID, target-selection generation, snapshot ID, freshness and source geometry metadata. Reject reused/missing targets until reselection; window titles alone are not identity. Normalize element/frame units explicitly. Rust converts to each monitor's physical bounds and overlay-local logical pixels; test negative origins, mixed DPI and cross-display paths. Do not assume one global scale.

A `TeachingCue` contains cueId, observationId, targetId, locale, caption and a closed visual gesture: `point`, `click`, `drag`, `type`, `scroll`. Point/click/type have a grounded rectangle; drag has grounded source/destination and bounded path points; scroll has grounded region/direction. Validate all endpoints against the current observation; model-invented coordinates are rejected. A ghost pointer is an SVG/CSS element, never an OS cursor API.

Keep one overlay per intersected monitor, nonactivating, always-on-top, skipped from taskbar, `set_ignore_cursor_events(true)` and `set_focusable(false)`. Use targeted `emit_to` and separate main/overlay capabilities; privileged commands additionally validate the window label. Overlay can acknowledge rendering only. Set content protection where supported and verify selected-window capture excludes overlay; do not assume it does.

While a cue is visible, bound content-free geometry checks to 250 ms and hide stale geometry after at most 1 second without confirmation. No continuous model/screenshot polling. Moved/minimized/closed targets invalidate cues; content changes are re-observed through Observe/I tried it or bounded post-check only. Reduced motion displays a static source/path/destination. Repeat is purely visual and must leave real pointer, focus and application state unchanged.

### Session and evidence persistence

Store account-scoped `sessions.sqlite3` in host-derived app-local data, never resources/legacy directories. Use stdlib SQLite, parameterized transactions, schema versioning, restrictive permissions and exclusive per-account ownership (`fcntl`/`msvcrt`). Keep one store, not a second SDK session database.

Persist session context and bounded facts: cue presented/acknowledged, learner reported attempt, check pending, fresh observation result and explicit feedback. Each has its own ID/source/timestamp. A learner report is not proof, a rendered cue is not an attempt, and a matching postcondition is not proof of mastery or attribution of who caused a change. No native action states, execution grants or mutation journal.

Crash/stop hides cues and marks unfinished checks interrupted/unknown. Explicit resume first re-observes; no old animation or completion is automatically replayed. Deduplicate check IDs and reject changed intent under the same ID. Do not persist raw screenshots/AX dumps by default. Preserve legacy unknown-action records only as historical read-only data if ever migrated; never execute them.

### Agent and model access

Use one Agent/Runner in the Python session with function tools for observe selected window, present a grounded visual cue and check a learner-reported attempt. No built-in computer tool, shell or generic CUA tool surface. “Show me how to drag this” produces a visual path and explanation, never a drag action, even if the model asks to execute one.

Inject `OpenAIResponsesModel` with `AsyncOpenAI` pointing at the application gateway. Use nonstreaming Runner.run, `max_turns=4`, `parallel_tool_calls=False`, 1,024 output tokens per request, finite timeouts and zero automatic client retries. Disable `set_tracing_disabled(True)`/RunConfig tracing and sensitive data export. Validate structured target IDs/gesture output against the current observation. Screen content is untrusted data, not policy.

Backend grants here authorize **model requests only**, never computer control. Host obtains a short-lived account/session-bound grant and passes it privately to Python. Rust gateway implements `/v1/runtime-grants` and bounded nonstreaming `/v1/responses`: hashed grants, 5-minute expiry, revocation, atomic four-call/token reservation, allowed model/function-tool subset, `store=false`, no hosted tools/background mode/previous_response_id, 8 MiB request and 2 MiB response caps. Fixed server-configured provider origin, redirects disabled, no caller-supplied URL/key. Stop cancels work and requests grant revocation; already-admitted model usage may still be billed.

Keep P0 fixtures intact. Add explicit proof-server mode and separate `tro_rebuild_proof` database with append-only `0002_runtime_grants.sql` creating proof_accounts/proof_sessions/runtime_grants. Do not edit published 0001 or repurpose its three constrained profiles. Branch auth by explicit mode; update migration guard for the selected database while retaining legacy refusal, and readiness to verify all migration checksums. Provision/revoke proof credentials using server CLI and owner-only files. Packaged Rust reads private app-config credentials and trusted HTTPS origin; frontend gets only account identity/status.

This is engineering proof authentication, not production SSO/classroom authorization. Real model evidence requires an authorized backend/account/provider key; fake upstream tests can proceed independently. No deployment/provider spend is authorized by this planning revision.

### Packaging and permissions

Start with PyInstaller 6.22.3 onedir, collecting CUA adjacent native libraries, Python/SDK metadata, schemas/digest and dynamic imports explicitly. Build on macOS arm64 and Windows x64; other architectures need separate evidence. Stage the complete executable + `_internal` layout as Tauri resources and resolve installed absolute paths. Existing Rust actor spawns only that fixed executable; no shell plugin or system-Python fallback. Produce a file/hash/version manifest and validate staged assets.

Retain debug `.venv` development and Vite-only preview. Enable macOS app/DMG and Windows NSIS with clean-machine WebView2 setup. Keep rebuild ID/data directory distinct from legacy. Direct distribution remains D06's candidate; macOS transparent-window private API means no App Store compatibility claim.

Rust owns consent/settings/relaunch UI, on the host main thread. Request only permissions required for observation/presentation; an OS permission that also permits control does not authorize Tro to use that ability. Distinguish permission preflight from fresh capture, and test responsible host/helper identity after installed-app relaunch. No shared daemon fallback.

Sign nested executable/libraries before the outer bundle with authorized inputs. Ad-hoc/unsigned engineering builds do not satisfy signed clean-distribution proof. Real interactive Windows/macOS devices, authorized signing material and real model access are explicit acceptance dependencies; do not pretend hosted compilation or fake models satisfy them.

## Files to Change

| Files (relative to repo) | Action / purpose |
| --- | --- |
| `packages/contracts/schema/{protocol,status,model-access}.schema.json`; generated TS/Rust/Python/digests; validators/corpus | Update/generate: read-only commands, visual gestures and evidence |
| `scripts/rebuild/contracts.mjs` | Extend generation for new shared schemas; avoid handwritten duplicates |
| Python `src/tro_runtime/{__main__,runtime,protocol}.py` | Update async lifecycle/transport |
| Python `src/tro_runtime/{observation_source,observations,guidance,session_store,agent,model_client}.py` | Create cohesive read-only native, presentation, persistence and SDK modules |
| Python tests for runtime/protocol/observations/guidance/session_store/agent/read_only_policy | Create/update real/fake boundary and regression tests |
| Runtime `pyproject.toml`, `uv.lock`; resource read-only policy | Pin dependencies and immutable product ceiling |
| Native `src/{config,commands,manager,worker,lifecycle,lib}.rs` | Update events, cancellation and installed configuration |
| Native `src/{overlay,geometry,permissions,account}.rs`; tests | Create native presentation, mapping, consent and credentials |
| Native manifests/build/config/platform entitlements; Cargo.lock; main/overlay capabilities | Update/create platform packaging and narrow IPC ACL |
| Frontend platform clients; `features/teaching/{TeachingPanel,TeachingOverlay}.tsx`, visual/state tests; App/main/styles | Add cue/gesture/check UI and explicit preview |
| API `src/{lib,config,auth,db,error,main}.rs`; `model_gateway.rs`; new migration/tests | Add model-only grants and isolated proof mode |
| Runtime `packaging/{runtime.spec,entry.py,hooks/hook-cua_driver.py}` | Create frozen entry/assets hook |
| `scripts/rebuild/{package-runtime,package}.mjs`, cli/ci-plan/verify, package.json, ignore rules | Explicit packaging and scoped verification |
| `.github/workflows/{foundation,native-proof}.yml` | Native package artifacts and explicit evidence gates |
| `tests/fixtures/native/gestures.html`; adversarial workers | Controlled manually operated target and failure fixtures |
| `tests/acceptance/{native-foundation.md,native-proof-evidence.schema.json}`; native runbook/README/navigation/spec | Evidence and operator/dev instructions |

**Not building:** native mutation adapter/dispatcher/journal, automatic demonstrations/setup, browser automation, app launching/focus changes, full adaptive lesson planner, teacher dashboard/material ingestion, voice, games, production login, updater rollout or published release.

## Step-by-Step Tasks

Finish each complete milestone's implementation, test code and source review before its applicable verification batch. Generation is an implementation operation; do not run the full gate after each edit.

### P1-A — Read-only observation and pointing

### Task 1: Pin dependencies and create the manual fixture
- **ACTION:** Add planning pins to runtime/build groups and lock all distributions.
- **IMPLEMENT:** Offline click/drag/type/scroll fixture with unique title, accessible controls/postconditions and manual reset. Add product read-only policy resource.
- **MIRROR:** Exact P0 dependency pins and isolated fixtures.
- **IMPORTS:** CUA read/config types, Agents SDK; never action types.
- **GOTCHA:** An installed SDK having action methods does not justify exposing them.
- **VALIDATE:** Locked imports and native-policy configuration load; fixture is usable manually offline.

### Task 2: Add v2 read-only and visual contracts
- **ACTION:** Extend schemas and regenerate bindings/digest.
- **IMPLEMENT:** Command catalog above, bounded cue geometry/gesture kinds, separate evidence sources, private initialization, direction validation and eventSeq.
- **MIRROR:** Existing schema/codegen/shared corpus.
- **IMPORTS:** Existing contract validators in all languages.
- **GOTCHA:** Visual `drag` data must not become an executable native command; update diagnostic-only capability equality deliberately.
- **VALIDATE:** Cross-language valid/invalid corpus; reject execute/click-input requests and invented geometry before side effects.

### Task 3: Add cancellable transport and session controller
- **ACTION:** Implement async admission and Rust event projection.
- **IMPLEMENT:** Reserved control capacity, bounded queues/writer, EOF cleanup, cancellation fence and no long-held lifecycle mutex. Ignore stale account/generation results.
- **MIRROR:** P0 actor, deadlines and account tickets.
- **IMPORTS:** asyncio/threading; Tokio sync/time; existing Worker/Manager.
- **GOTCHA:** Late capture/model completion cannot restore an overlay after stop.
- **VALIDATE:** Hung capture/model, partial frames, queue overload, bootstrap stop and account races reap within budget.

### Task 4: Implement read-only observation adapter and policy
- **ACTION:** Build ObservationSource with list/observe/close only.
- **IMPLEMENT:** One CUA object, exact-window selection, bounded snapshots, geometry/permission/degradation errors, composed restrictive native policy and telemetry off.
- **MIRROR:** Boundary validation and pure state logic.
- **IMPORTS:** CUA read types, dataclasses/typing/time; no mutation imports.
- **GOTCHA:** Incomplete AX projections do not prove uniqueness; no whole-desktop or automation fallback when a window read fails.
- **VALIDATE:** Adapter contract tests and native policy denial tests prove all input/focus/launch methods are unavailable; capture actual selected fixture.

### Task 5: Implement native permissions and overlay mapping
- **ACTION:** Add permissions/geometry/overlay modules in Rust.
- **IMPLEMENT:** Host consent/relaunch, per-monitor normalization, nonactivating click-through windows, restricted overlay ACL and stale expiry.
- **MIRROR:** Targeted emit and public errors.
- **IMPORTS:** Tauri Manager/Emitter/WebviewWindowBuilder/physical geometry and narrowly scoped platform bindings.
- **GOTCHA:** Test permission attribution, capture exclusion and actual units; ambiguous geometry must hide the cue.
- **VALIDATE:** Mixed DPI/negative-origin vectors plus actual native capture/point/click-through/focus-retention checks.

### Task 6: Wire point/click guidance and finish P1-A
- **ACTION:** Add TeachingPanel/TeachingOverlay and preview client states.
- **IMPLEMENT:** Select/Observe, grounded highlight/caption, ghost click, Repeat and Stop; register Rust emergency shortcut and report conflicts.
- **MIRROR:** DesktopClient injection, latest revisions and independent stop enablement.
- **IMPORTS:** React/contracts/DesktopClient; host shortcut plugin only.
- **GOTCHA:** No “demonstrate once” or execute button. Ghost click changes only overlay pixels.
- **VALIDATE:** Full P1-A batch after review; without touching input, repeated cues leave counter, pointer and focus unchanged; learner clicks through to change counter.

### P1-B — Visual gestures, agent guidance and learner evidence

### Task 7: Persist session and guidance evidence
- **ACTION:** Add session_store with account-isolated SQLite and exclusive ownership.
- **IMPLEMENT:** Separate presented/acknowledged/reported/observed/check facts, versioned schema, bounded metadata, interrupted checks and explicit fresh resume.
- **MIRROR:** Bound SQL, transactions and immutable history.
- **IMPORTS:** sqlite3/pathlib/UUID/time, fcntl or msvcrt for process lock.
- **GOTCHA:** No native dispatch/outcome states or second SDK session DB; do not infer attempts from animations.
- **VALIDATE:** Crash/reopen, lock contention, disk failure, duplicate check IDs and account-isolation tests preserve evidence labels without invented completion.

### Task 8: Implement visual drag/type/scroll guidance
- **ACTION:** Add pure cue validation and SVG/CSS gesture rendering.
- **IMPLEMENT:** Grounded drag source/path/destination, type-field highlight and scroll direction/region; reduced motion and local replay, with stale invalidation.
- **MIRROR:** Observation normalization and injected preview presentation.
- **IMPORTS:** Shared TeachingCue types, pure geometry helpers and React/SVG; no OS input API.
- **GOTCHA:** The learner holds the real mouse button for dragging; an animated ghost pointer never injects drag events.
- **VALIDATE:** Render/path bounds tests; live fixture unchanged through repeated animations, then changes only when the tester manually acts.

### Task 9: Add proof accounts and model-only gateway
- **ACTION:** Implement isolated proof-server mode, credentials and expiring model grants.
- **IMPLEMENT:** Separate proof tables, mode-specific migration/auth guard, complete checksum readiness, bounded Responses gateway, atomic budget limits and private host config.
- **MIRROR:** Existing SQLx auth, ApiError/correlation and fixed-origin reqwest.
- **IMPORTS:** Existing Rust dependencies and concrete model_gateway module.
- **GOTCHA:** Model grants cannot authorize native actions; preserve P0 fixture guard and unchanged 0001 migration.
- **VALIDATE:** Fake-upstream tests cover revoked/expired/cross-account grants, limits, timeouts, key redaction and arbitrary origin/hosted-tool rejection.

### Task 10: Connect the SDK to observation and visual guidance
- **ACTION:** Add agent/model_client with injected real/fake models.
- **IMPLEMENT:** Observe/present/check tools only, bounded nonstreaming run, tracing off, structured target/gesture validation and cancellation.
- **MIRROR:** Pure runtime tests with real I/O at boundaries.
- **IMPORTS:** Agent/Runner/RunConfig/ModelSettings/function_tool/RunContextWrapper/OpenAIResponsesModel, AsyncOpenAI.
- **GOTCHA:** “Do it for me” or malicious on-screen instructions cannot add a native tool; reply with guidance rather than execute.
- **VALIDATE:** Fake model attempts to invoke native click/drag/type/focus remain denied; valid requests produce only grounded visual cues. Real-model proof is separate.

### Task 11: Add I tried it/check and finish P1-B
- **ACTION:** Show fresh learner-action feedback and uncertainty.
- **IMPLEMENT:** Learner report triggers bounded fresh observation; compare explicit fixture postconditions, label confirmed/mismatch/unknown, preserve source/time and offer guidance. Stop/account change clears stale UI.
- **MIRROR:** Existing revision-checked React state/publicError.
- **IMPORTS:** Check/session/cue contracts and DesktopClient.
- **GOTCHA:** A matching snapshot does not establish mastery or who made the change. Rendering and model-run completion cannot mark a task complete.
- **VALIDATE:** Complete P1-B review/batch with manual click/drag/type/scroll, reported-without-change, ambiguous evidence and crash during check; native mutation count stays zero.

### P1-C — Packaged native proof

### Task 12: Bundle Python/CUA
- **ACTION:** Add PyInstaller spec/hook, staging manifest and installed WorkerProgram.
- **IMPLEMENT:** Onedir native assets/metadata/policy/schema, fixed resource paths and useful missing/corrupt-asset errors; no system Python fallback.
- **MIRROR:** Explicit CLI commands and fail-closed release config.
- **IMPORTS:** PyInstaller hooks, Node fs/path/crypto, Tauri resource resolution.
- **GOTCHA:** Adjacent DLL/dylib layout and read-only policy must survive relocation.
- **VALIDATE:** Relocated bundle under path with spaces, without dev PATH; handshake/import/read-policy self-check and missing/wrong-arch asset failures.

### Task 13: Produce Windows/macOS installers
- **ACTION:** Enable platform bundle configuration and consent/signing inputs.
- **IMPLEMENT:** macOS app/DMG, Windows NSIS/WebView2, stable separate rebuild identity, nested signing order and private writable state outside resources.
- **MIRROR:** Separate dev/build/package paths and legacy preservation.
- **IMPORTS:** Native packaging/signing tooling and Tauri config only.
- **GOTCHA:** Observation permissions may be broad at OS level; runtime policy remains read-only. Ad-hoc build is not signed distribution proof.
- **VALIDATE:** Clean install/reinstall, exact host capture identity, no Python/Node prerequisite, no changed legacy state.

### Task 14: Extend scoped CI and native evidence tooling
- **ACTION:** Add explicit package scripts/workflow and evidence schema.
- **IMPLEMENT:** Source gates before native bundles, pinned actions/artifacts, docs-only routing and PR/main checks without duplicate feature pushes. Add package:runtime/package:check/package commands.
- **MIRROR:** Existing ci-plan tests, aggregated failures and process ownership.
- **IMPORTS:** Existing Node utilities and platform tools.
- **GOTCHA:** Hosted compilation is not a live click-through/manual-gesture test. Do not provision external/paid runners implicitly.
- **VALIDATE:** Routing tests; manifest/self-check artifact records; evidence requires exact OS/arch/hash/signing state and manual learner interaction results.

### Task 15: Run packaged acceptance and update canonical status
- **ACTION:** Complete P1-C review then applicable batch and real-device checks.
- **IMPLEMENT:** Record observation→visual gesture→manual learner action→fresh check→stop on both target platforms, permissions/DPI/restart/account behavior and actual model guidance.
- **MIRROR:** P0 evidence/report discipline, F08 and F10.
- **IMPORTS:** Fixture, manifest and evidence schema.
- **GOTCHA:** Fake models, animated videos or native builds do not prove the installed experience; unmet device/signing/backend gates stay explicit.
- **VALIDATE:** Every acceptance criterion has evidence, especially zero native mutation calls; update D06 only as supported by installed results.

## Testing Strategy

| Area | Cases | Expected outcome |
| --- | --- | --- |
| Read-only authority | Model/UI/native requests for click, drag, type, scroll, focus, launch | Denied at boundaries; zero effects; no approval-mode bypass |
| Protocol | Bad version/direction/fields, oversize, malformed geometry/IDs | Bounded rejection before observation/presentation |
| Geometry | Mixed scale, negative origin, moved/closed window, stale snapshot | Correct local overlay geometry or hidden cue |
| Visual gestures | Repeated ghost click/drag/type/scroll, reduced motion | Pointer/focus/external state unchanged until learner acts |
| Transport | Saturation, partial frame, hung observation/model, stale events | Stop remains available; no late overlay resurrection; bounded reap |
| Evidence | Animation only, reported without change, ambiguous snapshot, duplicate check | Distinct labels; no invented completion/mastery |
| Store | Crash/reopen, two writers, disk failure, account switch | Isolated history; interrupted checks; explicit fresh resume |
| Gateway | Grant revocation/expiry, budget race, upstream hang, oversized body | Bounded error, no secrets/content in logs |
| Packaging | Missing libraries/policy, wrong arch, path spaces, clean machine | Explicit failure or real installed read-only proof, never fallback |

Use pure tests, subprocesses and read-only native-policy checks; live learner effects are performed manually on the fixture. Testing can use an independent harness to measure pointer/focus/effect counters, but cannot add a product native-action adapter. Do not claim browser preview tests establish OS behavior.

## Validation Commands

After each full milestone and source review, run the applicable subset through CI (F08). No full checks attached to save or dev restart.

```bash
npm run contracts:check
npm run lint
npm run typecheck
npm run test:scripts
npm run test:typescript
npm run test:browser
(cd services/teaching-runtime && uv run --locked ruff check . && uv run --locked ruff format --check . && uv run --locked mypy src && uv run --locked pytest)
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
npm run test:integration
npm run build
npm audit
uv run --project services/teaching-runtime --locked pip-audit
```

Keep P0's `--no-default-features` selection on Linux source jobs; full native checks run on Windows/macOS. Expect zero applicable errors, no codegen drift and passing new boundary tests. Commands to add in task 14 (not currently available): `npm run package:runtime`, `npm run package:check`, and an implemented `npm run package`. Package check verifies staged assets/read-only policy/handshake without model access or user input; package builds installers without publishing or repeating the full source gate.

## Acceptance Criteria and Manual Checklist

- [ ] Install macOS arm64 and Windows x64 bundles without Python/Node development tools; verify actual host permission/capture readiness and signing status.
- [ ] Select the manually opened fixture window; show accurately grounded click, drag, type and scroll cues.
- [ ] Replay each cue with no learner input: real pointer, focus, counter, drag item, text field and scroll position remain unchanged.
- [ ] Learner manually clicks/drags/types/scrolls through the overlay; I tried it obtains fresh evidence and labels the result correctly.
- [ ] Report an attempt without performing it; Tro does not infer success from the report or animation.
- [ ] Move/minimize/close the target and test mixed-DPI/multiple displays; stale or invalid cues disappear.
- [ ] Stop during capture/model/check; overlay disappears within measured bound and worker reaps without late events.
- [ ] Crash during check, restart and switch accounts; no false completion, stale animation or cross-account evidence.
- [ ] Native policy and application/agent contracts reject every mutation path, including explicit requests to do the task for the learner.
- [ ] Actual SDK/model explanation works through authenticated backend with bounded usage and no private tracing.
- [ ] Applicable checks pass on final revision and both packaged-platform evidence gates are complete; otherwise report the exact pending dependency.

## Completion Checklist

- [ ] F10 appears consistently in code boundaries, schemas, tool definitions, UI, tests and docs.
- [ ] No execute/demonstrate command, native mutation method, hidden automation flag, action dispatcher or mutation journal.
- [ ] Concrete modules and real test interfaces; no extra orchestration framework.
- [ ] Errors/logs follow existing privacy/concurrency patterns; generated files are reproducible.
- [ ] Session evidence preserves source and uncertainty; UI does not grade or submit on animation/model completion.
- [ ] Master spec, operator/native runbook and README reflect measured outcomes.

## Risks and Prerequisites

| Risk | Impact | Mitigation |
| --- | --- | --- |
| CUA's broad native capability leaks through adapter/policy | Violates core product requirement | Closed read-only interface + native deny-by-default policy + direct denial tests |
| Mixed-DPI/AX geometry ambiguity | Misleading gesture | Explicit normalization/fixture calibration; hide uncertain geometry |
| Drag path mistaken for a real cursor | Confusing guidance | Distinct ghost pointer/style and caption; no input APIs; reduced motion |
| Packaged permission attribution/native assets | Blocks installed capture | Early read proof, matched wheel and relocated signed bundle test |
| Snapshot misread as learning evidence | False progress | Separate report/observation/render facts and explicit uncertainty |
| Missing interactive Windows/signing/backend access | Blocks full acceptance | Independent source/fake tests proceed; live gates remain unclaimed |
| Scope grows into full P2/production auth | Delays native proof | Small fixture and bounded observe/explain/check loop |

## Handoff

Implement tasks 1–6 as P1-A, review the complete milestone, then run its applicable batch. Continue P1-B and P1-C in order; passing P1-A alone does not complete P1. Real backend/accounts, authorized signing inputs and interactive target machines are execution prerequisites whose availability is not yet established.

The learner—not Tro—performs every external-app action. Future plans and archived P0 material must not reintroduce automated setup/demonstrations as an exception to F10.
