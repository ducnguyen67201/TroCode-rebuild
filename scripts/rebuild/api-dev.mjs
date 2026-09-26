import { access } from 'node:fs/promises';
import { resolve } from 'node:path';
import { root } from './environment.mjs';
import { run, start, stopChild } from './processes.mjs';
import { watchChanges } from './file-watcher.mjs';

const rustSource = (name) => !name || name.endsWith('.rs');

function watchedApiSources() {
  return [
    {
      path: resolve(root, 'services/api/src'),
      recursive: true,
      accepts: rustSource,
    },
    {
      path: resolve(root, 'packages/contracts/src'),
      recursive: true,
      accepts: rustSource,
    },
    { path: resolve(root, 'services/api/Cargo.toml') },
    { path: resolve(root, 'packages/contracts/Cargo.toml') },
    { path: resolve(root, 'Cargo.toml') },
    { path: resolve(root, 'Cargo.lock') },
  ];
}

/**
 * Runs the already-built API binary and rebuilds it once per source change.
 * Both fixture and hosted-local instances share that one build, avoiding Cargo
 * package-cache locks and keeping reload behavior deterministic.
 */
export async function startApiDevServer(environment, localAuth) {
  const executable = resolve(
    root,
    'target/debug',
    process.platform === 'win32' ? 'tro-api.exe' : 'tro-api',
  );
  await access(executable);

  const specifications = [environment];
  if (localAuth) specifications.push(localAuth.environment);

  let children = [];
  let closeWatcher = () => {};
  let reloadTask = Promise.resolve();
  let reloading = false;
  let reloadQueued = false;
  let stopped = false;
  let settled = false;
  let resolveDone;
  let rejectDone;
  const done = new Promise((resolvePromise, rejectPromise) => {
    resolveDone = resolvePromise;
    rejectDone = rejectPromise;
  });

  async function stopChildren() {
    const current = children;
    children = [];
    for (const process of current) stopChild(process.child);
    await Promise.allSettled(current.map((process) => process.done));
  }

  function fail(error) {
    if (settled || stopped) return;
    settled = true;
    rejectDone(error);
    closeWatcher();
    void stopChildren();
  }

  function launch() {
    children = specifications.map((env) =>
      start(executable, ['serve'], { cwd: root, env }),
    );
    for (const process of children) {
      void process.done.then(
        () => {
          if (!reloading && !stopped)
            fail(new Error('Development API exited unexpectedly.'));
        },
        (error) => {
          if (!reloading && !stopped) fail(error);
        },
      );
    }
  }

  async function reload() {
    if (reloading) {
      reloadQueued = true;
      return;
    }
    reloading = true;
    let built = false;
    do {
      reloadQueued = false;
      await stopChildren();
      if (stopped) break;
      try {
        console.log('[dev] Rebuilding API after source change…');
        await run('cargo', ['build', '--locked', '-p', 'tro-api'], {
          cwd: root,
        });
        built = true;
      } catch (error) {
        built = false;
        console.error(
          `[dev] API rebuild failed; fix the error and save again. ${error.message}`,
        );
      }
    } while (reloadQueued && !stopped);
    if (built && !stopped) {
      launch();
      console.log('[dev] API reloaded.');
    }
    reloading = false;
    if (reloadQueued && !stopped) reloadTask = reload().catch(fail);
  }

  launch();
  closeWatcher = watchChanges(
    watchedApiSources(),
    () => {
      if (stopped) return;
      if (reloading) reloadQueued = true;
      else reloadTask = reload().catch(fail);
    },
    fail,
  );

  return {
    done,
    async stop() {
      if (stopped) return;
      stopped = true;
      closeWatcher();
      await stopChildren();
      await reloadTask.catch(() => {});
      if (!settled) {
        settled = true;
        resolveDone();
      }
    },
  };
}
