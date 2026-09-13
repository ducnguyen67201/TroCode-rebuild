import test from 'node:test';
import assert from 'node:assert/strict';
import { run, start, stopChild } from './processes.mjs';
import { mkdtemp, writeFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
test('propagates a failed child command', async () => {
  await assert.rejects(run(process.execPath, ['-e', 'process.exit(3)']));
});
test('rejects an externally terminated child', async () => {
  const job = start(process.execPath, ['-e', 'setInterval(() => {}, 1000)']);
  const rejected = assert.rejects(job.done);
  await new Promise((resolve) => job.child.once('spawn', resolve));
  job.child.kill('SIGTERM');
  await rejected;
});
test('interrupting the CLI fails the build and prevents later stages', {
  skip: process.platform === 'win32',
  timeout: 10000,
}, async () => {
  const directory = await mkdtemp(join(tmpdir(), 'tro-interrupt-'));
  const fakeNpm = join(directory, 'npm.cjs');
  await writeFile(fakeNpm, "console.log('ready'); setInterval(() => {}, 1000);");
  const job = start(process.execPath, ['scripts/rebuild/cli.mjs', 'build'], {
    cwd: resolve(import.meta.dirname, '../..'),
    env: { ...process.env, npm_execpath: fakeNpm },
    stdio: ['ignore', 'pipe', 'pipe'],
  });
  const rejected = assert.rejects(job.done, /exited with 143/);
  try {
    await new Promise((resolve, reject) => {
      let output = '';
      job.child.stdout.on('data', (chunk) => {
        output += chunk;
        if (output.includes('ready')) resolve();
      });
      job.child.once('error', reject);
      job.child.once('exit', () => reject(new Error('CLI exited before ready')));
    });
    job.child.kill('SIGTERM');
    await rejected;
  } finally {
    stopChild(job.child);
    await rm(directory, { recursive: true, force: true });
  }
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
