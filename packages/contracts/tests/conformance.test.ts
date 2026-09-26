import { describe, expect, it } from 'vitest';
import corpus from '../../../tests/fixtures/contracts/corpus.json';
import {
  parseAuthStatus,
  parseDeviceReadiness,
  parseMessage,
  parsePermissionSettingsGuide,
  parseStatus,
  parseWorkspaceMember,
  parseWorkspaceMemberList,
  parseVoiceStatus,
} from '../src/index';
const readiness = {
  platform: 'macos',
  requiresRelaunch: false,
  message: 'Device permissions checked.',
  screenCapture: {
    status: 'granted',
    required: true,
    canRequest: false,
    recovery: 'none',
    message: 'Screen Recording is ready.',
  },
  accessibility: {
    status: 'granted',
    required: true,
    canRequest: false,
    recovery: 'none',
    message: 'Accessibility is ready.',
  },
  microphone: {
    status: 'notDetermined',
    required: false,
    canRequest: true,
    recovery: 'request',
    message: 'Microphone setup is optional.',
  },
};
describe('wire conformance', () => {
  it('accepts a closed device readiness projection', () =>
    expect(parseDeviceReadiness(readiness)).toEqual(readiness));
  it('rejects native extras and unknown readiness states', () => {
    expect(() =>
      parseDeviceReadiness({ ...readiness, rawError: 'private native data' }),
    ).toThrow('Invalid device readiness');
    expect(() =>
      parseDeviceReadiness({
        ...readiness,
        microphone: { ...readiness.microphone, status: 'prompting' },
      }),
    ).toThrow('Invalid device readiness');
  });
  it('accepts only the closed permission settings guide projection', () => {
    expect(
      parsePermissionSettingsGuide({
        platform: 'macos',
        target: 'screenCapture',
      }),
    ).toEqual({ platform: 'macos', target: 'screenCapture' });
    expect(() =>
      parsePermissionSettingsGuide({
        platform: 'windows',
        target: 'camera',
      }),
    ).toThrow('Invalid permission settings guide');
    expect(() =>
      parsePermissionSettingsGuide({
        platform: 'macos',
        target: 'accessibility',
        settingsUrl: 'private://native-detail',
      }),
    ).toThrow('Invalid permission settings guide');
  });
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
