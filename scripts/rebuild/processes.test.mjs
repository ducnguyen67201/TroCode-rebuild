import test from 'node:test';
import assert from 'node:assert/strict';
import { run, start, stopChild } from './processes.mjs';
test('propagates a failed child command', async () => {
  await assert.rejects(run(process.execPath, ['-e', 'process.exit(3)']));
});
test('stops its owned child', async () => {
  const job = start(process.execPath, ['-e', 'setInterval(() => {}, 1000)']);
  await new Promise((resolve) => job.child.once('spawn', resolve));
  stopChild(job.child);
  await job.done.catch((error) => {
    if (process.platform !== 'win32') throw error;
  });
  assert.ok(job.child.exitCode !== null || job.child.signalCode !== null);
});
