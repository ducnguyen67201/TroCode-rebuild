# P0 legacy ownership and migration inventory

Read-only baseline: legacy `/Users/ducng/Desktop/workspace/TroCode` at `2a66b0bf84e1cbfc22cb8aabfba618285a909084`. Planning worktree `/Users/ducng/.codex/worktrees/9ee2/TroCode` has uncommitted AGENTS/README/verification/spec edits; preserved. Legacy has an untracked Agents SDK architecture plan; preserved. New workspace owns this rebuild and has its own `codex/rebuild-foundation` branch. No legacy code, data or published migrations were modified.

Paths below are relative to the legacy checkout. [Checksums](legacy-migration-checksums.json) record all 37 source migration files using SHA-256 for inventory; SQLx maintains separate migration checksums. The legacy `services/api/src/db.rs` also supports a broadcasts-first ordering: versions 34/35/36 map to files 035/036/034. Preserve both exact histories; do not renumber or rewrite files. Actual deployed schema/history is unverified.

| Domain | Legacy source | Disposition/owner | Future proof |
| --- | --- | --- | --- |
| Authentication | `src/main/auth/auth-session-store.ts`, `services/api/src/auth/sessions.rs` | Replace native storage via Rust OS protection; review backend session rules for porting | Old encrypted session cannot be copied as plaintext; reauth/export policy before P8 |
| Classroom roles | `services/api/src/auth/classroom_access.rs`, migrations 018 onward | Port reviewed rules into Rust class feature in P3 | Direct cross-account/role requests rejected |
| Materials | `services/api/src/http/knowledge.rs`, migrations 008/009 | Preserve private objects/ownership; Rust metadata module | Verify content version/reference continuity before migration |
| Assignments | `services/api/src/http/classroom_lessons.rs`, migration 037 | Map to versioned assignments in P3 | Active attempts remain pinned to original versions |
| Submissions | `services/api/src/http/classroom.rs`, `src/shared/contracts.ts` | Inventory actual persisted submission semantics before P6/P8; no inferred schema mapping | Selected artifact ownership and review authorship preserved |
| Task history | `src/main/history/`, runtime encrypted state store | Preserve readable history; Python owns future sessions | Legacy reader remains read-only, cannot dispatch old actions |
| SDK checkpoints | `src/main/agent-runtime/encrypted-agent-state-store.ts` | Replace with Python SDK session store in P1/P2 | No opaque cross-SDK resume promise |
| Unknown outcomes | Legacy local invocation journals and protocol | Preserve unresolved outcomes; Python journal owns new mutations in P1 | Restart/migration never blindly replays |
| Preferences | `src/main/preferences/app-preferences-service.ts` | Explicit Rust/React preference mapping later | Import known values only; preserve original file |
| Permissions | Legacy native permission/onboarding modules | Replace with signed Tauri identity | Packaged macOS/Windows permission proof in P1/P8 |
| Updater/app identity | Legacy packaging/manifests | Separate development ID now; release identity decision in P8 | Supported upgrade, signing and rollback proof |
| Admin/operator | `apps/admin/` | Leave legacy operational; review per domain | No production cutover implied by new fixture API |

New local database is an isolated fixture baseline, not a migrated product database. P0 creates no old/new compatibility reader or migration executor. Existing private data was not opened or copied to fixtures. Before P8, obtain authorized deployment history and installed-state samples, define each supported source/target version and rollback cutoff, and run upgrade-path tests. D01–D07 retain their named gates in the master spec.
