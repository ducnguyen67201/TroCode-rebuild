# P0 acceptance evidence — 2026-09-13

Environment: this macOS ARM64 workstation; Node 24.14.1, npm 11.6.2, Rust 1.95.0, Python 3.12.12, uv 0.12.1. No hosted resources or remote CI were used.

## Observed native behavior

- Real Tauri WebView → native command → Rust supervisor → Python diagnostic session: Start, Check connection, Stop succeeded.
- Selected Teacher against the local fixture API, started a session, switched to Student A, and started another session. The previous worker was replaced, not resumed under another account.
- A temporary CSS edit emitted Vite HMR events and preserved the running Python PID; the source edit was restored immediately.
- Closing the native window while the student worker was active exited the host and reaped its child.
- The final supervisor tests additionally confirm that arbitrary parent environment variables do not enter the Python child.

Native launch evidence: `.local/native-launch.log` and `.local/native-evidence.json` (local diagnostics, ignored by Git). Development Vite readiness measured 73 ms in preview and 84 ms in native mode. Native launch required a 3.50-second incremental compilation in the observed run. These are single observations on this machine, not service-level targets.

## Automated evidence

62 test cases: 11 command/routing tests, 17 TypeScript tests, 18 Python tests, 14 Rust unit/contract/process tests, one PostgreSQL/private-S3 integration scenario, one Chromium browser scenario. The browser checks explicit preview labeling, start/stop/health/restart/profile behavior, no page errors, and no horizontal overflow at 390 px. Preview screenshot: `.local/ui-preview.png`.

TypeScript typecheck/lint, Python Ruff/format/mypy, Rust format/clippy, generated contract drift, UI build and macOS native development build pass. npm and Python audits report no known vulnerabilities after dependency corrections. Local `tro-runtime` has no PyPI advisory entry and is covered by source/tests rather than a third-party package audit.

Validation logs: `.local/verification-initial.log`, `verification-contracts.log`, `verification-node-fixes.log`, `verification-python-fixes.log`, `verification-rust-fixes.log`, `verification-final-native.log`. Initial failures are preserved; corrected results are recorded in the implementation report. Applicable checks were rerun only when reported failures or subsequent related source/dependency corrections required them.

## Remaining gates

- [ ] Execute the configured CI workflow after a separately authorized remote/push.
- [ ] Run Windows native build/process tests and interactive acceptance on Windows.
- [ ] Validate signed installers, bundled Python/CUA, permission identity, overlay/scaling and stop on target platforms (P1).
- [ ] Establish actual hosted teacher/two-device acceptance and resolve dependent pilot decisions.
- [ ] Test supported legacy upgrade histories before any replacement release (P8).

P0 source implementation/local acceptance is complete. These results do not establish packaged CUA behavior, learning efficacy, production authentication or migration safety.
