# Generator iteration 01

## Design rationale

The sign-in boundary is treated as a threshold into a lesson rather than an
OAuth card. A split editorial composition pairs the action with a hand-drawn
topographic route: identify the account, find the shared studio, then return to
learning. The route gives every holding state a location and makes waiting feel
intentional without adding a dashboard or decorative stock imagery.

The visual language extends Tro's cream and forest foundation with one muted
rust annotation color and a warm yellow “pencil highlight.” Local system serif
type carries the human, instructional voice while the compact sans-serif labels
behave like notes in a workbook. Motion is limited to a slowly travelling dashed
route and a breathing current waypoint, both disabled under reduced motion.

At narrow widths the editorial column becomes the primary reading path and the
map compresses beneath it. The Google button remains the only signed-out action.
Membership, offline and boundary-failure states keep the same spatial grammar so
the interface does not jump into an unrelated error design.

## Implementation summary

- Added a token-free `AuthClient` and integrated validated Tauri command/event
  projections for the four closed auth commands.
- Added an `AuthGate` that subscribes before reading status, rejects stale
  revisions, fails closed on boundary errors, prevents duplicate actions and
  renders protected children only with an authenticated user and workspace.
- Added deterministic preview scenarios for all seven auth states. Default
  preview starts signed out; sign-in transitions through browser handoff to an
  authenticated workspace without a reload.
- Added authenticated account/workspace context and sign out to the existing app
  shell while leaving the overlay route outside the gate.
- Added Testing Library coverage for gating, membership retry, double-submit and
  fail-closed behavior, plus Playwright coverage for sign-in, sign-out, membership
  and 390×844 overflow.

## Assumptions

- The native event name is `auth-status`, matching the implementation plan.
- Native auth commands always return the shared `AuthStatus` projection and
  classify expected network failures as `offline` rather than throwing private
  errors across IPC.
- Preview query scenarios (`?auth=offline`, `?auth=error`, and peers) are a
  development-only design review surface because preview mode itself remains
  explicitly enabled by the existing Vite flag.
