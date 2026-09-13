# Planned guidance implementation

This extends PR #2 with the first bounded P2 planning/progression slice. P1 native
acceptance and the broader P2 teaching journey remain open. The earlier native
foundation report describes the original P1 revision, not this extension.

## Behavior

- One local SDK call prepares one to three semantic steps.
- The local controller rebinds each cue to fresh selected-window evidence.
- Two fresh matching observations after a nonmatching baseline advance the step
  without another model call. Uncertain results offer learner-confirmed Continue.
- Pause, resume, Stop and explicit replanning are exposed in the teaching panel.
- One automatic replan per learner request can recover a persistently missing
  target. It receives the objective, completed guidance and a fresh observation.
  Failed recovery pauses; observation/permission failures never trigger model retries.
- Observed and learner-reported advancement are separate local evidence facts.
- Full revisioned state updates reach React; old-session and old-revision updates
  cannot restore stopped guidance.

## Cleanup and documentation

Removed the unused single-cue model proposal path. Manual cue construction remains
an explicit collapsed tool for native proof. Planning types and deterministic
progression live in `planning.py`; TeachingSession remains the single coordinator.
The architecture guide starts at `docs/architecture/README.md` and covers module
ownership, local/remote boundaries, request routing, progression, development and
acceptance. The master specification remains canonical.

## Verification

The consolidated contracts/UI/Python batch passed, including 38 TypeScript tests,
13 script tests, browser smoke, contract drift and two Rust contract tests, Ruff,
mypy, formatting and UI build. Python's initial full suite passed 69 tests; the
additional recovery and deferred-import regressions are covered by focused reruns.
The source suite now contains 71 Python cases. macOS desktop Clippy and 12 worker
lifecycle tests passed. npm and Python dependency audits found no known vulnerabilities.
The local project itself is not auditable through PyPI.

The updated frozen runtime was rebuilt and checked for relocation, handshake and
native policy loading. The previous full app/DMG is not evidence for this changed
schema/runtime; final installer builds are delegated to PR CI.

The previous PR CI revision passed Linux source/integration but failed Windows
worker handshakes; macOS was cancelled by matrix fail-fast after successfully
building/checking its frozen runtime. SDK imports were moved out of bootstrap
and into the first planning request to reduce startup work. A subprocess regression
ensures handshake imports do not load Agents/OpenAI. Matrix fail-fast is disabled
so each platform supplies independent evidence. Confirmation on Windows requires
the updated remote run.

## Limits

Plans are in memory, limited to three steps, and currently check exact accessibility
values. There is no material ingestion, durable plan resume, long-horizon lesson
planner, model-based visual checker or mastery inference. Semantic labels must be
unique; uncertain/missing evidence fails closed. Native installed acceptance,
Windows/mixed-DPI validation, signing and real-provider proof remain pending.
