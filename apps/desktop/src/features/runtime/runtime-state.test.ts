import { expect, it } from 'vitest';
import { latestStatus, publicError } from './runtime-state';
import { createPreviewClient } from '../../platform/preview-client';
it('keeps newer snapshots across subscription races', async () => {
  const client = createPreviewClient();
  const old = await client.status();
  const latest = await client.start();
  expect(latestStatus(latest, old)).toBe(latest);
});
it('does not expose error payloads', () => {
  expect(publicError({ message: 'secret' })).not.toContain('secret');
  expect(publicError({ code: 'TIMEOUT' })).toContain('timed out');
});
it('preview stops and unsubscribes', async () => {
  const client = createPreviewClient();
  let count = 0;
  const off = await client.subscribe(() => count++);
  await client.start();
  off();
  await client.stop();
  expect(count).toBe(1);
  expect((await client.status()).state).toBe('stopped');
});
