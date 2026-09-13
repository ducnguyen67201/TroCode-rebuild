import type { RuntimeStatus } from '@tro/contracts';
export function latestStatus(
  current: RuntimeStatus,
  incoming: RuntimeStatus,
): RuntimeStatus {
  return incoming.revision >= current.revision ? incoming : current;
}
export function publicError(error: unknown): string {
  if (typeof error === 'object' && error !== null && 'code' in error) {
    switch (error.code) {
      case 'UNAUTHORIZED':
        return 'The development profile could not be authenticated.';
      case 'NOT_READY':
        return 'Runtime unavailable. Check setup and the local API.';
      case 'TIMEOUT':
        return 'The runtime timed out. Stop it, then retry.';
      case 'PROTOCOL_MISMATCH':
        return 'Runtime versions differ. Run setup again.';
    }
  }
  return 'The request failed. Check the local runtime and retry.';
}
