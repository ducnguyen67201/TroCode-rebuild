import { readFile, mkdtemp, cp, rm } from 'node:fs/promises';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { createHash, randomUUID } from 'node:crypto';
import { spawn, execFile } from 'node:child_process';
import { promisify } from 'node:util';
import { staging, filesUnder } from './package-runtime.mjs';
const infoPlist = await readFile(
  join(process.cwd(), 'apps/desktop/src-tauri/Info.plist'),
  'utf8',
);
if (!infoPlist.includes('<key>NSMicrophoneUsageDescription</key>'))
  throw new Error('Packaged macOS microphone purpose string is missing.');
const manifest = JSON.parse(
  await readFile(join(staging, 'manifest.json'), 'utf8'),
);
if (manifest.platform !== process.platform || manifest.arch !== process.arch)
  throw new Error('Runtime platform mismatch.');
const actual = await filesUnder(join(staging, 'tro-runtime'));
if (actual.length !== Object.keys(manifest.artifacts).length)
  throw new Error('Runtime manifest file count mismatch.');
for (const [path, digest] of Object.entries(manifest.artifacts)) {
  if (path.includes('..') || !path.startsWith('tro-runtime/'))
    throw new Error('Unsafe manifest path.');
  if (
    createHash('sha256')
      .update(await readFile(join(staging, path)))
      .digest('hex') !== digest
  )
    throw new Error('Runtime asset mismatch.');
}
const destination = await mkdtemp(join(tmpdir(), 'Tro relocated runtime '));
try {
  await cp(join(staging, 'tro-runtime'), join(destination, 'tro-runtime'), {
    recursive: true,
  });
  const executable = join(
    destination,
    'tro-runtime',
    process.platform === 'win32' ? 'tro-runtime.exe' : 'tro-runtime',
  );
  const env = Object.fromEntries(
    ['SYSTEMROOT', 'WINDIR', 'TEMP', 'TMP', 'HOME', 'USERPROFILE']
      .filter((key) => process.env[key])
      .map((key) => [key, process.env[key]]),
  );
  await new Promise((resolve, reject) => {
    const child = spawn(executable, [], {
      cwd: destination,
      env,
      stdio: ['pipe', 'pipe', 'ignore'],
    });
    const deadline = setTimeout(() => {
      child.kill();
      reject(new Error('Packaged handshake timed out.'));
    }, 15000);
    let output = '';
    let ready = false;
    const generationId = randomUUID();
    const frame = (kind) => ({
      protocolVersion: 3,
      kind: `runtime.${kind}`,
      requestId: randomUUID(),
      correlationId: randomUUID(),
      generationId,
    });
    child.stdout.on('data', (bytes) => {
      output += bytes;
      if (output.length > 262144) {
        child.kill();
        return;
      }
      let index;
      while ((index = output.indexOf('\n')) >= 0) {
        let value;
        try {
          value = JSON.parse(output.slice(0, index));
        } catch {
          child.kill();
          return;
        }
        output = output.slice(index + 1);
        if (
          value.kind === 'runtime.ready' &&
          value.schemaDigest === manifest.protocolDigest
        ) {
          ready = true;
          child.stdin.write(JSON.stringify(frame('shutdown')) + '\n');
        }
      }
    });
    child.on('error', (error) => {
      clearTimeout(deadline);
      reject(error);
    });
    child.on('close', (code) => {
      clearTimeout(deadline);
      if (code === 0 && ready) resolve();
      else reject(new Error('Packaged runtime handshake failed.'));
    });
    child.stdin.write(
      JSON.stringify({
        ...frame('initialize'),
        schemaDigest: manifest.protocolDigest,
        accountId: null,
      }) + '\n',
    );
  });
  const selfCheck = await promisify(execFile)(executable, ['--self-check'], {
    cwd: destination,
    env,
    timeout: 15000,
    maxBuffer: 65536,
  });
  const result = JSON.parse(selfCheck.stdout);
  if (
    result.schemaDigest !== manifest.protocolDigest ||
    result.nativePolicyLoaded !== true
  )
    throw new Error('Packaged native policy self-check failed.');
  console.log(
    'Relocated handshake and native policy load passed; live capture and signing require separate evidence.',
  );
} finally {
  await rm(destination, { recursive: true, force: true });
}
