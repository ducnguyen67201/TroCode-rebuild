import { describe, expect, it } from 'vitest';
import corpus from '../../../tests/fixtures/contracts/corpus.json';
import { parseAuthStatus, parseMessage, parseStatus } from '../src/index';
describe('wire conformance', () => {
  for (const entry of corpus)
    it(entry.name, () => {
      if (entry.valid) expect(() => parseMessage(entry.value)).not.toThrow();
      else
        expect(() => parseMessage(entry.value)).toThrow(
          'Invalid runtime message',
        );
    });
  it('rejects invalid presentation state', () =>
    expect(() => parseStatus({ state: 'healthy' })).toThrow());
  it('accepts a closed authenticated projection', () =>
    expect(() =>
      parseAuthStatus({
        state: 'authenticated',
        revision: 4,
        message: 'Signed in.',
        configured: true,
        retryable: false,
        user: {
          accountId: '00000000-0000-0000-0000-000000000001',
          displayName: 'Ada Learner',
          email: 'ada@example.com',
        },
        workspaces: [
          {
            workspaceId: '00000000-0000-0000-0000-000000000002',
            name: 'Robotics Studio',
            role: 'student',
          },
        ],
        accessTokenExpiresAt: '2026-09-25T12:15:00Z',
      }),
    ).not.toThrow());
  it('rejects credentials and membership-free authenticated state', () => {
    const base = {
      state: 'authenticated',
      revision: 4,
      message: 'Signed in.',
      configured: true,
      retryable: false,
      user: {
        accountId: '00000000-0000-0000-0000-000000000001',
        displayName: 'Ada Learner',
        email: 'ada@example.com',
      },
      workspaces: [],
      accessTokenExpiresAt: '2026-09-25T12:15:00Z',
    };
    expect(() => parseAuthStatus(base)).toThrow('Invalid auth status');
    expect(() => parseAuthStatus({ ...base, refreshToken: 'secret' })).toThrow(
      'Invalid auth status',
    );
  });
});
