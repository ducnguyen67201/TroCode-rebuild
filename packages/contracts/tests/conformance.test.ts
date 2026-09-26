import { describe, expect, it } from 'vitest';
import corpus from '../../../tests/fixtures/contracts/corpus.json';
import {
  parseAuthStatus,
  parseMessage,
  parseStatus,
  parseWorkspaceMember,
  parseWorkspaceMemberList,
  parseVoiceStatus,
} from '../src/index';
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
  it('accepts the bounded owner workspace member projection', () => {
    const value = {
      workspace: {
        workspaceId: '00000000-0000-0000-0000-000000000002',
        name: 'Robotics Studio',
        role: 'owner',
      },
      members: [
        {
          membershipId: '00000000-0000-0000-0000-000000000003',
          email: 'student+robotics@example.com',
          displayName: null,
          role: 'student',
          state: 'pending',
          joinedAt: null,
        },
      ],
    };
    expect(parseWorkspaceMemberList(value)).toEqual(value);
  });
  it('rejects malformed workspace lifecycle states and extra credential data', () => {
    const member = {
      membershipId: '00000000-0000-0000-0000-000000000003',
      email: 'student@example.com',
      displayName: null,
      role: 'student',
      state: 'active',
      joinedAt: null,
    };
    expect(() => parseWorkspaceMember(member)).toThrow(
      'Invalid workspace member',
    );
    expect(() =>
      parseWorkspaceMember({
        ...member,
        state: 'pending',
        refreshToken: 'must-never-cross-the-bridge',
      }),
    ).toThrow('Invalid workspace member');
  });
  it('accepts a closed voice projection and rejects private fields', () => {
    const value = {
      phase: 'idle',
      revision: 2,
      utteranceId: null,
      runId: null,
      partialTranscript: '',
      finalTranscript: '',
      queuedInstructions: [],
      targetTitle: null,
      message: 'Ready.',
      shortcut: 'Command+Control',
      permissions: {
        microphone: 'granted',
        keyboardMonitoring: 'granted',
        ready: true,
        recovery: '',
      },
      confirmation: null,
      actionsUsed: 0,
    };
    expect(parseVoiceStatus(value)).toEqual(value);
    expect(() => parseVoiceStatus({ ...value, audio: 'bytes' })).toThrow(
      'Invalid voice status',
    );
  });
});
