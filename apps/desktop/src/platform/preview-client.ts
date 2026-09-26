import { createPreviewTeaching } from './preview-teaching';
import type {
  AuthStatus,
  DeviceReadiness,
  RuntimeStatus,
  WorkspaceMember,
} from '@tro/contracts';
import type { AuthClient } from './auth-client';
import type { DesktopClient } from './desktop-client';
import { createPreviewVoice } from './preview-voice';

export type PreviewAuthScenario =
  | 'signedOut'
  | 'checking'
  | 'signingIn'
  | 'authenticated'
  | 'membershipRequired'
  | 'offline'
  | 'error';

export type PreviewPermissionScenario =
  | 'fresh'
  | 'ready'
  | 'screenDenied'
  | 'relaunchRequired'
  | 'microphoneUnavailable'
  | 'windowsMicrophoneDenied';

interface PreviewClientOptions {
  authScenario?: PreviewAuthScenario;
  workspaceRole?: PreviewWorkspaceRole;
  permissionScenario?: PreviewPermissionScenario;
}

export type PreviewWorkspaceRole = 'owner' | 'teacher' | 'student';

const previewUser = {
  accountId: '00000000-0000-0000-0000-000000000001',
  displayName: 'Ada Learner',
  email: 'ada@example.com',
};

export function createPreviewClient(
  options: PreviewClientOptions = {},
): DesktopClient {
  const workspace = {
    workspaceId: '00000000-0000-0000-0000-000000000002',
    name: 'Northstar Robotics',
    role: options.workspaceRole ?? ('student' as const),
  };
  let members: WorkspaceMember[] = [
    {
      membershipId: '00000000-0000-0000-0000-000000000003',
      email: 'owner@example.com',
      displayName: 'Northstar Owner',
      role: 'owner',
      state: 'active',
      joinedAt: '2026-09-25T12:00:00Z',
    },
  ];
  let status: RuntimeStatus = {
    state: 'stopped',
    generationId: null,
    revision: 0,
    message: 'Ready to preview a diagnostic session.',
  };
  const listeners = new Set<(value: RuntimeStatus) => void>();
  let readiness = previewReadiness(options.permissionScenario ?? 'ready');
  let pendingSettingsTarget:
    'screenCapture' | 'accessibility' | 'microphone' | null = null;
  const auth = createPreviewAuth(
    options.authScenario ?? 'signedOut',
    workspace,
  );
  async function change(state: RuntimeStatus['state']) {
    status = {
      state,
      revision: status.revision + 1,
      generationId: state === 'running' ? 'preview' : null,
      message:
        state === 'running'
          ? 'Simulated runtime is running.'
          : 'Simulated runtime stopped.',
    };
    for (const listener of listeners) listener(status);
    return status;
  }
  return {
    preview: true,
    auth,
    workspace: {
      members: async () => ({
        workspace: { ...workspace, role: 'owner' },
        members,
      }),
      addMember: async (_workspaceId, email, role) => {
        const existing = members.find(
          (member) => member.email.toLowerCase() === email.trim().toLowerCase(),
        );
        if (existing) return existing;
        const member: WorkspaceMember = {
          membershipId: `00000000-0000-0000-0000-${String(members.length + 3).padStart(12, '0')}`,
          email: email.trim(),
          displayName: null,
          role,
          state: 'pending',
          joinedAt: null,
        };
        members = [...members, member];
        return member;
      },
      removeMember: async (_workspaceId, membershipId) => {
        members = members.filter(
          (member) => member.membershipId !== membershipId,
        );
      },
    },
    voice: createPreviewVoice(),
    device: {
      check: async () => {
        if (pendingSettingsTarget) {
          const target = pendingSettingsTarget;
          pendingSettingsTarget = null;
          if (target === 'screenCapture') {
            readiness = {
              ...readiness,
              screenCapture: readyCapability('Screen Recording is ready.'),
            };
          } else if (target === 'accessibility') {
            readiness = {
              ...readiness,
              accessibility: readyCapability('Accessibility is ready.'),
            };
          } else {
            readiness = {
              ...readiness,
              microphone: readyCapability(
                'Microphone is ready. This preview retained no audio.',
                false,
              ),
            };
          }
        }
        return readiness;
      },
      request: async (kind) => {
        await previewPause();
        if (
          kind === 'microphone' &&
          options.permissionScenario !== 'microphoneUnavailable'
        ) {
          readiness = {
            ...readiness,
            microphone: readyCapability(
              'Microphone is ready. This preview retained no audio.',
              false,
            ),
          };
        }
        return readiness;
      },
      openSettings: async (target) => {
        await previewPause();
        pendingSettingsTarget = target;
      },
      relaunch: async () => {
        readiness = { ...readiness, requiresRelaunch: false };
      },
    },
    teaching: createPreviewTeaching(),
    status: async () => status,
    start: () => change('running'),
    stop: () => change('stopped'),
    health: async () => status,
    restart: () => change('running'),
    selectAccount: () => change('stopped'),
    subscribe: async (listener) => {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
  };
}

function previewReadiness(
  scenario: PreviewPermissionScenario,
): DeviceReadiness {
  if (scenario === 'windowsMicrophoneDenied') {
    return {
      platform: 'windows',
      requiresRelaunch: false,
      message: 'Windows capture support is ready for preview.',
      screenCapture: {
        status: 'available',
        required: true,
        canRequest: false,
        recovery: 'none',
        message: 'The Windows picker will confirm the selected window.',
      },
      accessibility: {
        status: 'available',
        required: false,
        canRequest: false,
        recovery: 'none',
        message: 'Windows does not require a separate Accessibility grant.',
      },
      microphone: {
        status: 'denied',
        required: false,
        canRequest: false,
        recovery: 'manualSettings',
        message: 'Desktop app microphone access is off.',
      },
    };
  }
  const ready = scenario === 'ready' || scenario === 'microphoneUnavailable';
  const denied = scenario === 'screenDenied';
  const relaunch = scenario === 'relaunchRequired';
  const screenReady = ready || relaunch;
  return {
    platform: 'macos',
    requiresRelaunch: relaunch,
    message: 'Preview device readiness is simulated.',
    screenCapture: {
      status: screenReady ? 'granted' : denied ? 'denied' : 'unknown',
      required: true,
      canRequest: !screenReady && !denied,
      recovery: relaunch
        ? 'relaunch'
        : denied
          ? 'manualSettings'
          : screenReady
            ? 'none'
            : 'request',
      message: relaunch
        ? 'Screen Recording changed. Relaunch Tro before observing.'
        : denied
          ? 'Allow Tro in System Settings → Privacy & Security → Screen Recording.'
          : screenReady
            ? 'Screen Recording is ready.'
            : 'Screen Recording has not been confirmed.',
    },
    accessibility: {
      status: screenReady ? 'granted' : denied ? 'denied' : 'unknown',
      required: true,
      canRequest: !screenReady && !denied,
      recovery: denied ? 'manualSettings' : screenReady ? 'none' : 'request',
      message: screenReady
        ? 'Accessibility is ready.'
        : 'Allow Tro in System Settings → Privacy & Security → Accessibility.',
    },
    microphone: {
      status:
        scenario === 'microphoneUnavailable'
          ? 'unavailable'
          : ready
            ? 'granted'
            : 'notDetermined',
      required: false,
      canRequest: scenario !== 'microphoneUnavailable',
      recovery:
        scenario === 'microphoneUnavailable'
          ? 'manualSettings'
          : ready
            ? 'none'
            : 'request',
      message:
        scenario === 'microphoneUnavailable'
          ? 'No microphone is available. Text remains available.'
          : ready
            ? 'Microphone is ready.'
            : 'Microphone setup is optional.',
    },
  };
}

function readyCapability(message: string, required = true) {
  return {
    status: 'granted' as const,
    required,
    canRequest: false,
    recovery: 'none' as const,
    message,
  };
}

function createPreviewAuth(
  initial: PreviewAuthScenario,
  workspace: {
    workspaceId: string;
    name: string;
    role: PreviewWorkspaceRole;
  },
): AuthClient {
  let revision = 0;
  let status = previewAuthStatus(initial, revision, workspace);
  const listeners = new Set<(value: AuthStatus) => void>();

  function publish(state: PreviewAuthScenario) {
    revision += 1;
    status = previewAuthStatus(state, revision, workspace);
    for (const listener of listeners) listener(status);
    return status;
  }

  return {
    status: async () => status,
    signIn: async () => {
      publish('signingIn');
      await previewPause();
      return initial === 'membershipRequired'
        ? publish('membershipRequired')
        : publish('authenticated');
    },
    retry: async () => {
      await previewPause();
      return status.user || initial === 'authenticated'
        ? publish('authenticated')
        : publish('signedOut');
    },
    signOut: async () => publish('signedOut'),
    subscribe: async (listener) => {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
  };
}

function previewAuthStatus(
  state: PreviewAuthScenario,
  revision: number,
  workspace: {
    workspaceId: string;
    name: string;
    role: PreviewWorkspaceRole;
  },
): AuthStatus {
  const hasIdentity =
    state === 'authenticated' || state === 'membershipRequired';
  const details: Record<
    PreviewAuthScenario,
    Pick<AuthStatus, 'message' | 'configured' | 'retryable'>
  > = {
    checking: {
      message: 'Checking this device for a secure session…',
      configured: true,
      retryable: false,
    },
    signedOut: {
      message: 'Sign in to continue to your Tro workspace.',
      configured: true,
      retryable: false,
    },
    signingIn: {
      message: 'Waiting for Google sign-in in your browser…',
      configured: true,
      retryable: false,
    },
    authenticated: {
      message: 'Signed in securely.',
      configured: true,
      retryable: false,
    },
    membershipRequired: {
      message: 'This account is not in an active workspace yet.',
      configured: true,
      retryable: true,
    },
    offline: {
      message: 'Tro cannot reach the sign-in service right now.',
      configured: true,
      retryable: true,
    },
    error: {
      message: 'Google sign-in is not configured for this build.',
      configured: false,
      retryable: false,
    },
  };

  return {
    state,
    revision,
    ...details[state],
    user: hasIdentity ? previewUser : null,
    workspaces: state === 'authenticated' ? [workspace] : [],
    accessTokenExpiresAt: hasIdentity ? '2026-10-25T12:15:00Z' : null,
  };
}

function previewPause() {
  return new Promise<void>((resolve) => setTimeout(resolve, 180));
}
