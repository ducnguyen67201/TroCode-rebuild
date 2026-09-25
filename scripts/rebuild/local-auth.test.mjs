import assert from 'node:assert/strict';
import { createServer } from 'node:http';
import test from 'node:test';
import { localAuthService, waitForReady } from './local-auth.mjs';

const configured = () => ({
  DATABASE_URL: 'postgres://fixture:secret@127.0.0.1:55439/tro_rebuild_test',
  TRO_API_BIND: '127.0.0.1:4318',
  TRO_AUTH_API_ORIGIN: 'http://127.0.0.1:4319',
  TRO_AUTH_ISSUER: 'https://api.tro.test',
  TRO_AUTH_AUDIENCE: 'tro-desktop-api',
  TRO_GOOGLE_CLIENT_ID: 'desktop.apps.googleusercontent.com',
  TRO_JWT_KEY_B64: 'jwt',
  TRO_REFRESH_KEY_B64: 'refresh',
});

test('configures the loopback hosted auth service alongside the fixture API', () => {
  const fixture = configured();
  const service = localAuthService(fixture);

  assert.equal(service.origin, 'http://127.0.0.1:4319');
  assert.equal(service.environment.TRO_API_MODE, 'hosted');
  assert.equal(service.environment.TRO_API_BIND, '127.0.0.1:4319');
  assert.equal(service.environment.TRO_ALLOW_HOSTED_MIGRATION, '1');
  assert.equal(service.environment.DATABASE_URL, fixture.DATABASE_URL);
});

test('does not launch a local service for absent or remote auth origins', () => {
  const fixture = configured();
  assert.equal(
    localAuthService({ ...fixture, TRO_AUTH_API_ORIGIN: undefined }),
    null,
  );
  assert.equal(
    localAuthService({
      ...fixture,
      TRO_AUTH_API_ORIGIN: 'https://api.tro.example',
    }),
    null,
  );
});

test('fails early when loopback auth is only partially configured', () => {
  const fixture = configured();
  delete fixture.TRO_REFRESH_KEY_B64;
  assert.throws(
    () => localAuthService(fixture),
    /configuration is incomplete: TRO_REFRESH_KEY_B64/,
  );
});

test('waits for the hosted auth service before desktop startup', async (context) => {
  const server = createServer((request, response) => {
    response.writeHead(request.url === '/readyz' ? 200 : 404).end();
  });
  await new Promise((resolve) => server.listen(0, '127.0.0.1', resolve));
  context.after(() => server.close());
  const address = server.address();

  await waitForReady(
    `http://127.0.0.1:${address.port}`,
    new Promise(() => {}),
    1,
  );
});

test('reports a hosted auth process that exits before readiness', async () => {
  await assert.rejects(
    waitForReady('http://127.0.0.1:9', Promise.reject(new Error('exit')), 1),
    /exited during startup/,
  );
});
