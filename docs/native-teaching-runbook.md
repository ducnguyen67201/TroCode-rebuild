# Native teaching foundation

The master specification owns P1 and F10. This implementation provides an observation
adapter, visual cue preview/overlay, separate evidence facts, an optional bounded model
connection, and explicit runtime packaging. P1 acceptance is still open until the
manual Windows/macOS record in `tests/acceptance/native-foundation.md` is complete.

## Development

Use `npm run setup`, then `npm run dev`. Start the runtime and use Observation
permissions. Open the offline gesture fixture yourself in a browser. Find windows,
select the fixture and Observe. The manual guidance controls let you select an observed
control and visualize a gesture. Repeat obtains fresh evidence. It never clicks, types,
scrolls, drags, focuses or launches another application.

The native overlay is nonactivating and click-through. It expires after one second without fresh evidence; a host loop re-observes geometry every 250 ms while guidance remains valid.
Command/Ctrl+Shift+Escape is the host emergency stop. A shortcut conflict prevents launch
rather than silently removing Stop. Secondary macOS monitor overlays remain disabled
pending calibration. Restart Tro after changing the connected display set. Capture protection and permission attribution require actual device
verification. No result from an animation establishes learning or submission.

## Optional proof server

Use a separate PostgreSQL database named `tro_rebuild_proof`, with no legacy `users`
table. Set `TRO_API_MODE=proof`, `DATABASE_URL`, and a loopback `TRO_API_BIND` behind a
trusted HTTPS reverse proxy. Run the API `migrate` command. Provision `proof_accounts`
and hashed, expiring `proof_sessions` explicitly; fixture accounts cannot authorize this
server. For `serve`, supply `OPENAI_API_KEY` and `TRO_MODEL` on the server only.

On the desktop create owner-only `~/.tro-rebuild/proof-account.json` on macOS, or
`%LOCALAPPDATA%/.tro-rebuild/proof-account.json` on Windows:

```json
{"origin":"https://your-proof-server.example","token":"64-hex-character-proof-account-token"}
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
