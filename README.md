# TroCode rebuild

A Tauri/React desktop, private Python runtime and modular Rust API. P0 is the source foundation; teaching, CUA and packaged installers arrive in P1/P2. The [master specification](docs/rebuild-architecture.md) remains canonical.

## Develop

Install Node **24.14.1** / npm **11.6.2**, Rust (rustup reads `rust-toolchain.toml`), uv **0.12.1**, and your platform's [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/). Docker is needed only for the fixture API. Setup uses pinned Python 3.12.12 through uv; students will not need these tools in the future packaged application.

```bash
npm run setup
npm run dev:ui
```

For the complete local fixture stack with Doppler-provided environment values,
configure this repository once and use the Make target:

```bash
doppler setup
make dev
```

`make dev` runs `doppler run -- npm run dev:full`, which starts PostgreSQL and
MinIO, migrates and seeds the fixture backend, then launches the Rust API, Vite,
the Tauri desktop and its supervised Python worker. Press Ctrl-C once to stop
the owned development processes. To select Doppler without saving a scoped CLI
configuration, use
`make dev DOPPLER_PROJECT=<project> DOPPLER_CONFIG=<config>`.

The fixture database and object-storage credentials are still generated locally
under `.local/` and override same-named injected values, preventing this command
from targeting a non-fixture database. Do not put private values in `VITE_*`
variables because Vite exposes that prefix to browser code. `make dev-local`
starts the same fixture stack without Doppler.

| Command                         | Purpose                                                                                       |
| ------------------------------- | --------------------------------------------------------------------------------------------- |
| `make dev`                      | Full local fixture stack under `doppler run`                                                  |
| `make dev-local`                | Full local fixture stack without Doppler                                                      |
| `npm run dev:ui`                | Fast browser preview; no Rust, worker, Docker or credentials                                  |
| `npm run dev`                   | Real Tauri desktop + private Python worker                                                    |
| `npm run dev:api`               | Start isolated DB/storage, migrate/seed and run fixture API                                   |
| `npm run dev:full`              | Desktop and local API together                                                                |
| `npm run test:browser`          | Headless Chromium preview smoke (install browser once with `npx playwright install chromium`) |
| `npm run contracts:generate`    | Regenerate bindings after schema changes                                                      |
| `npm run build`                 | Compile development artifacts; no full verification                                           |
| `npm run verify -- --scope all` | Explicit consolidated post-implementation gate                                                |
| `npm run package`               | Reports the P1 bundled-runtime prerequisite; produces no installer                            |

React edits use HMR. Restart Python using **Restart runtime** inside the desktop; only the host owns its private process. Native Rust edits use Tauri's rebuild. API edits use an explicit API restart. Startup does not install dependencies or run tests/typechecks/audits. Generated contracts are checked only in the explicit quality gate.

The diagnostic UI starts a session, checks health and stops/restarts the worker. It performs no model calls, computer actions or learning assessment. The preview is visibly simulated. Development profiles use the local fixture API and never expose tokens to React.

See [navigation/architecture](docs/CODEX-NAVIGATION-GUIDE.md), [test environment](docs/testing/foundation-environment.md), [verification](docs/testing/ci-workflow.md) and [migration inventory](docs/migration-inventory.md). Local credentials live in ignored `.local/` files; setup preserves existing files. No remote, deployment or production migration is configured.

## P1 visual teaching work

P1 implementation is in progress: selected-window observation, visual gesture guidance,
account evidence, a bounded model gateway, and native runtime packaging. Tro never operates
the learner's external application. Follow [the native runbook](docs/native-teaching-runbook.md).
Windows/macOS manual acceptance and signed distribution remain pending.

See the [code architecture guide](docs/architecture/README.md) for the local planner, observation loop, contracts and module responsibilities.
