import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { randomBytes } from 'node:crypto';
import { resolve } from 'node:path';
export const root = resolve(import.meta.dirname, '../..');
export async function fixtureEnvironment() {
  await mkdir(resolve(root, '.local'), { recursive: true, mode: 0o700 });
  const path = resolve(root, '.local/environment.json');
  const secret = () => randomBytes(32).toString('hex');
  try {
    const dbPassword = secret();
    await writeFile(
      path,
      JSON.stringify(
        {
          TRO_FIXTURE_MODE: '1',
          TRO_DB_PASSWORD: dbPassword,
          DATABASE_URL: `postgres://fixture:${dbPassword}@127.0.0.1:55439/tro_rebuild_test`,
          TRO_API_BIND: '127.0.0.1:4318',
          TRO_S3_ENDPOINT: 'http://127.0.0.1:19000',
          TRO_S3_KEY: 'tro-fixture',
          TRO_S3_SECRET: secret(),
          TRO_PROFILES_FILE: resolve(root, '.local/profiles.json'),
        },
        null,
        2,
      ),
      { flag: 'wx', mode: 0o600 },
    );
  } catch (error) {
    if (error.code !== 'EEXIST') throw error;
  }
  try {
    await writeFile(
      resolve(root, '.local/profiles.json'),
      JSON.stringify(
        { teacher: secret(), 'student-a': secret(), 'student-b': secret() },
        null,
        2,
      ),
      { flag: 'wx', mode: 0o600 },
    );
  } catch (error) {
    if (error.code !== 'EEXIST') throw error;
  }
  return { ...process.env, ...JSON.parse(await readFile(path, 'utf8')) };
}
