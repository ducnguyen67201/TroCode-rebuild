import { expect, it } from 'vitest';
import type { AuthStatus } from '@tro/contracts';
import { hasWorkspaceAccess, latestAuthStatus } from './auth-state';

const signedOut: AuthStatus = {
  state: 'signedOut',
  revision: 4,
  message: 'Signed out.',
  configured: true,
  retryable: false,
  user: null,
  workspaces: [],
  accessTokenExpiresAt: null,
};

it('ignores a stale authentication projection', () => {
  const stale = { ...signedOut, revision: 3 };
  expect(latestAuthStatus(signedOut, stale)).toBe(signedOut);
});

it('requires both authenticated state and workspace access', () => {
  expect(hasWorkspaceAccess(signedOut)).toBe(false);
  expect(
    hasWorkspaceAccess({
      ...signedOut,
      state: 'authenticated',
      user: {
        accountId: '00000000-0000-0000-0000-000000000001',
        displayName: 'Ada Learner',
        email: 'ada@example.com',
      },
      workspaces: [
        {
          workspaceId: '00000000-0000-0000-0000-000000000002',
          name: 'Northstar Robotics',
          role: 'student',
        },
      ],
      accessTokenExpiresAt: '2026-10-25T12:15:00Z',
    }),
  ).toBe(true);
});
