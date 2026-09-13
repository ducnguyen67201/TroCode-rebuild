import { resolve } from 'node:path';
import { access } from 'node:fs/promises';
import {
  run,
  start,
  stopOwned,
  stopChild,
  interruptOwned,
} from './processes.mjs';
import { root, fixtureEnvironment } from './environment.mjs';
import { verify } from './verify.mjs';
const action = process.argv[2];
const pythonProject = resolve(root, 'services/teaching-runtime');
const vite = resolve(root, 'node_modules/vite/bin/vite.js');
const node = process.execPath;
const npmCli = process.env.npm_execpath;
const npm = (args) => run(node, [npmCli, ...args], { cwd: root });
const compose = ['compose', '-f', resolve(root, 'infra/compose.test.yml')];
for (const [signal, exitCode] of [
  ['SIGINT', 130],
  ['SIGTERM', 143],
]) {
  process.on(signal, () => {
    process.exitCode = exitCode;
    interruptOwned(signal);
  });
}
async function databaseUp(env) {
  await run(
    'docker',
    [...compose, 'up', '-d', '--wait', 'postgres', 'storage'],
    { env },
  );
  await run('docker', [...compose, 'run', '--rm', 'bucket'], { env });
}
async function apiCommand(command, env) {
  await run('cargo', ['run', '--locked', '-p', 'tro-api', '--', command], {
    cwd: root,
    env,
  });
}
async function prepareApi() {
  const env = await fixtureEnvironment();
  await databaseUp(env);
  await apiCommand('migrate', env);
  await apiCommand('seed', env);
  return env;
}
async function desktop() {
  const python = resolve(
    pythonProject,
    process.platform === 'win32'
      ? '.venv/Scripts/python.exe'
      : '.venv/bin/python',
  );
  await access(python).catch(() => {
    throw new Error('Run npm run setup first.');
  });
  const ui = start(node, [vite, '--host', '127.0.0.1'], {
    cwd: resolve(root, 'apps/desktop'),
    env: { ...process.env, VITE_TRO_PREVIEW: '0' },
  });
  const tauri = start(
    node,
    [resolve(root, 'node_modules/@tauri-apps/cli/tauri.js'), 'dev'],
    { cwd: resolve(root, 'apps/desktop') },
  );
  try {
    await Promise.race([ui.done, tauri.done]);
  } finally {
    stopChild(ui.child);
    stopChild(tauri.child);
    await Promise.allSettled([ui.done, tauri.done]);
  }
}
try {
  switch (action) {
    case 'setup':
      await npm(['ci', '--no-audit']);
      await run('uv', ['sync', '--project', pythonProject, '--locked']);
      await fixtureEnvironment();
      console.log('Dependencies ready. Use dev:ui, dev, or dev:api.');
      break;
    case 'dev:ui':
      await run(node, [vite, '--host', '127.0.0.1'], {
        cwd: resolve(root, 'apps/desktop'),
        env: { ...process.env, VITE_TRO_PREVIEW: '1' },
      });
      break;
    case 'dev':
      await desktop();
      break;
    case 'dev:api': {
      const env = await prepareApi();
      await apiCommand('serve', env);
      break;
    }
    case 'dev:full': {
      const env = await prepareApi();
      const api = start(
        'cargo',
        ['run', '--locked', '-p', 'tro-api', '--', 'serve'],
        { cwd: root, env },
      );
      try {
        await Promise.race([api.done, desktop()]);
      } finally {
        stopOwned();
        await Promise.allSettled([api.done]);
      }
      break;
    }
    case 'dev:runtime:restart':
      console.log(
        'Use “Restart runtime” in the desktop. Only its Rust host owns the private worker; no external restart listener is exposed.',
      );
      break;
    case 'db:up':
      await databaseUp(await fixtureEnvironment());
      break;
    case 'db:migrate':
      await apiCommand('migrate', await fixtureEnvironment());
      break;
    case 'db:seed':
      await apiCommand('seed', await fixtureEnvironment());
      break;
    case 'env:check': {
      const response = await fetch('http://127.0.0.1:4318/readyz', {
        signal: AbortSignal.timeout(5000),
      });
      if (!response.ok) throw new Error('Fixture API is not ready.');
      console.log(await response.text());
      break;
    }
    case 'test:integration': {
      const env = await prepareApi();
      await run(
        'cargo',
        [
          'test',
          '--locked',
          '-p',
          'tro-api',
          '--test',
          'foundation',
          '--',
          '--ignored',
          '--test-threads=1',
        ],
        { cwd: root, env },
      );
      await run(
        'cargo',
        [
          'test',
          '--locked',
          '-p',
          'tro-api',
          '--lib',
          '--',
          '--ignored',
          '--test-threads=1',
        ],
        { cwd: root, env },
      );
      break;
    }
    case 'build':
      await npm(['run', 'build', '-w', '@tro/desktop']);
      await run('cargo', ['build', '--workspace', '--locked'], { cwd: root });
      break;
    case 'verify':
      await verify(process.argv.slice(3));
      break;
    case 'package':
      await npm(['run', 'package:runtime']);
      await npm(['run', 'package:check']);
      await npm(['run', 'build', '-w', '@tro/desktop']);
      await run(
        node,
        [
          resolve(root, 'node_modules/@tauri-apps/cli/tauri.js'),
          'build',
          '--config',
          'src-tauri/tauri.package.conf.json',
          '--bundles',
          process.platform === 'win32' ? 'nsis' : 'app,dmg',
        ],
        { cwd: resolve(root, 'apps/desktop') },
      );
      break;
    default:
      throw new Error(`Unknown development command: ${action}`);
  }
} catch (error) {
  console.error(error.message);
  stopOwned();
  process.exitCode ||= 1;
}
