# Tro rebuild contributor instructions

Read `docs/rebuild-architecture.md` first. It is the canonical master spec.
Map implementation work to its C/F requirement IDs, P0–P9 phases and acceptance
criteria. Update the spec for changed decisions; do not create competing plans.
Detailed defaults remain proposals where the spec says they are unresolved.

## Ownership

- Python OpenAI Agents SDK owns one teaching session, its planner and tool loop.
- CUA Driver is observation-only. Tro shows where/how to click, drag, scroll
  or type; the learner performs every external-app action (F10). Never expose
  native input tools, automated demonstrations or a do-it-for-me mode. Store
  session/guidance evidence, not a new mutation journal; never replay legacy actions.
- Tauri/Rust owns native windows, shortcuts, permissions and worker supervision.
- React owns presentation. Keep raw native tools, arbitrary process spawning
  and provider credentials out of the frontend. Validate cross-process messages.
- The Rust backend owns shared classroom authority, assignments and submissions.
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
implementation report for verification status; native P1 acceptance is separate.
Use the `codex/` branch prefix. Preserve other contributors' edits.
Do not push, publish, deploy, create remote resources or send messages without
the user's authorization for that action. Local planning and implementation
within the requested scope can proceed without repeated confirmation.
