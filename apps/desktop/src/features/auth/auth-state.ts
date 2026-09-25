import type { AuthStatus } from '@tro/contracts';

export const checkingStatus: AuthStatus = {
  state: 'checking',
  revision: 0,
  message: 'Checking this device for a secure session…',
  configured: true,
  retryable: false,
  user: null,
  workspaces: [],
  accessTokenExpiresAt: null,
};

export function latestAuthStatus(
  current: AuthStatus,
  incoming: AuthStatus,
): AuthStatus {
  return incoming.revision >= current.revision ? incoming : current;
}

export function hasWorkspaceAccess(status: AuthStatus): boolean {
  return status.state === 'authenticated' && status.workspaces.length > 0;
}
