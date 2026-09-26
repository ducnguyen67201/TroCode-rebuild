# F11 push-to-talk guidance acceptance

Status: **source implementation and packaged native evidence are tracked separately**.
Complete this checklist on signed macOS arm64 and Windows x64 builds. Never
record audio, transcript text, screenshots, captions, accessibility values or
typed values.

## Build record

- Date, commit and installer SHA-256:
- OS, version, architecture and keyboard layout:
- Signing/notarization status:
- Runtime and contract digests:
- Provider models and pricing date (names only; never credentials):

## Permission and lifecycle

- [ ] After authenticated launch, voice auto-arms once; manual Disable voice
      keeps it off for the rest of that app session.
- [ ] Microphone, screen-recording and accessibility denial is visible and recoverable.
- [ ] Revocation, sleep/lock, sign-out and app exit stop capture and guidance.
- [ ] Stop cancels capture, uploads, planning and presentation with no replay.

## Exact passive chord

- [ ] macOS: left/right Command plus left/right Control activates in either order.
- [ ] Windows: physical Left Control plus Left Alt activates in either order.
- [ ] Shift, extra Option/Alt, Windows key and Windows AltGr never activate.
- [ ] The app does not suppress or rewrite the two modifier events.
- [ ] Pressing the second key shows Listening; releasing either key finalizes once.

## Transcription and dispatch

- [ ] English, Vietnamese and noisy samples use completed WAV chunks.
- [ ] Empty, silent, failed or incomplete transcription produces no plan.
- [ ] Partial text never dispatches; release produces at most one guidance plan.
- [ ] Text fallback uses the same plan, cursor, pacing and cancellation journey.
- [ ] Raw audio and transcripts remain memory-only.

## Observation-only Instructor Cursor

- [ ] Chord-down pins the frontmost eligible non-Tro window read-only.
- [ ] On `tests/fixtures/native/gestures.html`, point to the named heading; the
      page remains unchanged until the tester moves the real pointer.
- [ ] Show a ghost click over the counter; its value and focus remain unchanged
      until the tester clicks.
- [ ] Show the complete ghost-drag trajectory; the block remains at its source
      until the tester drags it.
- [ ] Point to the text field and explain what kind of value to enter without
      displaying or inserting the value; the field remains unchanged until typing.
- [ ] Show each scroll direction; scroll position remains unchanged until the
      tester scrolls.
- [ ] The overlay shows the current caption and `Step N of M` and remains click-through.
- [ ] The real pointer does not move during any automated presentation.
- [ ] The fixture receives no click, drag, typing, keypress, focus or scroll until
      the tester performs it.
- [ ] Repeat, Pause/Resume and Continue preserve one authoritative journey.
- [ ] Target/bounds changes, permission loss, auth loss and worker exit clear old cues.
- [ ] Presentation and acknowledgement never become learner-action or completion evidence.

## Measured evidence

| Metric                                    | Target                       | macOS   | Windows |
| ----------------------------------------- | ---------------------------- | ------- | ------- |
| Chord → visible Listening                 | p95 ≤ 50 ms                  | pending | pending |
| Chord → capture ready                     | p95 ≤ 100 ms                 | pending | pending |
| Release → final transcript                | p50 ≤ 600 ms; p95 ≤ 1,200 ms | pending | pending |
| Final transcript → guidance dispatch      | p95 ≤ 50 ms                  | pending | pending |
| Guidance dispatch → first visible cue     | report                       | pending | pending |
| Billed/spoken audio ratio                 | approximately 1.25×          | pending | pending |
| Requests and estimated transcription cost | separate from guidance cost  | pending | pending |

If clean devices, provider authorization, signing or reference-network evidence is
unavailable, keep P7-A/P8 release acceptance open even when source gates pass.
