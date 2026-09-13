# Foundation verification

Finish coherent implementation, test code and source review first. Then run the consolidated gate once; collect failures, fix them together and rerun affected checks. No full gate in save hooks, HMR or normal startup.

`npm run verify -- --scope all` runs script tests, schema drift, TypeScript lint/typecheck/tests/browser smoke/UI build, Python Ruff/format/mypy/tests, Rust format/clippy/tests, DB/S3 integration, native development build and npm/Python dependency audits. Individual scopes are `ui`, `runtime`, `api`, `native`, `contracts`, `docs`. Rust process tests run without the desktop feature in the source gate; target-specific desktop builds are separate.

CI definitions live in `.github/workflows/foundation.yml`; `scripts/rebuild/ci-plan.mjs` maps changed paths. Documentation skips application gates; UI changes use frontend gates; Python/native changes include process checks; API changes include scoped backend tests/integration; root/lock/contracts/unknown changes select the conservative set. No deployment or release action exists.

CI is the default once an authorized remote is configured. The explicit `prp-implement` request on 2026-09-13 authorizes its final local consolidated validation pass for this milestone. CI execution itself remains pending until a remote/push is separately authorized. A local pass is not a claim that CI or Windows interactive acceptance ran.

`build` performs required compilation only. `package` reports its P1 prerequisites until native Python/CUA bundling is implemented. Source/type checks are not repeated inside packaging. Fake fixtures establish routing and failure handling, not real screen understanding or learning.

Upgrade tests against real legacy data and packaged Windows/macOS acceptance belong to their master-spec gates. No current check establishes safe production migration. All applicable final-revision checks must pass before merge/release; unavailable checks remain explicitly pending.

Browser smoke uses pinned Playwright and Chromium. Install the browser once with
`npx playwright install chromium` before local browser checks. CI installs it
only for frontend/native/contract source changes; it is not a dev prerequisite.
