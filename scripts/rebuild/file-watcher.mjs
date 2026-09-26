import { watch } from 'node:fs';

/** Debounced development watcher. Production code never imports this module. */
export function watchChanges(entries, onChange, onError, debounceMs = 160) {
  let timer;
  let closed = false;
  let pending;
  const watchers = entries.map(({ path, recursive = false, accepts }) => {
    const watcher = watch(path, { recursive }, (_event, filename) => {
      const relative = filename?.toString() ?? '';
      if (closed || (accepts && !accepts(relative))) return;
      pending = relative ? `${path}/${relative}` : path;
      clearTimeout(timer);
      timer = setTimeout(() => {
        timer = undefined;
        const changed = pending;
        pending = undefined;
        if (!closed && changed) onChange(changed);
      }, debounceMs);
    });
    watcher.on('error', onError);
    return watcher;
  });

  return () => {
    closed = true;
    clearTimeout(timer);
    for (const watcher of watchers) watcher.close();
  };
}
