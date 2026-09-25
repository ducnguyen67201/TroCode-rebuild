const requiredHostedValues = [
  'TRO_AUTH_ISSUER',
  'TRO_AUTH_AUDIENCE',
  'TRO_GOOGLE_CLIENT_ID',
  'TRO_JWT_KEY_B64',
  'TRO_REFRESH_KEY_B64',
];

export function localAuthService(environment) {
  const configuredOrigin = environment.TRO_AUTH_API_ORIGIN;
  if (!configuredOrigin) return null;

  let origin;
  try {
    origin = new URL(configuredOrigin);
  } catch {
    return null;
  }
  if (
    origin.protocol !== 'http:' ||
    origin.hostname !== '127.0.0.1' ||
    origin.pathname !== '/' ||
    origin.username ||
    origin.password ||
    origin.search ||
    origin.hash
  ) {
    return null;
  }

  const port = Number(origin.port);
  if (!Number.isInteger(port) || port < 1024 || port > 65_535) {
    throw new Error(
      'Local authentication API origin must use an explicit unprivileged port.',
    );
  }
  const missing = requiredHostedValues.filter(
    (key) => !environment[key]?.trim(),
  );
  if (missing.length > 0) {
    throw new Error(
      `Local authentication API configuration is incomplete: ${missing.join(', ')}.`,
    );
  }

  return {
    origin: origin.origin,
    environment: {
      ...environment,
      TRO_API_MODE: 'hosted',
      TRO_API_BIND: `127.0.0.1:${port}`,
      TRO_ALLOW_HOSTED_MIGRATION: '1',
    },
  };
}

export async function waitForReady(origin, processDone, attempts = 50) {
  const ready = new URL('/readyz', origin);
  const exited = processDone.then(
    () => ({ exited: true }),
    () => ({ exited: true }),
  );
  for (let attempt = 0; attempt < attempts; attempt += 1) {
    try {
      const result = await Promise.race([
        fetch(ready, { signal: AbortSignal.timeout(1_000) }).then(
          (response) => ({ response }),
        ),
        exited,
      ]);
      if ('exited' in result) {
        throw new Error('Local authentication API exited during startup.');
      }
      if (result.response.ok) return;
    } catch (error) {
      if (error instanceof Error && error.message.includes('exited'))
        throw error;
    }
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
  throw new Error(`Local authentication API did not become ready at ${ready}.`);
}
