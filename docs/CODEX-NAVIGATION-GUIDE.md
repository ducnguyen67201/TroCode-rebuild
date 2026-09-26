# Foundation navigation and clean architecture

The [master specification](rebuild-architecture.md) owns product decisions and phase acceptance. P0 implements F07/F08 and C08 groundwork. P1 adds F10 visual teaching; P7-A adds the distinct F11 bounded voice/text control path. Installed acceptance remains open.

| Surface                    | Owner/entry                                                    | Dependency direction                                                                 |
| -------------------------- | -------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| UI                         | `apps/desktop/src/features/teaching/TeachingPanel.tsx`         | Feature → DesktopClient; no native tool/credential imports                           |
| Workspace UI               | `apps/desktop/src/features/workspace/WorkspaceAccessPanel.tsx` | Owner-only presentation → WorkspaceClient; no bearer credentials                     |
| Native adapter             | `apps/desktop/src/platform/tauri-client.ts`                    | Typed Tauri commands; received payloads validated                                    |
| Host                       | `apps/desktop/src-tauri/src/commands.rs`                       | Commands → manager → worker actor                                                    |
| Worker lifecycle           | `apps/desktop/src-tauri/src/worker.rs`                         | Actor owns child/pipes/pending requests; no UI types                                 |
| Runtime                    | `services/teaching-runtime/src/tro_runtime/teaching.py`        | Session decisions → read-only observation/model adapters; transport in `__main__.py` |
| API                        | `services/api/src/lib.rs`, `model_gateway.rs`                  | Fixture API and isolated proof mode; provider secrets stay on backend                |
| Hosted workspace authority | `services/api/src/workspace.rs`, `workspace/handlers.rs`       | JWT/session authentication → owner recheck → SeaORM transaction/audit                |
| Contracts                  | `packages/contracts/schema/`                                   | Single authority → generated bindings/validators                                     |
| Dev workflow               | `scripts/rebuild/cli.mjs`                                      | Thin composition of existing package tools                                           |

## Philosophy

Prefer cohesive modules and explicit dependencies. Separate transport/serialization from policy and lifecycle decisions. A function is usually enough; introduce an interface when two implementations exist (native/preview desktop client, real/in-memory object storage). No generic service locator, base repository, global event bus or empty future modules.

Use meaningful names, bounded functions, early error returns and public errors that do not disclose private inputs. Keep side effects at boundaries. The actor serializes process ownership; runtime logic remains testable without Tauri, a model or a database. A UI status is a projection, never a second source of truth.

## Add a capability

1. Reference its C/F requirement and P phase in the master spec.
2. Add a closed schema variant and shared positive/negative fixtures.
3. Run `npm run contracts:generate` after schema edits (generation is an explicit source operation).
4. Add the domain transition/handler inside its owning module; add I/O only through that module's boundary.
5. Extend the narrow desktop adapter and consuming UI if needed.
6. Add boundary/lifecycle tests, review the complete milestone, then run the applicable verification batch.

Do not import legacy orchestration or share active state with it. Do not export arbitrary process spawning, raw CUA, account tokens or model keys to React. P1/F10 keeps visual teaching read-only. P7-A/F11 may call only the closed selected-window ComputerTool adapter from Python; it cannot expose a generic native dispatcher, shell, clipboard, app launch or cross-window focus. Consequential actions require an explicit host confirmation and all interrupted work is cancelled without replay.

Use a single npm lockfile, Cargo lockfile and Python project lockfile. Scope changed packages in CI. Generated files are committed and never hand-edited. Native worker tests also run without the desktop feature, avoiding WebView dependencies in Linux source gates.

See the [code architecture guide](architecture/README.md) for the local planner, observation loop, contracts and module responsibilities.
