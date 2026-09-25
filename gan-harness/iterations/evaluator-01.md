# Evaluator iteration 01

## Scores

- Design Quality: 7.4 / 10, weighted 2.59
- Originality: 8.1 / 10, weighted 2.43
- Craft: 4.8 / 10, weighted 1.20
- Functionality: 3.2 / 10, weighted 0.32

Weighted total: 6.54 / 10

Verdict: ITERATE

## Strongest qualities

- The "path into the lesson" threshold is a memorable learning-adjacent idea and avoids the generic centered OAuth card pattern.
- The gate is structurally well placed: `AuthGate` renders protected children only after authenticated workspace access, exposes sign out through the authenticated shell, and keeps React token-free.
- Public state coverage is thoughtful. Signed out, checking, browser handoff, membership pending, offline, and error all share one visual language with clear primary actions.

## Deficiencies

1. The auth UI currently fails to render in browser preview because AJV strict mode rejects the auth schema. `npm run test:browser -- --grep "signed-out threshold"` fails with `strict mode: missing type "array" for keyword "minItems"` at `packages/contracts/schema/auth.schema.json` inside the authenticated conditional. This blocks visual review and user access entirely.
2. The conditional schema branches repeat constraints on `workspaces` without a local `"type": "array"` at `packages/contracts/schema/auth.schema.json:91` and the membership branch has the same strict-type issue for `maxItems` at the matching `workspaces` conditional. This means the shared contract is not usable under the repo's current strict AJV validator.
3. Because the app crashes before rendering, the claimed narrow viewport behavior, overflow safety, and state transitions are not presently proven by the existing Playwright check.
4. The source shows strong typography direction, but several negative letter-spacing rules are applied to compact and responsive text. That increases risk of readability/fitting issues at high zoom and runs against the frontend design constraint that letter spacing should be 0.
5. The visual concept is strong, but it still needs one confirmed screenshot pass across signed-out, membership-required, offline, and authenticated states after the schema fix to verify the map, waypoints, footer, and primary action never collide on 390px and desktop.

## Revision brief

Fix the auth schema strict-mode blocker first by making the conditional `workspaces` constraints valid under AJV strict types, regenerate/check the contracts, and rerun the focused browser auth tests. Then do one visual polish pass that removes negative letter spacing, confirms the map composition at 390x844 and desktop, and captures all required auth states. If those checks pass without overflow or runtime errors, this design is likely close to a PASS.
