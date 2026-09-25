import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { assessEvidence } from './native-evidence.mjs';
const schema = JSON.parse(
  await readFile(
    new URL(
      '../../tests/acceptance/native-proof-evidence.schema.json',
      import.meta.url,
    ),
  ),
);
const fixture = () => ({
  version: 2,
  commit: 'a'.repeat(40),
  os: 'macOS',
  arch: 'arm64',
  installerSha256: 'b'.repeat(64),
  signing: 'unsigned',
  tester: 'fixture',
  observedAt: '2026-09-13T00:00:00Z',
  checks: Object.fromEntries(
    schema.oneOf[1].properties.checks.required.map((name) => [
      name,
      { status: 'pass', reason: 'Synthetic validator test only.' },
    ]),
  ),
  timings: {
    localTransition: Array(20).fill(500),
    stop: Array(10).fill(300),
    model: [2000],
  },
});
test('requires complete explicit evidence and measured latency', () => {
  const value = fixture();
  assert.equal(assessEvidence(value).accepted, true);
  value.checks.realModel.status = 'not_run';
  assert.equal(assessEvidence(value).accepted, false);
  value.checks.realModel.status = 'pass';
  value.timings.stop = [];
  assert.equal(assessEvidence(value).accepted, false);
  value.timings.stop = Array(10).fill(1001);
  assert.equal(assessEvidence(value).accepted, false);
});
test('rejects extra content and accepts historical evidence only as historical', () => {
  const value = fixture();
  value.secret = 'must not be permitted';
  assert.equal(assessEvidence(value).valid, false);
  const historic = fixture();
  delete historic.version;
  delete historic.timings;
  historic.checks = Object.fromEntries(
    schema.oneOf[0].properties.checks.required.map((name) => [name, true]),
  );
  assert.deepEqual(assessEvidence(historic).valid, true);
  assert.deepEqual(assessEvidence(historic).accepted, false);
});
