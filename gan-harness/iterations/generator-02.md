# Generator iteration 02

## Revision response

Iteration 02 keeps the editorial threshold and topographic learning path while
addressing the evaluator's craft concerns directly.

- Removed every negative letter-spacing value from both the authentication
  threshold and authenticated shell. Display type now uses natural glyph spacing
  (`letter-spacing: 0`) for better readability and high-zoom behavior.
- Changed the threshold from clipped overflow to horizontal containment with
  vertical scrolling. Short windows and high zoom can reach every action and
  status without allowing decorative map geometry to widen the document.
- Moved the single-column composition breakpoint to 960px, before the two fixed
  grid minima can collide with page gutters.
- Shortened the offline display line to preserve the intended two-line rhythm at
  390px without shrinking the type.
- Gave the authenticated account header an explicit narrow grid: workspace
  context occupies its own row, with avatar, account name and sign out beneath.
- Added a focused Playwright matrix for signed-out, membership-required, offline
  and authenticated states at 390×844 and 1280×900. Each case asserts its state
  marker, checks horizontal overflow and records a full-page screenshot in the
  ignored `.local/` review directory.
- The first screenshot pass exposed a collapsed label in non-Google actions;
  retry buttons now retain the same three-column icon/text/arrow grid using an
  explicit empty leading cell.

## Preserved qualities

The cream/forest/rust palette, local serif editorial typography, drawn route,
workbook annotations, restrained motion and complete reduced-motion path remain
unchanged. No remote asset or token-bearing renderer surface was introduced.

## Focused verification

- `npx vitest run apps/desktop/src/features/auth/AuthGate.test.tsx apps/desktop/src/features/auth/auth-state.test.ts`
  — 2 files, 7 tests passed.
- `npm run test:browser -- --grep "auth layouts remain"` — 1 focused
  Playwright test passed, rendering eight state/viewport combinations without
  page errors or horizontal overflow.
- Screenshot review confirmed each primary action remains above the fold at
  390×844, the holding-state identity and map do not collide, and the signed-in
  account header resolves into two deliberate rows.
