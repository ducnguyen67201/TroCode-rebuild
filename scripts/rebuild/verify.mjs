import { resolve } from 'node:path';
import { run } from './processes.mjs';
import { root } from './environment.mjs';
export async function verify(args) {
  const index = args.indexOf('--scope');
  const scope = index < 0 ? 'all' : args[index + 1];
  if (
    !['all', 'ui', 'runtime', 'api', 'native', 'contracts', 'docs'].includes(
      scope,
    )
  )
    throw new Error('Unknown verification scope.');
  const npm = (script) =>
    run(process.execPath, [process.env.npm_execpath, 'run', script], {
      cwd: root,
    });
  const python = (tool, ...arguments_) =>
    run('uv', ['run', '--locked', tool, ...arguments_], {
      cwd: resolve(root, 'services/teaching-runtime'),
    });
  const failures = [];
  async function check(name, execute) {
    try {
      await execute();
    } catch {
      failures.push(name);
    }
  }
  await check('script tests', () => npm('test:scripts'));
  if (scope === 'docs') {
    if (failures.length) throw new Error('Script checks failed.');
    return;
  }
  if (['all', 'contracts'].includes(scope))
    await check('contract drift', () => npm('contracts:check'));
  if (['all', 'ui', 'contracts'].includes(scope)) {
    await check('lint', () => npm('lint'));
    await check('typecheck', () => npm('typecheck'));
    await check('TypeScript tests', () => npm('test:typescript'));
    await check('browser smoke', () => npm('test:browser'));
    await check('UI build', () =>
      run(
        process.execPath,
        [process.env.npm_execpath, 'run', 'build', '-w', '@tro/desktop'],
        { cwd: root },
      ),
    );
  }
  if (['all', 'runtime', 'contracts', 'native'].includes(scope)) {
    await check('Ruff', () => python('ruff', 'check', '.'));
    await check('Python format', () =>
      python('ruff', 'format', '--check', '.'),
    );
    await check('mypy', () => python('mypy', 'src'));
    await check('pytest', () => python('pytest'));
  }
  const packages =
    scope === 'api'
      ? ['-p', 'tro-api', '-p', 'tro-contracts']
      : scope === 'contracts'
        ? ['-p', 'tro-contracts']
        : [
            '-p',
            'tro-api',
            '-p',
            'tro-contracts',
            '-p',
            'tro-desktop',
            '--no-default-features',
          ];
  if (['all', 'api', 'contracts', 'native', 'runtime'].includes(scope)) {
    await check('Rust format', () =>
      run('cargo', ['fmt', '--all', '--', '--check'], { cwd: root }),
    );
    await check('clippy', () =>
      run(
        'cargo',
        [
          'clippy',
          ...packages,
          '--all-targets',
          '--locked',
          '--',
          '-D',
          'warnings',
        ],
        { cwd: root },
      ),
    );
    await check('Rust tests', () =>
      run('cargo', ['test', ...packages, '--locked'], { cwd: root }),
    );
  }
  if (scope === 'all') {
    await check('integration', () => npm('test:integration'));
    await check('native build', () =>
      run('cargo', ['build', '--locked', '-p', 'tro-desktop'], { cwd: root }),
    );
    await check('npm audit', () =>
      run(process.execPath, [process.env.npm_execpath, 'audit'], { cwd: root }),
    );
    await check('Python audit', () => python('pip-audit'));
  }
  if (failures.length)
    throw new Error(`Failed verification checks: ${failures.join(', ')}`);
}
