# P1 native acceptance

Historical P1/F10 acceptance: Tro renders pixels and the tester performs every
external-app gesture manually. This file does not establish F11 direct-control
acceptance; see `push-to-talk-computer-control.md`.
Build/source checks cannot substitute for this record. Current status: **pending**.

For each macOS arm64 and Windows x64 installation record:

- Commit and installer SHA-256; runtime manifest digest; OS/version/architecture.
- Signing identity/notarization or explicit unsigned/ad-hoc status.
- Clean install without Python/Node; launch/relaunch and legacy-data preservation.
- Actual Screen Recording/Accessibility permission attribution and denied/revoked results.
- Explicit settings buttons open the exact macOS Screen Recording, Accessibility and Microphone destinations and the Windows Microphone destination.
- The floating settings guide remains nonactivating and click-through; all toggles, + actions and Tro.app drag/drop actions are learner-performed.
- First authenticated run shows the device checkup before Learn; the local Tro pointer identifies only a Tro control and disappears before every OS prompt.
- Deny required screen access, follow the displayed manual recovery path, recheck, and relaunch where requested. Repeat after revoking access outside Tro.
- Grant/probe microphone once and confirm no sample, file, transcript, device name or raw error is retained or logged.
- Reset microphone consent and complete the full Learn journey with **Use text instead**.
- Confirm there is no camera prompt, `NSCameraUsageDescription`, camera entitlement or camera code path.
- On Windows, confirm onboarding says capture is available/unknown and that the first selected-window Observe—not onboarding—proves access.
- Re-enter **Settings → Device permissions** after changing OS privacy state and confirm a fresh recheck reflects it.
- Manually open `tests/fixtures/native/gestures.html` in a separate browser.
- Select it in Tro. Observe, show each click/drag/type/scroll cue repeatedly without acting.
- Record that real pointer/focus/counter/text/drag/scroll remain unchanged by Tro.
- Manually perform the gesture through the overlay, then choose I tried it.
- Record a reported attempt without action and an ambiguous/incomplete observation.
- Verify Stop and Command/Ctrl+Shift+Escape during capture, model request and bootstrap.
- Move/minimize/close the target and exercise every monitor/DPI combination.
- Crash/reopen/check/account-switch: evidence must stay isolated and never invent completion.
- Repeat an observation/cue in a normal native application, not only the fixture.
- Verify actual authenticated model guidance through the isolated proof server.
- With a worker running, use the explicit app relaunch control and confirm the worker is stopped/reaped before the new process starts.

The current overlay expires after one second without fresh evidence and omits secondary macOS displays until
coordinate calibration is established. This is an explicit acceptance limitation, not
proof of mixed-DPI support. No signing, device interaction or provider spend is implied
by running source checks or package:check.

Record current runs as version 3 evidence. Version 1 and 2 files remain readable as
historical records but cannot satisfy current acceptance. Keep macOS arm64 and Windows
x64 records separate, using the actual release signing identity: changing the identity
can change macOS privacy attribution.
