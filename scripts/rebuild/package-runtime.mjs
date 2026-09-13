import { readdir, readFile, writeFile, mkdir, rm } from 'node:fs/promises';
import { resolve, join, relative } from 'node:path';
import { createHash } from 'node:crypto';
import { run } from './processes.mjs';
import { root } from './environment.mjs';
export const staging = resolve(
  root,
  'apps/desktop/src-tauri/resources/runtime',
);
export async function filesUnder(directory) {
  const result = [];
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) result.push(...(await filesUnder(path)));
    else if (entry.isFile()) result.push(path);
    else
      throw new Error('Runtime staging cannot contain links or special files.');
  }
  return result.sort();
}
export async function packageRuntime() {
  if (!['darwin', 'win32'].includes(process.platform))
    throw new Error('Build the native runtime on macOS or Windows.');
  await mkdir(staging, { recursive: true });
  await rm(join(staging, 'manifest.json'), { force: true });
  await run(
    'uv',
    [
      'run',
      '--project',
      'services/teaching-runtime',
      '--locked',
      'pyinstaller',
      '--noconfirm',
      '--clean',
      '--distpath',
      staging,
      '--workpath',
      resolve(root, '.local/runtime-build'),
      'services/teaching-runtime/packaging/runtime.spec',
    ],
    { cwd: root },
  );
  const artifacts = {};
  for (const path of await filesUnder(join(staging, 'tro-runtime'))) {
    artifacts[relative(staging, path).replaceAll('\\', '/')] = createHash(
      'sha256',
    )
      .update(await readFile(path))
      .digest('hex');
  }
  await writeFile(
    join(staging, 'manifest.json'),
    JSON.stringify(
      {
        version: 1,
        platform: process.platform,
        arch: process.arch,
        protocolDigest: (
          await readFile(
            resolve(root, 'packages/contracts/schema/digest.txt'),
            'utf8',
          )
        ).trim(),
        artifacts,
      },
      null,
      2,
    ) + '\n',
  );
}
if (process.argv[1] === import.meta.filename) await packageRuntime();
