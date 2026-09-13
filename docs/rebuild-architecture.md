# Tro rebuild master specification

Status: master planning specification, 2026-09-13. The founder agreed to the
stack and product direction; detailed behaviors and phases below are the
proposed implementation baseline unless explicitly marked agreed. P0 is merged (PR #1); local verification and final-revision Linux integration/source
plus Windows/macOS native build/worker CI passed. P1 source and local macOS packaging are implemented; native acceptance remains in progress.
Interactive packaged-platform validation belongs to P1 and remains unproven.

This is the canonical rebuild spec, now maintained in
`/Users/ducng/Desktop/workspace/TroCode-rebuild`. Read it before starting any milestone.
Implementation tickets and narrower designs must reference its requirement and
phase IDs. Existing Electron documentation describes the legacy app; it is not
the target architecture. Amend this spec when decisions change rather than
creating a competing rebuild plan. Preserve a record of why a requirement changed.

Start with the [execution checklist](#execution-checklist-start-here) for the next
work to do, the [customer map](#customer-request-to-product-map) for why, then the
[phase plan](#phased-implementation-plan). The [decision register](#decisions-still-required)
identifies remaining uncertainty; [verification](#implementation-and-verification-order)
preserves implementation-first execution.

## How to use this specification

- Sections below connect evidence → requirements → ownership → phases → acceptance.
- `C` requirements derive from the customer interview; `F` requirements derive
  from the founder's original notes or explicit rebuild instructions.
- A requested outcome is not proof that our proposed implementation will solve it.
- `D` decisions remain unresolved. They block only the dependent capability or
  release, not independent implementation work.
- All phase checkboxes start unchecked. Code existing in the old app does not
  establish that the replacement meets a phase's acceptance criteria.

## Agreed interaction boundary — learner performs every external-app action

Founder clarification, 2026-09-13 (F10): Tro shows where and how to click,
drag, scroll or type; the learner performs the action. This supersedes earlier
proposals for automated setup/demonstration and a native mutation journal.
CUA is observation-only. Tro must not move the real pointer, inject clicks,
keystrokes or drags, change focus, or operate another app on the learner's behalf.
Animation and replay are visual guidance only. There is no “do it for me” mode.
This restriction concerns external-app/device control; ordinary explicit Tro
UI operations such as selecting a lesson, asking for help or submitting work
remain possible under their own authority rules.

## Evidence register and limits

| ID | Source | What it establishes |
| --- | --- | --- |
| E1 | NoteSystem: `20_Projects/Tro/Customer Interview - 2026-08-24.md`, “Normalized notes supplied by Đức” and opening pain synthesis | Founder-recorded customer observations and requests; strongest evidence for the walk-around checking problem |
| E2 | Same file, timestamped transcript of `New Recording 47.m4a`, 1:53:24 | Classroom observation and discussion; automatic transcription contains substantial noise and overlapping speech |
| E3 | NoteSystem: `20_Projects/Tro/docs/gtm-cofounder/founder-brief.md` and `gtm-roadmap.md` | Founder vision, screen-aware MVP, teacher-support positioning, buyer and pilot hypotheses |
| E4 | Founder conversation in this task, 2026-09-13 | SDK controlling CUA, planner, teaching cursor, screen awareness, teacher materials, accepted stack, implementation-first verification |
| E6 | Founder clarification in this task, 2026-09-13 | Show where/how to act; learner performs every external-app action; no automated demonstration |
| E5 | Granola: August 19, 16:00 “Short recording”, `9800dff4-f559-484a-bacb-c5d0c7a70887` | Persistent/translucent highlight, tutorial language and readable pacing feedback; speaker is not established as a customer |

E1 and E2 cover one private robotics/programming center. Parent needs are
reported by the center, not by direct parent interviews. Partnership discussion
appears around 00:02:30–00:03:00; purchase interest is recorded by Đức, without
a reliable verbatim transcript quote. No price, purchase authority, named pilot
class, start date, or signed commitment is established by these notes. Do not
describe this as a sale or multi-customer validation.

The historical English-center segment and free 14-day pilot for up to 20
students in E3 are hypotheses. They must not override the actual robotics-center
workflow or silently become confirmed capacity and scheduling requirements.

## Customer request-to-product map

The “recorded need” column is a paraphrase unless quotation marks are present.
The behavior and acceptance columns are our proposed implementation of that need.

| ID | Recorded need and evidence | Proposed product behavior | Observable acceptance | Phase |
| --- | --- | --- | --- | --- |
| C01 | Teacher manually walks over to inspect/check work. E1; E2 00:45–00:46, 00:50–00:51, 00:58:30–00:59, 01:11:30–01:15 | Live class overview and inspectable student context, with separate progress, help, connectivity and submission facts | Teacher identifies the assigned task, latest reliable state and evidence for each of two students without visiting the device; stale/unknown state is visible | P3, P5 |
| C02 | Teacher sometimes does not know where a student is stuck; recommendations would help. E1; E2 00:41–00:42, 00:58:30–00:59, 01:45–01:47 | Contextual student help, bounded recovery, and a teacher attention queue; opt-in in-session assistance candidates under D03 | A planted mistake produces a useful next step; unresolved help reaches the teacher with task, step, evidence and attempted guidance; quiet unresolved students are not silently labeled healthy | P2, P5 |
| C03 | Changing a URL requires walking around to enter it; wants one button for the action. E1; E2 01:25–01:26, 01:30–01:31 | Teacher distributes a URL/resource and guidance; students open it themselves | Delivery and student-reported/observed opening remain distinct; automatic opening is outside F10, so the original one-button outcome is only partially addressed | P4 |
| C04 | Students enter the wrong thing and cannot understand the result; help takes a long time. E1 and classroom correction passages in E2 | Observe the actual error, explain why it happened, guide recovery while retaining the lesson objective | Student recovers from an agreed real-app error without losing assignment context or receiving an impermissible completed answer | P2 |
| C05 | “Nếu tự xong, tự submit thì sẽ hay.” Completion/submission concern is explicit in E1; transcript wording is uncertain | Ready/check/submission/review are separate; proposed default is one explicit student submission after preview | Teacher distinguishes finished-but-unsubmitted from submitted; retry does not duplicate a submission; no narration or agent completion submits work | P3, P6; D04 |
| C06 | Insight into slow/fast students, common difficult steps, and who needs attention. E1; E2 00:04–00:04:30, 00:10:30–00:11:30, 01:14–01:15 | Session and across-session summaries of steps, help, unresolved issues, submissions and interventions | Teacher can locate a repeated difficult step and supporting evidence; disconnected time and model latency are not called poor learning pace | P5, P7 |
| C07 | Teacher triggers a session from start into working on a task. E1 | Teacher-controlled start, pause/resume and end, with assigned task and visible participation | Joined students enter the intended assignment; ending or pausing the session prevents new class actions; reconnect does not revive obsolete directives | P3, P4 |
| C08 | Teachers need broader controls and students limited permissions. E1; E2 00:07:30–00:10:30 | Server-enforced class roles, scoped membership, teacher-owned materials and help policy | Student cannot publish class material, broadcast, inspect another student's private work, change the teacher policy or approve their own review through UI or direct API | P0, P3 onward |
| C09 | Quiet voice commands may fail and need repetition. E1; E2 supports the noisy environment, not an exact quote | Push-to-talk, transcript preview where appropriate, visible listening state, and complete text/button paths | The full teaching/classroom journey works with microphone disabled; noisy or failed transcription does not silently dispatch the wrong class action | P2 silent path, P7 voice |
| C10 | Parent-facing value includes visible progress and controlled AI/computer use. E2 01:19–01:22:30, center's account of parents | Teacher-reviewed progress summary from class evidence; enforce assignment help boundaries | Teacher can explain what was attempted, submitted and supported; report does not invent mastery, diagnoses or unrelated device activity | P3 policies, P7 reporting; D03/D05 |
| C11 | Custom cursor and visual personalization. E1, weak exact transcript support | Optional cursor appearance choices independent of core pointing behavior | Customization preserves target accuracy, readability, click-through behavior and reduced motion | P9 |
| C12 | Leaderboard. E1, weak exact transcript support | Conditional teacher-configured engagement feature after purpose and scoring are agreed | Scoring is explainable and does not present inferred speed as ability; teacher can disable it | P9; D08 |
| C13 | Scratch-like customization. E1, meaning not specified | Discovery item: determine whether this means appearance, block programming, or authoring before implementation | A concrete example and scope are recorded; no speculative visual-programming subsystem is built from this phrase alone | P9; D08 |
| C14 | Small games for early finishers or students unsure what to do next. E1 | Optional teacher-assigned follow-on activity after completion; unresolved confusion still routes to help | Teacher controls availability; a game cannot replace help, bypass required work, or falsely complete an assignment | P9; D08 |

The Vietnamese completion phrase does not settle whether the customer meant
students submitting independently or automatic software submission. D04 preserves
that ambiguity. Explicit submission is the current product baseline, not a claim
that the customer rejected automation.

## Founder requirements and supporting features

| ID | Requirement and source | Implementation boundary | Phase |
| --- | --- | --- | --- |
| F01 | OpenAI Agents SDK controls CUA Driver. E4 | Python runtime owns orchestration; CUA supplies read-only native observation under F10 | P1, P2 |
| F02 | Planner. E4 | Structured, revisable teaching plan in the same session; no extra planning microservice | P2 |
| F03 | Cursor explains “what we are doing, then this, then that.” E4 | Step overview, grounded pointing, explanation, student action, fresh check; presentation does not grade | P1, P2 |
| F04 | Know what is on screen. E3/E4 | Fresh native/semantic/visual context with target identity, coordinate mapping and stale-target handling | P1, P2 |
| F05 | Teacher uploads material and uses it to make class work easier. E4 | Upload/ingestion state, review, immutable material versions, assignments and scoped retrieval | P3 |
| F06 | Help students become independent with teacher guidance. E3/E4 | Visual guidance with learner-performed actions; teacher controls explanation/answer reveal, never external-app automation | P2, P3 |
| F07 | Cross-platform rebuild using agreed stack. E4 | Tauri + React/TypeScript + bundled Python SDK/CUA + Rust backend | P0, P1, P8 |
| F08 | Implement first, typecheck and verify afterward. E4 | Complete milestone → review → applicable verification batch → grouped fixes | Every phase |
| F09 | Readable, persistent, correctly localized teaching cues. E5, unattributed feedback | Highlight persists while relevant; local pacing, repeat/next controls, correct lesson language | P1, P2 |
| F10 | Show where/how to click, drag, scroll or type; never perform it for the learner. E6 | Read-only CUA, visual gesture cues, learner action then fresh check; no native mutation tools or journal | P1 onward |

Uploads, SDK choice, and the exact cursor teaching loop are founder requirements.
The interview supports their purpose; do not attribute their detailed design to
the customer. General consumer agents, broad connector catalogs, a complete LMS,
video-to-course generation, and autonomous grading are outside the first release.

## Roles and primary surfaces

| Role/surface | Minimum experience | Boundary |
| --- | --- | --- |
| Teacher: preparation | Class, roster, upload status, material preview, objective, help policy, assignment version | Only authorized class staff publish; incomplete ingestion cannot masquerade as usable material |
| Teacher: live class | Start/pause/end, participants, current assignment, attention queue, resource distribution receipts | No assumption that sent means delivered or that online means making progress |
| Teacher: review | Submitted artifacts, checks/evidence, return with feedback, complete/review decision, class summary | AI feedback is labeled; teacher decision and student submission have separate authors |
| Student: assignment | Join class, see task/material, open resource, text help, optional voice, teaching cursor | Never needs a development environment; no broad teacher controls |
| Student: learning | What/why/next, repeat, I tried it/check, ask teacher, pause/stop | Explanation and action authority stay distinct; silent interaction remains complete |
| Student: submission | Ready indicator, work preview, submit receipt, returned feedback, revision | Only selected work is submitted; account/class scope remains explicit |
| Operator | Account access, deployment health and bounded runtime diagnostics | Operational privileges do not imply classroom content access |

For P3, propose PDF, UTF-8 text/Markdown and HTTPS links as the initial material
set; D02 confirms customer formats before that scope is locked. Each format
has explicit unsupported/encrypted/scanned/too-large handling. Add Office or
video support only when chosen, rather than claiming every upload is readable.
Teacher UI may reuse the web frontend, but a separate web deployment and mobile
teacher experience are not prerequisites unless the pilot needs them.

## Product to preserve

Tro is a screen-aware teaching assistant. A student asks for help inside the
software they are already using. Tro plans how to teach the task, points with
a teaching cursor, explains the next step, lets the student act, observes the
result, and adapts. Teacher materials and classroom controls supply the learning
context and make running a class easier.

The August 24 customer interview identifies repeated teacher walk-arounds as
the strongest recorded pain: discovering who is stuck, inspecting mistakes,
repeating URL/setup actions, checking finished work, and confirming submissions.
Teacher material upload, assignments, contextual help, progress, and explicit
submission belong in the product. Pet customization and gamification are
secondary to a working teaching and intervention journey.

The founder intends to rebuild the application. This agreement records the
target; it does not authorize deleting existing user data, rewriting published
migrations, changing production services, or publishing a replacement build.

## Stack

| Component | Agreed direction | Responsibility |
| --- | --- | --- |
| Desktop shell | Tauri 2 / Rust | Native windows, overlay placement, shortcuts, permissions, worker lifecycle |
| Interface | React + TypeScript + Vite | Student experience, teacher controls, materials, progress, captions |
| Teaching runtime | Bundled Python + OpenAI Agents SDK | Planner, teaching session, model/tool loop, evaluation of results |
| Computer interaction | CUA Driver Python SDK | Read-only window/semantic/screen observation; native input capabilities are disabled |
| Teaching cursor | Transparent overlay with SVG/CSS presentation | Highlighting, pointing, explanations, local animation |
| Shared backend | Rust API | Accounts, classroom authority, assignments, submissions, model access/accounting |
| Shared persistence | PostgreSQL + private object storage | Classroom records and uploaded material bytes |

Use Rust where native and backend responsibilities benefit from it; do not
rewrite the Python Agents SDK orchestration in Rust merely to have one language.
Review existing Rust backend behavior for reuse or replacement rather than
assuming its current architecture must survive unchanged.

The Python interpreter, dependencies, and native CUA assets must ship inside
the application package. Students must not install Python, Node, or packages.
Use a supervised private worker with a versioned, schema-validated message
protocol. Keep arbitrary process spawning and raw CUA access out of the UI.
Provider keys remain backend-only; the local runtime uses authenticated access.
The OpenAI Agents SDK and teaching session run on the user's machine inside
that bundled Python worker. Model inference runs remotely through the backend
gateway; the backend does not host the teaching agent loop.

## Ownership

The Python teaching runtime owns one authoritative teaching session. Planning
is a capability within that runtime, initially producing structured plans;
it is not a separate service or competing execution loop. The SDK executes and
revises the plan using observations and tool results.

Each teaching step identifies its objective, explanation, observed target,
actor (always the learner for external-app actions), and evidence required to continue.
Preserve assignment version, teacher guidance constraints, learner question,
current step, and progress while explaining, waiting for the learner and checking fresh evidence.

CUA supplies read-only observation. Its mutation APIs are neither exposed as
agent tools nor implemented in the application adapter. Enforce the same
read-only boundary in native policy, so an unexpected model tool request cannot
turn into a click, drag, keystroke, app launch or focus change. No native action
dispatcher or mutation journal is needed in the new implementation.

Tauri owns native presentation and process supervision. The interface animates
locally from structured presentation requests. The model must not control each
animation frame. A teaching pointer can move independently of the real mouse;
highlighting and simulated click effects must not dispatch actual clicks.

The backend owns shared classroom facts. Student reports, model hypotheses,
observations, and teacher confirmations must remain distinguishable. A model
turn finishing, narration finishing, or a tool succeeding does not establish
student understanding, assignment completion, or submission.

## Teaching flow

1. Teacher uploads material and defines the assignment and permitted help.
2. Student enters the assignment or asks a question in the current application.
3. The planner combines material, objective, question, and fresh screen context.
4. Tro explains and points at the next relevant target.
5. The student clicks, drags, scrolls or types; Tro only shows the gesture and explanation.
6. Tro obtains fresh evidence when needed and continues, revises, or escalates.
7. The student explicitly submits work; the teacher reviews it.

Do not play a whole multi-screen lesson from one stale screenshot. Use semantic
context where available and visual observation where necessary. Presentation
completion and student progress are separate events.

The customer wants earlier visibility into stuck students. Decide the exact
context-sharing behavior with the pilot: explicit help snapshots, requested
inspection, or another agreed signal. Do not silently interpret the existing
event-based dashboard as permission for continuous screen monitoring.

## Data, contracts and lifecycle rules

These are logical records and ownership rules, not a mandate to create one
service, table, class or independent state machine per row. Reuse existing
concepts where their meaning matches; do not keep duplicate writers for aliases.

| Record | Authority | Minimum meaning |
| --- | --- | --- |
| Class / Membership | Backend | Teacher/student membership and effective class permissions |
| MaterialVersion | Backend metadata + private object store | Immutable bytes/reference, content digest, ingestion status and readable source locations |
| AssignmentVersion | Backend | Objective, pinned materials, permitted explanation/visual guidance, evidence criteria and submission expectations |
| ClassSession | Backend | Teacher-controlled lifecycle and currently assigned work |
| StudentAttempt | Backend | One student's participation/work, explicit ready/submission/review facts |
| TeachingSession / PlanRevision | Python runtime, durable local store owned by that runtime | Attempt/version context, question, steps, current position, stop/resume and accumulated evidence |
| Observation | Local CUA adapter | Observation ID, target identity, freshness, coordinate space and permitted screen/semantic data |
| GuidanceEvidence | Python session store | Cue identity, rendering acknowledgement, learner report and fresh observation remain distinct; no native dispatch state |
| HelpCase / ProgressEvidence | Backend, submitted through authenticated scoped requests | Reported issue, origin, timestamp, step, evidence, proposed guidance and teacher resolution |
| Submission / Review | Backend | Selected artifact versions, receipt and separately authored teacher decision |

Use one versioned wire-schema source for the Rust/Python/TypeScript boundary;
choose the generation tooling in P0. Generate language types where useful and
validate at runtime. Do not maintain three hand-written interpretations of the
same payload. The old Zod contract file is migration input, not the sole future
cross-language schema authority.

Local commands include start/resume/pause/stop, student reply, present step and
presentation acknowledgement. Events include plan revision, grounded cue,
waiting for learner, observation/result, help escalation, failure and terminal
state. Include protocol version, request/session IDs and correlation fields.
UI acknowledgements describe rendering/narration only. Reject incompatible
worker protocols before admitting work; do not restart and silently resume a stale cue or infer completion from
a rendered animation. Use bounded queues and omit screen content from logs.

For visual guidance, validate the observed target and display geometry before
presenting a cue. A drag cue includes a grounded start, path and destination;
a typing cue identifies the field and describes what the learner should enter.
After “I tried it,” obtain fresh evidence and report observed/reported/unknown
progress separately. A completed animation proves only that guidance was shown.

Teaching session states express preparing, explaining, waiting for student,
checking, recovering, waiting for teacher, paused and terminal outcomes. A plan
revision preserves the session and references the new evidence. Student action
checking is triggered by an explicit “I tried it” or an agreed bounded observation
policy; it must not become unlimited continuous model polling.

Keep class connectivity, learning progress, help and submission as separate
dimensions in the dashboard. “Offline,” “unknown,” “possibly needs help,”
“student says ready,” “submitted” and “teacher reviewed” are not interchangeable.
A short-lived event stream may update projections, but durable backend state
and event sequence/cursors let reconnect reconcile missed events. Late joiners
receive current assignment state, not a backlog of obsolete device commands.

Persist local session/checkpoint data under the signed-in account with OS-backed
protection. Share only the class-scoped progress/context allowed by its policy.
Budget exhaustion or a provider error preserves progress and exposes a useful
paused/error state. Logs use correlated IDs, timings and outcome codes; no raw
screens, transcripts, material text or provider secrets by default.

## Phased implementation plan

| Phase | Deliverable | Dependency | Status |
| --- | --- | --- | --- |
| P0 | Scope, ownership/migration inventory and shared test environment | None | Source complete; merged PR #1, local and final-revision CI passed |
| P1 | Packaged native SDK/CUA + teaching cursor proof | P0 technical decisions | In progress: source and local macOS packaging implemented; native acceptance open. See [architecture and acceptance](architecture/development.md) |
| P2 | Adaptive student teaching journey | P1 | In progress: bounded planned guidance/local progression slice; full journey acceptance open |
| P3 | Materials, assignments, roles and teacher class overview | P2 integration; P0 backend | Planned |
| P4 | Resource distribution and learner follow-through receipts | P3 | Planned |
| P5 | Contextual teacher intervention and stuck assistance | P2/P3; observation policy | Planned |
| P6 | Explicit submissions and teacher review | P2/P3 | Planned |
| P7 | Classroom voice and evidence-based summaries | P5/P6 | Planned |
| P8 | Upgrade, real pilot and replacement release | Applicable P1–P7 gates | Planned |
| P9 | Selected engagement features and additional platforms | P8 evidence and scope decisions | Conditional |

Phases are dependency and acceptance boundaries, not time estimates. After each
complete milestone, execute its verification batch. Implementation can contain
smaller reviewable changes without triggering the full suite between every edit.
Build teacher-visible status in P3 so the core customer workflow is not left
until after all agent infrastructure. P5 adds contextual intervention and bounded
detection; explicit Help alone does not satisfy the quiet-student problem.

## Execution checklist: start here

This section turns P0–P9 into implementation batches. It is a plan, not a claim
that source setup, new CI, account provisioning or deployment has happened.
Complete each batch's implementation and test code before its verification pass.
Use the detailed phase acceptance criteria below to decide whether it is done.

### Source repository decision

Founder update, 2026-09-13: create a separate local rebuild workspace at
`/Users/ducng/Desktop/workspace/TroCode-rebuild`. This replaces the earlier
proposal to put replacement packages directly in the legacy checkout.

At planning handoff the new folder contained only this master spec, README and
contributor/verification instructions. P0 has since initialized local Git on
`codex/rebuild-foundation`, scaffolded the application and pinned dependencies.
No remote repository, push or deployment has been created. Preserve the legacy TroCode repository and its Git history,
backend data and migrations as the migration/reference source.

The canonical legacy checkout is `/Users/ducng/Desktop/workspace/TroCode`;
the planning worktree was `/Users/ducng/.codex/worktrees/9ee2/TroCode` at
`2a66b0bf84e1cbfc22cb8aabfba618285a909084`. Planning edits in that worktree remain
uncommitted and must not be discarded. Future rebuild-spec edits belong here;
the planning worktree's document is a superseded handoff snapshot.

Source ownership layout (implemented in P0):

```text
TroCode-rebuild/
  apps/
    desktop/
      src/                    React student/teacher UI and overlay rendering
      src-tauri/              Rust native host, commands and worker supervision
  services/
    teaching-runtime/         Python SDK, planner, teaching sessions, CUA adapter
    api/                      Rust shared backend; review and change by domain
  packages/
    contracts/                Wire schemas and generated language bindings
  tests/
    acceptance/               Cross-process and teacher/student scenarios
    fixtures/                 Non-private sample materials and model traces
  scripts/
    rebuild/                  Development, packaging and verification commands
  docs/
    rebuild-architecture.md   This master spec
```

In the legacy TroCode repository, `src/` and `services/agent-runtime/` contain
the Electron/TypeScript runtime. Do not import their orchestration into the new
Python runtime or connect both runtimes to the same active task. The legacy
`apps/admin/` remains operational until its scope is reviewed. Reuse individual tested domain
rules through explicit ports; copying the entire old service graph is not setup.

Use one Python project, one desktop frontend project and one explicit Rust
workspace dependency strategy. Choose and pin tools/versions in the foundation
batch; preserve existing lockfiles until intentionally migrating them. Package
the Python runtime per target OS/architecture. Do not choose a Python bundler
without checking that it includes CUA's native assets in a signed installer.

### Ordered work batches

| Batch | Work to do, in order | Deliverable / next gate |
| --- | --- | --- |
| P0-A: preserve and inventory | Record current revision and worktree changes; set implementation branch; inventory old UI/runtime/backend contracts and state; decide what is reused, replaced, migrated or retired | Source baseline and migration checklist with no lost edits/data |
| P0-B: scaffold source | Create desktop Tauri/React package, Python teaching-runtime package, shared contract source, non-private fixtures and rebuild scripts; add scoped contributor instructions defining owners | Replacement packages have clear responsibilities; no old orchestration imports |
| P0-C: developer workflow | Pin dependencies; configure formatting/type checks/test commands; define independent dev/build/verify commands; add an example environment file without credentials | One documented development entry point; full verification runs only when requested after implementation |
| P0-D: establish boundaries | Implement version/health handshake over private worker transport; define start/stop/events and correlation IDs; establish Rust API model-proxy contract and account isolation | UI → Rust → Python round trip with clean shutdown; no model/CUA behavior needed yet |
| P0-E: test environment | Prepare reproducible database/object-store configuration and teacher/two-student fixtures; identify shared test API, migration version, native test devices and credential provisioning method | Environment can be used for later acceptance; external provisioning remains pending if unavailable/unauthorized |
| P0-F: foundation verification | After P0 implementation and test code, run applicable CI checks and inspect failures together | Foundation reviewed; unresolved pilot decisions do not block independent P1 work |
| P1-A: first real observation | Bundle/load CUA in Python; discover and observe an external window; map coordinates to Tauri; show real permission/error states | Real screen context reaches a bounded target presentation |
| P1-B: visual gestures + learner check | Add click/drag/type/scroll cues, bounded agent explanations, session evidence and I tried it/check | Learner performs every action; native mutation calls remain zero |
| P1-C: packaged proof | Build Windows/macOS installers after implementation; verify clean machine startup, native assets, permissions, scaling/multi-monitor and worker cleanup | Accept or revise the native/overlay packaging choice before broader features |
| P2-A: teaching plan | Add SDK model integration, structured plan/step contracts, sample objective/material and session persistence | Request + screen + material produces an inspectable teaching plan |
| P2-B: learner loop | Add explanation/pointing, repeat, student reply/check, fresh observation, recovery, pause/resume and budget/error states | One complete real-app exercise including a wrong step works in the same session |
| P3-A: teacher preparation | Implement membership/roles, supported uploads, ingestion status, material preview/versioning and assignment objective/help policy | Teacher can prepare an actual lesson without editing files or config |
| P3-B: live class | Implement joining, start/pause/end, assigned context and teacher overview of explicit student facts | Teacher and two students work against the same pinned assignment and backend |
| P4: resource guidance | Add selected-recipient resources/instructions and separate delivery/learner follow-through receipts | Resource delivery never opens or controls an external app automatically |
| P5: intervention | Add contextual help cases, teacher responses and bounded assistance candidates under D03 | Teacher can understand and resolve a student's difficulty without first walking over |
| P6: work hand-in | Add selected-work preview, submission receipt, return/review and revision history | Finished, submitted and reviewed work are unambiguously different |
| P7-A: voice | Add optional push-to-talk/narration, visible input and silent fallback; evaluate actual noisy classroom examples | Voice helps without becoming a prerequisite or accidentally dispatching actions |
| P7-B: insights | Add evidence-backed class summaries and teacher-reviewed exports; measure latency, event freshness, load and cost | Teacher can identify recurring difficult steps from traceable evidence |
| P8-A: migration + hardening | Implement supported local/database upgrades and legacy compatibility; complete regression, native, packaging and update verification | Replacement release candidate with documented recovery/rollback boundaries |
| P8-B: pilot + release | Run named class pilot, compare baseline workload/help/submissions, fix observed failures in batches, obtain release authorization | Measured customer results and authorized replacement rollout |
| P9: optional expansion | Reconfirm engagement ideas; qualify extra platforms and formats individually | Only evidence-supported additions enter the roadmap |

P0-A through P0-E form the first agreed foundation milestone: prepare their
implementation together, then run P0-F. Source inspection and design review are
allowed throughout. Do not run the full suite separately after every scaffold
file. Later P1/P2/etc. batches follow the same completed-milestone rule.

Command names to establish in `scripts/rebuild/` (interfaces, not working commands
yet): `setup` installs pinned development dependencies; `dev` starts the desktop
and worker; `dev-api` starts the shared backend against an explicit test database;
`build` produces development/runtime artifacts; `verify` runs the applicable
post-implementation quality batch; `package` bundles a chosen platform target.
No `dev` restart should call `verify`. Necessary compilation may still fail on
invalid code; implementation-first verification does not mean hiding compiler
errors or claiming an unbuilt app runs.

### Immediate next milestone

P0 planning update, 2026-09-13: the founder requested fast development and room
for growth through appropriate abstractions. P0 implementation and verification
are complete; see the P0 implementation notes below. Native packaged acceptance
remains a separate gate. Planning packets and working reports stay local and are
not committed; durable architecture and acceptance decisions belong in docs.

P0 implementation defaults: npm workspaces, one Cargo workspace and one uv Python
project; a draft-07 JSON Schema source with generated bindings and runtime
validation; a private inherited-pipe Rust/Python transport; and a modular Rust
API. Provide independent UI-preview, native-worker and API development commands.
Normal startup must not reinstall dependencies or run the full verification
gate. Introduce interfaces at actual I/O boundaries and extract shared modules
when real consumers require them; avoid speculative framework layers. Exact
tool/dependency versions are pinned during P0-C rather than claimed validated
by this planning update.

P0 uses isolated local account/database/object-store fixtures and an explicit
disabled-provider model-access contract. Hosted shared-device acceptance and
real SDK model integration remain separate evidence gates. The legacy migration
inventory must preserve the alternate history selected by migration 34's
checksum; the replacement fixture baseline must never run on a legacy database.

The completed source milestone is **P0: repository/source foundation**. It ends
with the master spec preserved, replacement package skeletons, pinned toolchains,
a private Rust/Python health round trip, test-environment configuration and
documented separate development/verification commands. It should not yet add
the full planner, classroom dashboard, voice stack or material ingestion.

The next implementation milestone is **P1: packaged native teaching foundation**.
P1 proves the native experience in a packaged app. If the teaching overlay
or Python/CUA package fails on a target platform, fix that boundary before
spending effort on the rest of the classroom product.

## Detailed phase acceptance

### P0 — Lock pilot assumptions and replacement boundaries

- [ ] Record D01–D07, marking unresolved answers and dependent gates explicitly.
- [ ] Inventory current contracts, deployed migration history, account/session
  state, classroom records, material objects and native adapters. For each major
  component record reuse, replace, migrate or retire and its owner.
- [ ] Choose target package layout, shared protocol/schema tooling, local
  persistence owner, worker supervision, model-proxy contract, environment and
  release channels. Avoid adding framework layers without a concrete consumer.
- [ ] Establish one shared test backend with separate teacher/student profiles,
  migration baseline and usable sample materials. Define check commands/routing
  for the proposed TypeScript, Python and Rust projects.

Dependencies: none. Covers F07/F08, C08 groundwork.
Exit: a reviewable ownership/migration inventory and environment contract exist;
unanswered customer questions have named effects on scope. Do not invent their
answers. Independent P1 work may proceed while pilot details are unresolved.

### P1 — Prove the packaged native teaching foundation

Implementation detail: [code architecture](architecture/README.md).
Planning is in progress; the acceptance boxes below remain unproven. Implement
P1-A observation/presentation, P1-B agent/visual gestures/learner checks and P1-C packaging as
complete milestones, each followed by its applicable verification batch.

- [ ] Tauri/React shell, Python SDK worker and in-process CUA integration ship in
  platform-specific installers with no development-tool prerequisites.
- [ ] Observe an external application, map targets across displays/scales and
  show a persistent, readable, transparent click-through teaching cursor.
- [ ] Implement typed worker communication, basic authenticated model access,
  account-scoped session/evidence storage, crash detection and enforced read-only CUA.
- [ ] Implement stop/cancellation, worker cleanup and actionable native permission
  onboarding. Resolve the overlay/distribution issue in D06.

Dependencies: P0 technical decisions. Covers F01/F03/F04/F07/F09.
Exit: packaged Windows and macOS evidence demonstrates observation → click/drag
cue → learner-performed action → fresh check → stop; native mutation calls stay
at zero, visual replay leaves the real pointer/input unchanged, and no worker is orphaned. This is the go/no-go for
Tauri's overlay path, before extensive UI reconstruction.

### P2 — Complete one adaptive student teaching journey

- [ ] Plan from objective, sample material, student request and fresh observation;
  produce steps with actor, explanation, target and verification criteria.
- [ ] Implement overview, point/explain, repeat, next/check, learner question,
  recovery, pause/stop/resume, and local persistence in one teaching session.
- [ ] Teach click/drag/type/scroll gestures with grounded visual cues and clear
  explanation; the student performs them. “Show me how” means visual guidance,
  never permission to control the student's app.
- [ ] Implement text/buttons fully, localized readable cues, and clear behavior
  for moved windows, wrong apps, missing permissions, unavailable tools and budgets.

Dependencies: P1. Covers C02/C04, silent C09; F01–F04/F06/F09.
Exit: a student completes one multi-screen exercise, encounters a planted error,
gets a grounded correction and resumes; stale targets are re-observed. A
teaching cue or narrated answer never counts as demonstrated understanding.
Unresolved help is represented explicitly for P5 to deliver to a teacher.

### P3 — Teacher materials, assignments and live class basics

- [ ] Teacher creates class membership, uploads supported materials, sees
  processing/error status, reviews extracted content and publishes an assignment.
- [ ] Pin assignment/material versions; retrieve bounded relevant passages with
  source references. A student question cannot change teacher-authored policy.
- [ ] Join/start/pause/end class, attach each student to the correct assignment,
  and show the teacher participant/connectivity, current task, explicit Help,
  Ready and submission status with freshness/source labels.
- [ ] Enforce class/role boundaries on the backend and retain task context across
  individual help requests. Handle late join, sign-out and assignment updates.

Dependencies: P2 for agent integration; P0 backend baseline and D02 formats.
Covers C01/C05/C07/C08/C10 foundations, F05/F06.
Exit: one teacher and two student accounts use the same backend and pinned
material. Teacher sees different student states; unauthorized direct requests
fail. Editing material creates a new version without changing an active attempt.

### P4 — Distribute resources and guide learner setup

- [ ] Teacher selects recipients and sends a resource/URL with instructions.
- [ ] Show resources inside Tro; students open/navigate external apps themselves.
- [ ] Separate queued/received/declined/expired delivery from learner-reported
  or freshly observed follow-through. Delivery is not proof of opening.
- [ ] Pause/end invalidates pending guidance; reconnect deduplicates delivery
  and does not revive stale cues or automatically open resources.

Dependencies: P3 and P1 guidance/session evidence; D03 sharing policy.
Covers C03 partially, C07/C08. F10 rules out automated URL opening; preserve
that unmet portion of the original customer request instead of calling it solved.
Exit: one teacher distributes a resource to two students, sees delivery state
and separately labeled learner follow-through, with zero external-app automation.

### P5 — Contextual teacher intervention and stuck assistance

- [ ] Implement HelpCase with objective, current step, latest permitted evidence,
  what the student asked, guidance already attempted and unresolved issue.
- [ ] Add teacher triage, request-for-context, guidance response and resolution
  that returns to the same student session.
- [ ] Under resolved D03, implement bounded assistance candidates during active
  lessons (such as repeated failed checks or unresolved recovery). Label them as
  hypotheses with reason/freshness; provide student/teacher correction.
- [ ] Make unobserved/quiet state explicit; measure misses as well as false
  alerts. Do not infer confusion from idle time or broad background activity.

Dependencies: P3, P2; D03 is required for new automatic observation/sharing.
Covers C01/C02/C06 and C04 escalation.
Exit: teacher receives a useful case for one planted failure and notices an
agreed quiet-student failure scenario under the chosen observation policy.
Resolve the case without a walk-around in the acceptance exercise. If the
policy cannot support quiet-student detection, record C02 as partially met;
do not call the full customer visibility requirement complete.

### P6 — Submission, checking and teacher review

- [ ] Student previews selected work, submits with a durable receipt and sees
  returned feedback; retries are idempotent at the application boundary.
- [ ] Teacher checks evidence, returns work or records review completion. Model
  suggestions remain distinguishable from a teacher's decision.
- [ ] Keep Ready, assistant completion, submission and teacher review separate.
  Version revisions and preserve earlier submitted work for the chosen policy.

Dependencies: P3; P2 for assistance; D04 semantics and D02 artifact formats.
Covers C05/C01/C08.
Exit: a ready-but-unsubmitted student and a submitted student appear differently;
teacher returns one submission and the student resubmits a new version. No
agent action silently uploads arbitrary files or changes a final grade.

### P7 — Classroom voice and evidence-based summaries

- [ ] Add optional push-to-talk and narration with complete silent fallback,
  visible input state, interruption and teacher-action transcript confirmation.
- [ ] Evaluate quiet/noisy Vietnamese and English input on agreed classroom
  examples; select voice providers from evidence rather than the vendor demo.
- [ ] Report help patterns, repeated difficult steps, interventions and actual
  submissions; teacher reviews any exported parent-facing summary.
- [ ] Establish practical latency, cost and class-size budgets from measurements
  and D07, including ingestion/model failure and backpressure behavior.

Dependencies: P5/P6; D03 report scope and D07 pilot metrics.
Covers C06/C09/C10.
Exit: core journey works with mic off and with agreed noisy voice samples;
voice failure does not dispatch unintended actions. Every progress claim can
be traced to allowed evidence; no inferred ability ranking or unproven mastery.

### P8 — Upgrade, pilot and replacement release

- [ ] Finish the P0 migration inventory: test upgrades from supported installed
  state and existing database versions, preserve material/submission ownership,
  and define old/new client compatibility and rollback cutoffs.
- [ ] Complete packaged native acceptance, release signing/update behavior,
  protocol compatibility and account/session recovery. Delete obsolete executable
  paths only after their replacements and migration behavior are verified.
- [ ] Test the agreed classroom load and two-device acceptance scenarios, then
  run the named real-class pilot with baseline and follow-up measurements.
- [ ] Record which customer requirements are met, partial or deferred, remaining
  defects, per-platform scope, and the buyer's actual continuation decision.

Dependencies: P1–P7 applicable to the confirmed pilot; D01–D07 release gates.
Covers F07 and cross-cutting acceptance of C01–C10.
Exit: applicable CI passes on the final revision, native and upgrade evidence
exists, pilot results are recorded, and release is separately authorized.
Without the real pilot, label this release-candidate readiness, not validated
customer value. Optional voice deferral requires an explicit recorded scope
change; it cannot silently count C09 as fully met.

### P9 — Conditional engagement and platform expansion

- [ ] Revisit C11–C14 with the teacher after core value is demonstrated.
- [ ] Implement only chosen customization/leaderboard/follow-on activities with
  explicit pedagogical purpose, teacher control and acceptance examples.
- [ ] Qualify selected Linux environments or design an iPad-specific experience
  only if actual demand justifies their different platform capabilities.

Dependencies: pilot evidence from P8; D08 and any new platform decision.
Exit: each selected addition improves the agreed experience without weakening
teaching, help or submission. These items are recorded customer ideas, not
mandatory blockers for the first replacement release.

## Decisions still required

| ID | Missing decision | Proposed working assumption | Must be resolved before |
| --- | --- | --- | --- |
| D01 | Named class, ages, teacher, apps, device count and OS mix | Prove desktop Windows/macOS; do not assume iPads can use desktop CUA | Pilot commitment and platform scope in P8 |
| D02 | Real material formats, representative lessons and submitted artifacts | PDF, text/Markdown and links; explicit format errors; file/answer artifacts as selected | P3 ingestion scope and P6 submission implementation |
| D03 | What observations/context may be collected, shown to teachers, retained and exported; when | Explicit help context by default; bounded active-lesson observation only when agreed; no continuous unrelated monitoring | Automatic observation and sharing in P5; reporting in P7 |
| D04 | Does “tự submit” mean independent student submission or automatic submission? | Explicit student submission after preview | Any automatic submission; confirm P6 acceptance with teacher |
| D05 | Explanation/answer-reveal policy and interruption | F10 settles external-app control: learner acts; teachers cannot enable automation. Explanation depth/answer reveal still needs agreement | P3 instructional policy; never an exception to F10 |
| D06 | Distribution channel and acceptable overlay implementation | Direct desktop distribution candidate; validate packaged Tauri overlay | P1 native choice and P8 release packaging |
| D07 | Buyer, budget authority, pilot dates, baseline, capacity, latency/cost targets and success thresholds | One teacher/two devices for engineering acceptance; class pilot size and thresholds remain unset | P7 load targets and P8 pilot claims |
| D08 | Exact purpose/meaning of cursor customization, leaderboard, Scratch-like features and games | Defer until core workflow is useful | Each P9 feature |

The founder owns product decisions; the pilot teacher/center supplies classroom
constraints and confirms acceptance expectations; the implementation owner
records technical choices and evidence. An unanswered question is not approval.
Dates and thresholds must be recorded before evaluating the real pilot rather
than chosen afterward to make results look successful.

## Verification and customer-value acceptance

Each phase has two different kinds of proof: deterministic tests of contracts,
lifecycle and failure handling, plus observed behavior in the relevant packaged
application/classroom environment. Fake models can prove routing; they cannot
prove that a real model correctly understands a student's screen.

| Acceptance scenario | Required evidence | Phases |
| --- | --- | --- |
| Fresh packaged installation on Windows/macOS | Bundled runtime works without Python/Node installation; permissions, overlay, scaling and stop behave correctly | P1/P8 |
| Multi-step lesson with a moved window and wrong input | Re-observation, correct target, useful recovery, preserved lesson identity | P2 |
| Malicious instructions inside an uploaded resource | Content cannot change teacher authority, tool grants or submission policy | P3 |
| Two roles plus a cross-account access attempt | Backend denies improper publish/broadcast/read/review, independent of hidden UI | P3/P6 |
| Duplicate directive and disconnected student | Durable per-device outcome; no stale backlog execution or false success | P4 |
| Crash while showing a gesture or checking work | Cue hidden; no completion inferred; explicit resume obtains fresh observation; native mutation count remains zero | P1/P2/P8 |
| Explicit Help and quiet unresolved student | Allowed contextual evidence reaches teacher; hypotheses and unknown state remain labeled | P5 |
| Finished-but-unsubmitted, return, resubmit | Separate facts and artifact versions; no duplicate upload or accidental final grade | P6 |
| Mic disabled, quiet speech, noise and interrupted narration | Full silent path, recoverable transcription failure, no unintended dispatch | P2/P7 |
| Existing installed state/database upgrade | Data/ownership preserved, supported compatibility path and realistic rollback plan | P8 |

Record baseline and pilot values for manual discovery trips/checks per class,
time from reported or observed difficulty to useful help, repeated setup actions,
successful submissions, unresolved cases, false alerts and missed difficulties.
Also measure time to first cue, model/tool calls per lesson, provider cost,
worker failures and class-event freshness. Track the origin of difficulty
timestamps; an unobserved onset cannot be reported as an exact latency.

Use the teacher's assessment of help and actual completion evidence alongside
counts. Purchase interest, outreach volume, animated demos and passing CI alone
do not establish reduced teacher workload or improved learning.

## Migration and release discipline

Maintain a scoped old-to-new inventory for authentication, classroom roles,
materials, assignments, submissions, task history, settings, native permissions,
SDK state and updater identity. Do not copy opaque SDK checkpoints across
incompatible runtimes; preserve readable history and explicitly handle sessions
that cannot resume. Preserve any legacy unknown-action records as read-only history; the replacement must never replay them.

Published SQL migrations remain immutable. Use append-only upgrades and
upgrade-path tests; do not assume a fresh database test proves a safe upgrade.
Retire old endpoints/writers only after supported clients have a defined path.
Local user data, private objects and submission history are never disposable
just because application code is being replaced.

One orchestration path survives per supported behavior after cutover. Legacy
readers may remain only with an explicit format/version, purpose and removal
condition; they must not dispatch old actions. Keep the replacement rollout
reversible where possible, and state any irreversible migration boundary before
release. The current task authorizes the spec, not production cutover.

## Platform and packaging decision gates

Windows and macOS are the proposed first supported release targets. Qualify
Linux by desktop/window system afterward; CUA support varies by compositor.
The observed classroom included iPads. This stack does not establish iPad
desktop observation/overlay support; confirm the pilot device mix before promising coverage.

Before broad feature implementation, prove a packaged vertical slice on Windows
and macOS: launch the bundled Python/CUA worker, observe a real external app,
show a transparent click-through teaching cursor and readable explanation,
detect a student's change, and cancel/exit without orphan processes. Include
multiple monitors, different display scaling, focus behavior, capture excluding
Tro overlays where required, and native permission onboarding.

Tauri's documented transparent-window path on macOS requires its private-API
flag and is incompatible with Mac App Store acceptance. If that distribution
channel is required, evaluate an appropriate native overlay before locking the
implementation. Packaged behavior must be tested, not inferred from a dev window.

Then prove a teacher and two distinct student accounts/devices against one shared
backend: distribute material, recover one student's mistake, distinguish another
student's finished-but-unsubmitted work, escalate with context, submit, and review.
Cover disconnects, duplicate broadcasts, stop, restart, and unknown action results.

## Implementation and verification order

Founder requirement: implementation first, verification afterward. Finish each
complete agreed milestone, including regression test code and source review,
before running its verification batch. Do not repeatedly invoke typecheck,
lint, full tests, audits, or packaging after individual edits. Do not defer all
verification until the entire app is rebuilt either.

CI remains the default venue under [the verification workflow](testing/ci-workflow.md).
Run applicable TypeScript/Python type checks, Rust checks, tests, and native
packaging after implementation; collect failures and fix them in one correction
pass. Focused local checks are appropriate for diagnosing a concrete failure.
Do not claim verification passed when CI or native acceptance has not run.

Keep development commands separate from the full quality gate. Configure exact
Python/Rust/TypeScript commands and change routing with the new build system,
avoiding repeated typechecks during packaging after the source gate already
validated that revision. Final changes still require applicable passing checks.

## Evidence and references

- Customer source: NoteSystem, `20_Projects/Tro/Customer Interview - 2026-08-24.md`.
- [Open the local customer note](</Users/ducng/Desktop/workspace/NoteSystem/NoteSystem/20_Projects/Tro/Customer Interview - 2026-08-24.md>).
- Original product notes: NoteSystem, `20_Projects/Tro/docs/gtm-cofounder/founder-brief.md`
  and `gtm-roadmap.md`. One substantial interview supports the recorded pain;
  stated purchase interest is not evidence of a signed pilot or sale.
- [Open the local founder brief](</Users/ducng/Desktop/workspace/NoteSystem/NoteSystem/20_Projects/Tro/docs/gtm-cofounder/founder-brief.md>).
- Initial TroCode intent: README at commit `53e9635` (2026-08-15), with answer,
  guide, act, and mixed modes. This is historical context, not a mandate to
  restore every old runtime or approval mechanism.
- [OpenAI agent runtimes](https://developers.openai.com/api/docs/guides/agents).
- [CUA integration choices](https://cua.ai/docs/concepts/choose-a-cua-driver-integration).
- [CUA platform support](https://cua.ai/docs/reference/cua-driver/platform-support).
- [Tauri sidecar packaging](https://v2.tauri.app/develop/sidecar/).
- [Tauri window configuration](https://v2.tauri.app/reference/config/#windowconfig).

External documentation was checked during the stack discussion on 2026-09-13.
Pin and validate concrete dependency versions during implementation.

## P0 implementation notes (2026-09-13)

The source foundation was merged through PR #1 at `675df3e`.
Final PR revision `1917680` passed Linux source/integration and Windows/macOS
native build/worker CI. Pre-merge fixes moved MinIO images to pinned Quay
manifests and made interrupted workflows fail, adding two regression tests
(64 local automated cases total). Windows interactive acceptance and P1
packaging remain separate gates. The PR records the CI evidence.

Implementation refinements: MinIO replaces the LocalStack fixture candidate to
exercise signed S3 reads and anonymous-access denial. The P0 Python diagnostic
loop uses bounded synchronous stdio because every diagnostic operation completes
immediately; P1 must introduce cancellable asynchronous work before native/model
operations. The Rust actor already has separate bounded control and ordinary
queues. Status event revisions are host-owned; request/response frames use IDs
and generation isolation and do not pretend to be a durable session event stream.

Restart is a native UI command owned by the supervisor; the CLI explains how to
invoke it rather than exposing another local control listener. Account switching
invalidates the previous generation before resolving credentials; superseded
identity responses cannot select an older account. Python persists no learner
content in P0. These decisions preserve F07/F08 and C08 groundwork.


## P1 planning refinements (2026-09-13)

The subordinate execution plan preserves F01/F03/F04/F07/F09 and F08. Its
implementation baseline is one in-process CUA instance inside the existing
private Python worker, one Agents SDK session and one Python-owned SQLite
session/evidence store per account. Rust owns permissions, native presentation,
credential handling and stop supervision. No CUA daemon, native mutation dispatcher or input-control tool is introduced.

The published CUA 0.28.1 package exposes typed window observation and action
methods; its native library must ship with the matching Python bindings.
OpenAI Agents SDK 0.22.2 and PyInstaller 6.22.3 onedir are the planning pins.
Prove installed loading, not just a development import. Preserve P0's fast
preview and independent dev commands; release launch uses fixed bundled paths.

P1 uses explicit window selection, a separate click-through overlay and
visual click/drag/type/scroll guidance on an offline fixture in an external
browser. The learner performs every action. Session evidence separates rendered
cues, learner reports and fresh observations; no mutation journal is built. Full adaptive teaching stays in P2. Model access uses
short-lived account/session grants through a bounded Rust Responses gateway;
provider keys stay server-side. Controlled proof accounts are separate from
P0's three fixtures and from production classroom authentication.

Initial packaged evidence targets macOS arm64 and Windows x64. Other
architectures require separate evidence before claiming support. Direct
distribution with a Tauri overlay remains the D06 candidate, not a resolved
App Store or signing decision. PyInstaller onedir assets are staged as Tauri
resources and spawned by the existing private Rust worker actor. Signing,
permission responsibility and clean-machine acceptance remain required.

A real model backend/account, authorized signing inputs and interactive devices
are execution dependencies; their availability has not been established.
Deterministic tests and source/package implementation can proceed independently,
but fake models and hosted builds cannot satisfy the corresponding live gates.
No new deployment, provider spend, credential change or native action is
performed by the planning task.


## Interaction correction record (2026-09-13)

The founder clarified that Tro only shows where/how the learner can act. The
previous P1 draft proposed one automated demonstration and a durable mutation
journal. Those proposals are superseded, not deferred switches to enable later.
F10 applies to P1 and later phases, including classroom setup. The current P1
execution plan is rewritten around visual gestures, read-only observation and
learner-performed action checks. Archived P0 reports/plans are historical and
must not override this correction. No product code for automated actions had
been implemented when the correction was made.

### P1 implementation checkpoint — 2026-09-13

P1 source implementation is in progress on `codex/plan-native-teaching-foundation`.
The runtime exposes observation and grounded visual proposals; the learner performs all
external-app input (F10). The model uses structured output with no callable native tools.
An isolated proof gateway issues bounded model-only grants. Local SQLite stores evidence
facts separately from visual presentation and learner reports.

Implementation refinements: teaching commands return complete revisioned projections
instead of separate partial cue/check events; host generation and presentation epochs
reject stale results. CUA uses a bounded manifest intersecting inherited native policies,
with one selected-window resource. Overlay expiry is one second, with read-only refresh every 250 ms; secondary macOS displays
are withheld pending calibration. These choices do not establish full geometry acceptance.

P1 remains **in progress**. Native acceptance criteria remain open until measured
on both Windows x64 and macOS arm64. See
`docs/native-teaching-runbook.md` and `tests/acceptance/native-foundation.md`.


### Planned guidance refinement — 2026-09-13

Founder requested a seamless sequence: agent plans ahead, learner performs input,
and local observation checks progress without a model round trip per action.
The first bounded P2 slice prepares up to three semantic steps in the existing local
TeachingSession. Two fresh matching observations after a nonmatching baseline can
advance a step; uncertain outcomes require explicit learner confirmation. Pause,
resume, one bounded automatic missing-target replan and explicit replanning preserve
learner control. F10 remains unchanged.

This bounded source slice is implemented; it does not complete P1/P2 acceptance.
Verification scope and remaining acceptance are described in
[development and acceptance](architecture/development.md); PR checks record CI results.
Plans are in memory; durable plan resume, longer lessons, materials and adaptive
model-assisted replanning remain open. See [code architecture](architecture/README.md)
for module ownership, request routing, exact checking rules and limitations.


### Screenshot targeting refinement — 2026-09-13

Founder approved visual coordinate targeting alongside accessibility selectors.
Planned targets may identify a rectangle normalized to the selected-window screenshot,
allowing guidance on canvases and unlabelled controls. The runtime binds these regions
to the planning image and window geometry, maps them to native coordinates, and
invalidates them when the image or bounds change. The existing bounded replan allowance
applies; no native input capabilities are added (F10).

Screenshot-only observations may guide without complete accessibility data, but cannot
establish automatic completion from missing/partial AX evidence. Visual-only outcomes
use learner confirmation. See [visual targeting](architecture/visual-targeting.md) for
coordinate mapping, permission requirements and conservative image-comparison limits.
