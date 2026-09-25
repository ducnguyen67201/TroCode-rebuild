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
[`push-to-talk-computer-control.md`](../../tests/acceptance/push-to-talk-computer-control.md)
on macOS arm64 and Windows x64. Source and preview tests cannot prove passive
global hooks, microphone/privacy behavior, AltGr handling, native input delivery,
provider latency, perceived responsiveness, signing or clean-machine cleanup.
