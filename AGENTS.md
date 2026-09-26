# Tro rebuild contributor instructions

Read `docs/rebuild-architecture.md` first. It is the canonical master spec.
Map implementation work to its C/F requirement IDs, P0–P9 phases and acceptance
criteria. Update the spec for changed decisions; do not create competing plans.
Detailed defaults remain proposals where the spec says they are unresolved.

## Ownership

- Python OpenAI Agents SDK owns one teaching session, its planner and tool loop.
- F10 guided teaching uses observation-only CUA: Tro shows where/how to act and
  the learner performs the external-app action. F11 is the sole direct-control
  exception: an explicit voice/text instruction may use allowlisted CUA actions
  in one pinned window, with action limits, fresh validation, confirmation for
  consequential or ambiguous actions, emergency stop and no replay. Never expose
  raw/native input tools to React or a generic dispatcher to the model. Store only
  bounded action outcome metadata, never transcripts, screenshots or typed values.
- Tauri/Rust owns native windows, shortcuts, permissions and worker supervision.
- React owns presentation. Keep raw native tools, arbitrary process spawning
  and provider credentials out of the frontend. Validate cross-process messages.
- The Rust backend owns shared classroom authority, assignments and submissions.
- New Rust backend persistence uses SeaORM entities, ActiveModels, typed queries,
  transactions and the SeaORM migration DSL. Do not add handwritten SQL in new
  handlers, repositories, tests or migrations. Preserve existing published SQL
  migrations; replacing legacy SQLx code is separate, explicitly scoped work.
- Teaching cursor motion does not move/click the real pointer. Presentation
  completion is not evidence of learning or student submission.
- Preserve legacy data and published migrations. A rewrite is not permission to
  delete user state. Audit old/new compatibility before migrating or retiring it.

## Implementation first, verification afterward

Founder requirement: finish a complete agreed milestone, including its test
code and source review, then run the applicable verification batch. Do not run
typecheck, lint, full tests, audits or packaging after each edit or file.
Do not attach the full quality gate to save hooks or ordinary dev restarts.

CI is the default post-implementation gate once configured. Focused local checks
are appropriate to diagnose a concrete failure or when explicitly requested.
Collect failures and fix them together. Final revisions must pass applicable
checks before merge/release; report verification pending when it cannot run.
Do not defer all verification until the entire replacement application is built.

## Workspace state and external actions

This workspace contains the P0 implementation. Consult the master spec and
PR checks for verification status; native P1 acceptance is separate.
Keep `.claude/PRPs/plans/` and `.claude/PRPs/reports/` local and untracked.
Use the `codex/` branch prefix. Preserve other contributors' edits.
Do not push, publish, deploy, create remote resources or send messages without
the user's authorization for that action. Local planning and implementation
within the requested scope can proceed without repeated confirmation.
