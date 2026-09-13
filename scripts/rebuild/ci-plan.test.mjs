import test from 'node:test';
import assert from 'node:assert/strict';
import { scopesFor } from './ci-plan.mjs';
for (const [paths, required] of [
  [['docs/file.md'], ['docs']],
  [['apps/desktop/src/App.tsx'], ['docs', 'ui']],
  [
    ['services/teaching-runtime/src/tro_runtime/runtime.py'],
    ['docs', 'runtime', 'native'],
  ],
  [['services/api/src/auth.rs'], ['docs', 'api']],
  [['apps/desktop/src-tauri/src/worker.rs'], ['docs', 'native']],
])
  test(`routing ${paths[0]}`, () =>
    assert.deepEqual(scopesFor(paths), required));
for (const path of [
  'Cargo.lock',
  'package.json',
  'packages/contracts/schema/protocol.schema.json',
  'unknown/path',
])
  test(`conservative routing ${path}`, () =>
    assert.ok(
      scopesFor([path]).includes('native') &&
        scopesFor([path]).includes('contracts'),
    ));
