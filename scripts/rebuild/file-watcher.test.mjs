import assert from 'node:assert/strict';
import { mkdtemp, mkdir, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';
import { watchChanges } from './file-watcher.mjs';

test('debounces relevant source changes and stops cleanly', async () => {
  const directory = await mkdtemp(join(tmpdir(), 'tro-watch-'));
  const source = join(directory, 'src');
  await mkdir(source);
  let changes = 0;
  let resolveChange;
  const changed = new Promise((resolve) => {
    resolveChange = resolve;
  });
  const close = watchChanges(
    [
      {
        path: source,
        recursive: true,
        accepts: (name) => name.endsWith('.rs'),
      },
    ],
    () => {
      changes += 1;
      resolveChange();
    },
    (error) => assert.fail(error),
    20,
  );
  try {
    await writeFile(join(source, 'ignored.txt'), 'ignored');
    await writeFile(join(source, 'main.rs'), 'fn main() {}');
    await writeFile(join(source, 'main.rs'), 'fn main() { println!("ok"); }');
    await Promise.race([
      changed,
      new Promise((_, reject) =>
        setTimeout(() => reject(new Error('watch timed out')), 2_000),
      ),
    ]);
    await new Promise((resolve) => setTimeout(resolve, 60));
    assert.equal(changes, 1);
    close();
    await writeFile(join(source, 'after.rs'), 'fn after() {}');
    await new Promise((resolve) => setTimeout(resolve, 60));
    assert.equal(changes, 1);
  } finally {
    close();
    await rm(directory, { recursive: true, force: true });
  }
});
