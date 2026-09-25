import { readFile, writeFile, mkdtemp, rm, copyFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { resolve, join } from 'node:path';
import { createHash } from 'node:crypto';
import { compile } from 'json-schema-to-typescript';
import { spawnSync } from 'node:child_process';
const check = process.argv.includes('--check');
const root = resolve(import.meta.dirname, '../..');
const temporary = await mkdtemp(join(tmpdir(), 'tro-contracts-'));
const canonical = (value) =>
  Array.isArray(value)
    ? value.map(canonical)
    : value && typeof value === 'object'
      ? Object.fromEntries(
          Object.keys(value)
            .sort()
            .map((key) => [key, canonical(value[key])]),
        )
      : value;
async function output(relative, bytes) {
  const path = join(root, relative);
  if (check) {
    if ((await readFile(path, 'utf8')) !== bytes)
      throw new Error(`Generated drift: ${relative}`);
  } else await writeFile(path, bytes);
}
function run(command, args) {
  const result = spawnSync(command, args, {
    cwd: root,
    stdio: 'inherit',
    shell: false,
  });
  if (result.error || result.status !== 0)
    throw new Error(`Code generation failed: ${command}`);
}
try {
  for (const [schemaName, suffix] of [
    ['protocol', ''],
    ['status', '-status'],
    ['model-access', '-model-access'],
    ['auth', '-auth'],
    ['workspace', '-workspace'],
    ['voice', '-voice'],
  ]) {
    const source = join(
      root,
      `packages/contracts/schema/${schemaName}.schema.json`,
    );
    const schema = JSON.parse(await readFile(source, 'utf8'));
    await output(
      `packages/contracts/src/generated${suffix}.ts`,
      await compile(schema, schema.title, {
        bannerComment: '/* Generated. Do not edit. */',
        style: { singleQuote: true },
      }),
    );
    if (schemaName === 'voice') {
      const rust = join(temporary, 'generated_voice.rs');
      run('cargo', [
        'run',
        '--quiet',
        '--locked',
        '-p',
        'tro-contracts',
        '--features',
        'codegen',
        '--bin',
        'contracts-gen',
        '--',
        source,
        rust,
      ]);
      await output(
        'packages/contracts/src/generated_voice.rs',
        await readFile(rust, 'utf8'),
      );
    }
    if (schemaName !== 'protocol') continue;
    const digest = createHash('sha256')
      .update(JSON.stringify(canonical(schema)))
      .digest('hex');
    await output('packages/contracts/schema/digest.txt', digest);
    await output(
      'services/teaching-runtime/src/tro_runtime/schema.json',
      JSON.stringify(schema, null, 2) + '\n',
    );
    await output(
      'services/teaching-runtime/src/tro_runtime/digest.txt',
      digest,
    );
    const rust = join(temporary, 'generated.rs');
    run('cargo', [
      'run',
      '--quiet',
      '--locked',
      '-p',
      'tro-contracts',
      '--features',
      'codegen',
      '--bin',
      'contracts-gen',
      '--',
      source,
      rust,
    ]);
    await output(
      'packages/contracts/src/generated.rs',
      await readFile(rust, 'utf8'),
    );
    const python = join(temporary, 'generated.py');
    // Stable basename prevents absolute checkout paths from entering generated source.
    await copyFile(source, join(temporary, 'protocol.schema.json'));
    run('uv', [
      'run',
      '--project',
      'services/teaching-runtime',
      '--locked',
      'datamodel-codegen',
      '--input',
      join(temporary, 'protocol.schema.json'),
      '--input-file-type',
      'jsonschema',
      '--output-model-type',
      'pydantic_v2.BaseModel',
      '--disable-timestamp',
      '--formatters',
      'black',
      'isort',
      '--output',
      python,
    ]);
    await output(
      'services/teaching-runtime/src/tro_runtime/generated.py',
      await readFile(python, 'utf8'),
    );
  }
} finally {
  await rm(temporary, { recursive: true, force: true });
}
