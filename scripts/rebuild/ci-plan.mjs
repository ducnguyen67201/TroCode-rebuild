import { pathToFileURL } from 'node:url';
export function scopesFor(paths) {
  const scopes = new Set(['docs']);
  for (const path of paths) {
    if (/^(docs\/|README.md$|\.claude\/)/.test(path)) continue;
    if (
      path.startsWith('apps/desktop/src/') ||
      path === 'apps/desktop/index.html'
    )
      scopes.add('ui');
    else if (path.startsWith('services/teaching-runtime/')) {
      scopes.add('runtime');
      scopes.add('native');
    } else if (path.startsWith('services/api/')) scopes.add('api');
    else if (path.startsWith('apps/desktop/src-tauri/')) scopes.add('native');
    else return ['docs', 'ui', 'runtime', 'api', 'native', 'contracts'];
  }
  return [...scopes];
}
if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href)
  console.log(JSON.stringify(scopesFor(process.argv.slice(2))));
