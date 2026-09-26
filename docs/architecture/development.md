# Development and acceptance

## Extending a feature

1. Map the change to requirements and phase acceptance in the master spec.
2. Put pure decisions in their owning Python module, side effects in adapters,
   native lifecycle/presentation in Rust and UI behavior in React.
3. Change the canonical schema for cross-process data, then run
   `npm run contracts:generate`. Never hand-edit generated bindings.
4. Add meaningful boundary/progression tests and review the complete change.
5. Run the applicable verification batch after implementation, fix collected
   failures together, and use focused reruns for corrections.

Use `npm run dev:ui` for explicit browser simulation and `npm run dev` for native
work. The preview cannot establish real desktop progress. Manual guidance tools
are collapsed in the teaching panel so the planned flow remains prominent.

`make dev` is the single native development entry point. Doppler remains the outer
process so every child receives the selected development configuration. Vite reloads
React and CSS, Tauri rebuilds desktop Rust, and the Tauri watcher also restarts the
desktop-owned Python worker when `services/teaching-runtime/src` changes. A small
Node supervisor rebuilds the Rust API once per source change and then restarts both
local API modes from the same debug binary, avoiding competing Cargo builds. Changes
to dependencies, migrations or generated contracts still require an explicit setup,
migration or generation command; the watcher does not infer those operations.

Debug desktop builds register `tauri-plugin-log` with a terminal-only target for
`tro::guidance`. Guidance traces pair the gateway's sanitized model request/output
shape with the runtime's closed grounding reason. Release builds do not register
this logger, and the trace never includes transcript text, screenshot bytes,
accessibility labels or values, typed content, captions or provider reasoning.

## Evidence

Python progression tests cover automatic advancement, stable transitions, stale or
partial evidence, semantic rebinding, duplicate controls, pause/resume, preexisting
outcomes, forbidden gestures and the one-planner-call/local-refresh boundary.
Shared contract fixtures cover validation in each language. Native lifecycle tests
cover worker cancellation and identity fencing. UI tests must exercise state delivery
and controls rather than infer native success from rendering.

For actual installed validation, use [the native runbook](../native-teaching-runbook.md)
and [acceptance checklist](../../tests/acceptance/native-foundation.md). Test a learner
performing consecutive steps, control movement, missing targets, Stop, permission
revocation and model errors. Record real evidence; source checks are not platform proof.

P1's existing limits remain: secondary macOS overlays are disabled pending calibration,
Windows/clean-machine validation and signing remain open, and no real provider run is
claimed. Packaged artifacts must be rebuilt after this source/protocol change.

F11 packaged validation is separate: use
[`push-to-talk-guidance.md`](../../tests/acceptance/push-to-talk-guidance.md)
on macOS arm64 and Windows x64. Source and preview tests cannot prove passive
global hooks, microphone/privacy behavior, AltGr handling, overlay click-through,
provider latency, perceived responsiveness, signing or clean-machine cleanup.

To add an Instructor Cursor capability, update the master requirement, the explicit
Python tool method/schema, plan validation, renderer/accessibility text and
unit/native acceptance tests. The Responses credential relay treats SDK-owned tool
schemas as opaque; it is not a second tool registry. Do not use dynamic discovery or
a generic native dispatcher.
