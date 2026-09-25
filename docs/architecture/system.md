# System and ownership

## Presentation

`features/auth/AuthGate.tsx` is the product boundary. Until native state reports
an authenticated account with at least one active workspace, React renders only
the sign-in/holding experience. `platform/auth-client.ts` carries a validated,
token-free projection; browser preview scenarios are explicit simulations.

`apps/desktop/src/features/teaching/TeachingPanel.tsx` owns controls and the plan
projection. `TeachingOverlay.tsx` draws highlights and gesture animations.
`OverlayWindow.tsx` receives validated, expiring cue payloads. React has no CUA
imports, provider keys or arbitrary process API.

`platform/teaching-client.ts` defines the feature interface. `tauri-client.ts`
implements native requests and state subscriptions. `preview-teaching.ts` is an
explicit simulation; it never silently replaces a failing native connection.

## Native host

`apps/desktop/src-tauri/src/commands.rs` is the UI command boundary. `manager.rs`
owns runtime availability and account selection. `worker.rs` owns child process,
private pipes, correlated requests and shutdown. `overlay.rs` owns nonactivating,
click-through windows, coordinate mapping, cue expiry and the refresh loop.
`geometry.rs` validates presentation bounds against observed elements.

`account.rs` reads private proof configuration and obtains bounded model grants.
`permissions.rs` handles OS observation consent. Tauri capability files restrict
which WebView can invoke which command; these are separate from OS consent.
The overlay cannot invoke teaching, account or worker control commands.

`auth/` owns system-browser Google authorization with PKCE/state/nonce, the
fixed-origin hosted API client, access-token memory and rotating refresh-token
storage in the OS credential manager. Only the main window can invoke the closed
auth commands. Runtime start, health, restart, proof and teaching commands check
current native workspace access again; React visibility is not an authorization
boundary. Logout or terminal renewal failure clears native identity and stops the
account-scoped worker. Ordinary application exit preserves the remembered-device
refresh credential.

## Local Python

`__main__.py` handles transport and scheduling; `runtime.py` handles lifecycle.
`teaching.py` owns one TeachingSession and coordinates these cohesive modules:

| Module                  | Responsibility                                                                                |
| ----------------------- | --------------------------------------------------------------------------------------------- |
| `planning.py`           | Accessibility/visual plan types, bounded target resolution and deterministic step progression |
| `agent.py`              | Local Agents SDK calls producing structured proposals/plans, with no callable tools           |
| `observations.py`       | Immutable window, element and geometry values                                                 |
| `observation_source.py` | Read-only CUA adapter and bounded selected-window native policy                               |
| `guidance.py`           | Pure cue construction and explicit expected-value checks                                      |
| `session_store.py`      | Account-isolated, locked SQLite evidence journal                                              |
| `model_client.py`       | HTTPS model client using a scoped backend grant                                               |

The controller depends on an observation interface, allowing test observations
without native software. PlanProgress is independent of transport, UI, CUA and
the SDK. Neither a service locator nor a separate planner process is required.

## Backend and data

Hosted mode in `services/api/src/hosted.rs` uses a SeaORM connection and typed
migration crate. Google proves issuer/subject and a verified email; the backend
creates the Tro account/session, issues a 15-minute audience-restricted JWT and
rotates an opaque 30-day refresh credential whose HMAC digest is stored. Reuse of
a rotated refresh credential revokes its device family. Hosted auth code and its
migration contain no handwritten SQL.

Workspace authority remains workspace-owned. Authentication calls the
transaction-scoped `WorkspaceMembershipStore` boundary to claim pre-added email
memberships and load current access. Until the workspace branch supplies that
SeaORM adapter, `PendingWorkspaceMembershipStore` returns no memberships, so a
verified account remains in `membershipRequired` and cannot enter the product.

`services/api/src/model_gateway.rs` provides isolated proof identity, expiring
model grants and a fixed provider route. It enforces a four-request grant budget,
1024 output tokens, request/response size limits and timeouts. Provider keys stay
here. Shared classroom functionality is later work; proof identity is not a
production authentication system.

Selected-window images and bounded AX context may go to model inference through
the gateway. Images do not enter React projections or SQLite. SQLite stores
identifiers and evidence kinds, including separately observed and learner-reported
step advancement. Plans currently live in memory and are cleared on Stop; durable
plan checkpoints and crash resume are not implemented. Published migrations and
legacy user state are preserved.
