# Contracts, requests and lifecycle

`packages/contracts/schema/protocol.schema.json` is the wire authority. The generator
produces TypeScript, Rust and Python bindings plus a schema digest. Both sides must
match the digest before teaching begins. The current development protocol is v2;
this unreleased extension changes the digest, so incompatible workers fail handshake.

## A request's path

React TeachingClient → Tauri command → RuntimeManager → Worker pipe → Python
parser/scheduler → TeachingSession → validated response → Rust overlay and React.
Model SDK imports are deferred until the first planning request, keeping worker
bootstrap independent of model initialization. Messages carry request, correlation
and worker-generation identifiers. Rust rejects
payload attempts to override the envelope. Python bounds frames and queue sizes.

`TEACHING_REQUESTS` in `__main__.py` is an immutable routing set, not model tools or
OS permission grants. Teaching requests use a bounded ordinary queue; lifecycle
Stop/shutdown use a separate reserved control queue that cancels active work.

| Request | Owner and effect |
| --- | --- |
| `listTargets`, `selectTarget` | Discover/select an observation window; clear the prior plan |
| `observe` | Fresh selected-window evidence |
| `ask` | Replace current plan using one bounded SDK planning call |
| `refreshCue` | Local plan checks and cue grounding; legacy manual cue refresh when no plan exists |
| `planControl` | Pause, resume or explicitly confirm an uncertain step |
| `explain` | Manual single-cue construction; leaves planned mode |
| `check` | Explicit expected-value check for manual guidance |
| `presentationAck` | Presentation acknowledgement, never learning evidence |
| `configure` | Host-private storage/model initialization; excluded from UI's request allowlist |

## State and stale results

TeachingSession owns state. Replies contain complete revisioned projections,
including an optional journey with captions, current index, status and message.
Model grants, semantic postconditions and image bytes are not exposed in that journey.
Rust emits refreshed projections to the main window and validated cues to overlays.
React filters updates by active session and increasing revision. Overlay presentation
uses host epochs, and worker results are checked against active generation/status.

A journey keeps refreshing while running or awaiting confirmation, including gaps
where no cue can be safely shown. Completed/paused journeys stop tracking. Cues expire
within one second, and null/invalid cues hide previous pixels immediately. Native
operations execute on the main thread; the observation loop remains asynchronous.

Account changes and Stop invalidate old work. Shutdown cancels pending work, closes
local resources and reaps the worker. No interrupted native input can be replayed,
because native input is not part of the product interface.
