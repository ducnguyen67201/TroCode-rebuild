# F11 push-to-talk computer-control acceptance

Status: **source implementation and packaged native evidence are tracked separately**.
Complete this checklist on signed macOS arm64 and Windows x64 builds. Do not
record audio, transcript text, screenshots, accessibility values or typed values.

## Build record

- Date, commit and installer SHA-256:
- OS, version, architecture and keyboard layout:
- Signing/notarization status:
- Runtime and contract digests:
- Provider models and pricing date (names only; never credentials):

## Permission and lifecycle

- [ ] Voice remains disabled until the user explicitly selects Enable voice.
- [ ] Microphone and keyboard-monitoring/accessibility denial is visible and recoverable.
- [ ] Revocation, device removal, sleep/lock, sign-out and app exit stop capture/input.
- [ ] Emergency stop cancels capture, uploads, confirmation and action work with no replay.

## Exact passive chord

- [ ] macOS: left/right Command plus left/right Control activates in either order.
- [ ] Windows: physical Left Control plus Left Alt activates in either order.
- [ ] Shift, extra Option/Alt, Windows key and Windows AltGr never activate.
- [ ] The app does not suppress or rewrite the two modifier events.
- [ ] Pressing the second key shows Listening; releasing either key finalizes once.

## Transcription and dispatch

- [ ] 3–8 second English, Vietnamese and noisy samples use completed WAV chunks.
- [ ] Empty/silent/failed/missing chunks produce no agent dispatch.
- [ ] Partial text never dispatches; release produces at most one final dispatch.
- [ ] Text fallback completes the same journey with microphone disabled.
- [ ] Raw audio is memory-only and no private content appears in logs or persistence.

## Selected-window action policy

- [ ] The frontmost eligible non-Tro window is pinned at chord-down.
- [ ] Harmless click/type/scroll completes without another click after release.
- [ ] Window identity/bounds changes cancel before the next native mutation.
- [ ] App launch/switch, shell, filesystem and clipboard behavior is unavailable.
- [ ] Send/post/upload/purchase/delete, secure input, settings and ambiguous actions
      pause for Approve/Reject in the main window; voice cannot approve itself.
- [ ] Reject, timeout, cancel, auth loss and worker crash never replay an action.

## Measured evidence

Record sample count plus p50/p95 where applicable:

| Metric                                    |                            Target |   macOS | Windows |
| ----------------------------------------- | --------------------------------: | ------: | ------: |
| Chord → visible Listening                 |                       p95 ≤ 50 ms | pending | pending |
| Chord → capture ready                     |                      p95 ≤ 100 ms | pending | pending |
| Release → final transcript                |      p50 ≤ 600 ms; p95 ≤ 1,200 ms | pending | pending |
| Final transcript → agent dispatch         |                       p95 ≤ 50 ms | pending | pending |
| First native action                       |                            report | pending | pending |
| Billed/spoken audio ratio                 |               approximately 1.25× | pending | pending |
| Requests and estimated transcription cost | report separately from agent cost | pending | pending |

If clean devices, provider authorization, signing or reference-network evidence is
unavailable, keep P7-A/P8 release acceptance open even when source gates pass.
