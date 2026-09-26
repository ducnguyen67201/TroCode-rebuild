# P1 native acceptance

Historical P1/F10 acceptance: Tro renders pixels and the tester performs every
external-app gesture manually. F11 now uses the same observation-only ceiling;
see `push-to-talk-guidance.md` for voice and typed instruction acceptance.
Build/source checks cannot substitute for this record. Current status: **pending**.

For each macOS arm64 and Windows x64 installation record:

- Commit and installer SHA-256; runtime manifest digest; OS/version/architecture.
- Signing identity/notarization or explicit unsigned/ad-hoc status.
- Clean install without Python/Node; launch/relaunch and legacy-data preservation.
- Actual Screen Recording/Accessibility permission attribution and denied/revoked results.
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

The current overlay expires after one second without fresh evidence and omits secondary macOS displays until
coordinate calibration is established. This is an explicit acceptance limitation, not
proof of mixed-DPI support. No signing, device interaction or provider spend is implied
by running source checks or package:check.
