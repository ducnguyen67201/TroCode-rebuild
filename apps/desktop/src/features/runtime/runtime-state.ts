import type { RuntimeStatus } from '@tro/contracts';
import type { TranslationKey } from '../../i18n';
export function latestStatus(
  current: RuntimeStatus,
  incoming: RuntimeStatus,
): RuntimeStatus {
  return incoming.revision >= current.revision ? incoming : current;
}
export function publicErrorKey(error: unknown): TranslationKey {
  if (typeof error === 'object' && error !== null && 'code' in error) {
    switch (error.code) {
      case 'UNAUTHORIZED':
        return 'runtime.unauthorized';
      case 'NOT_READY':
        return 'runtime.notReady';
      case 'TIMEOUT':
        return 'runtime.timeout';
      case 'PROTOCOL_MISMATCH':
        return 'runtime.protocolMismatch';
    }
  }
  return 'runtime.requestError';
}
