# Native teaching foundation

The master specification owns P1 and F10. This implementation provides an observation
adapter, visual cue preview/overlay, separate evidence facts, an optional bounded model
connection, and explicit runtime packaging. P1 acceptance is still open until the
manual Windows/macOS record in `tests/acceptance/native-foundation.md` is complete.

## Development

Use `npm run setup`, then `npm run dev`. After authentication, complete the device
checkup or revisit **Settings → Device permissions**. Open the offline gesture fixture yourself in a browser. Find windows,
select the fixture and Observe. The manual guidance controls let you select an observed
control and visualize a gesture. Repeat obtains fresh evidence. It never clicks, types,
scrolls, drags, focuses or launches another application.

The native overlay is nonactivating and click-through. It expires after one second without fresh evidence; a host loop re-observes geometry every 250 ms while guidance remains valid.
Command/Ctrl+Shift+Escape is the host emergency stop. A shortcut conflict prevents launch
rather than silently removing Stop. Secondary macOS monitor overlays remain disabled
pending calibration. Restart Tro after changing the connected display set. Capture protection and permission attribution require actual device
verification. No result from an animation establishes learning or submission.

For permission-only development QA without hosted authentication, launch the Tauri
development app with `?nativePermissionQa=1` in its `devUrl`. This route exists only
when Vite compiles in development mode and still uses the real native permission
bridge for device checks and settings actions. Completing onboarding automatically
opens the normal Learn page with local preview workspace/runtime data, so no hosted
account or classroom data is required for this QA route.

## Device permission checkup

Tro checks without prompting when the checkup opens. An OS prompt or exact settings
destination is opened only after the learner presses the matching Tro button. The
illustrated Tro pointers are presentation only. The settings guide is nonactivating and
click-through: it can explain the visible path and add/drag fallback, but never moves,
clicks, toggles, drags or drops the real pointer and never annotates an OS-owned consent
prompt. On macOS the guide is placed beside the visible System Settings app list when
the window bounds are available, with its arrow pointing back toward that list. It does
not claim to detect an individual toggle or row.

On macOS, **Open Screen Recording settings** deep-links to **System Settings → Privacy
& Security → Screen & System Audio Recording**. Turn Tro on; if it is absent, use **+ →
Applications → Tro.app → Open**, or drag Tro.app from Applications into the app list.
Return to Tro and choose **I changed it — recheck**, then repeat for **Accessibility**.
Optional microphone recovery opens **Privacy & Security → Microphone**. Use **Relaunch
Tro** when shown. Test with the actual release identity, because a signing or bundle
identity change can create a different TCC entry.

On Windows 11, screen readiness is a support preflight. The first selected-window
Observe remains authoritative, and protected/elevated windows can still be unavailable.
For optional microphone recovery, use **Settings → Privacy & security → Microphone**
and confirm both microphone access and **Let desktop apps access your microphone**.
Tro opens that exact page only after the learner chooses **Open Microphone settings**;
the learner performs every change. **I changed it — recheck** repeats the bounded,
discarded-sample probe so Tro does not infer a grant from presentation alone. Windows
screen access still uses the secure picker when Observe begins and has no onboarding
app-list toggle.

For a clean consent test, reset/revoke the relevant OS privacy decisions before launch.
Those resets affect other tests and may require a relaunch. Verify both optional paths:
one successful microphone request/probe and one complete **Use text instead** journey.
The probe discards callback buffers immediately. There is no camera request, screen
video recording, audio file, transcript or upload in this milestone.

## Optional proof server

Use a separate PostgreSQL database named `tro_rebuild_proof`, with no legacy `users`
table. Set `TRO_API_MODE=proof`, `DATABASE_URL`, and a loopback `TRO_API_BIND` behind a
trusted HTTPS reverse proxy. Run the API `migrate` command. Provision `proof_accounts`
and hashed, expiring `proof_sessions` explicitly; fixture accounts cannot authorize this
server. For `serve`, supply `OPENAI_API_KEY` and `TRO_MODEL` on the server only.

On the desktop create owner-only `~/.tro-rebuild/proof-account.json` on macOS, or
`%LOCALAPPDATA%/.tro-rebuild/proof-account.json` on Windows:

```json
{
  "origin": "https://your-proof-server.example",
  "token": "64-hex-character-proof-account-token"
}
```

Use Connect proof account, then Start. Rust verifies identity and obtains a five-minute,
four-request model grant for the teaching session. Only that grant reaches Python.
Each response is capped at 1024 output tokens. No native tools or hosted provider tools
are available. The SDK returns a structured visual proposal; the controller re-observes
and validates its target before presentation. Screen text and selected-window images are untrusted evidence. Images remain inside Python/model access and are never persisted in the session store.

Account evidence lives under the corresponding `.tro-rebuild/evidence/accounts/<uuid>`
folder. It stores identifiers and labeled facts, not screenshots, raw AX content or keys.
Interrupted checks remain interrupted; reconnecting starts a fresh teaching session.

## Packaging

`npm run package:runtime` freezes Python, CUA, schemas and policy using PyInstaller.
`npm run package:check` verifies the manifest and a relocated private-pipe handshake
without development PATH, then initializes the native read-only policy. `npm run package` stages the runtime and builds the native
installer locally. Signing and notarization use the platform's Tauri environment inputs;
an unsigned/ad-hoc artifact does not establish distribution readiness.

Live native capture, read-only policy, permissions, clean installation, model access and
manual gestures must be measured separately. Do not publish an installer as P1 complete
until those records exist for both target platforms.

## F11 voice/control setup

Hosted mode additionally requires backend-only `OPENAI_API_KEY` and fixed
`TRO_TRANSCRIPTION_MODEL=gpt-transcribe`. The action model is compiled as
`gpt-5.6-sol`; environment configuration cannot change it. The desktop receives
only short-lived subject-bound grants. After authentication, voice auto-arms
once per app session and reports any required microphone or
keyboard-monitoring/accessibility recovery in the main window.

Debug desktop builds permit the private model gateway at the exact
`http://127.0.0.1` loopback host used by `make dev`. Release builds do not set
that runtime capability and continue to require HTTPS.
Debug terminal diagnostics identify only the bounded failure stage and code
(capture start, selected-window observation, transcription transport, action
setup or model transport); they never include audio, transcript, screen content,
typed values, grants or provider response bodies.
The voice HUD has a transparent document root and is hidden on every terminal
state, including failure, so it cannot leave a persistent strip above other apps.

Daily use is exactly Command+Control on macOS or physical Left Control+Left Alt
on Windows. Hold to record and release either key to finalize. Windows AltGr and
extra modifiers do not activate. The frontmost eligible window at chord-down is
the entire observation scope. Use Command-or-Control+Shift+Escape if capture or guidance
work must stop immediately.

Transcription uses overlapping completed WAV files rather than a live session.
Provider/network failure, an incomplete chunk sequence, silence, target change or
queue pressure fails closed. Tro stages only click-through visual demonstrations;
the learner performs every real action. Diagnose with IDs, phases, durations and counts only—
never record audio, transcripts, screenshots, accessibility labels or typed text.
Complete `tests/acceptance/push-to-talk-guidance.md` on both packaged
platforms before claiming F11/P7-A/P8 acceptance.
