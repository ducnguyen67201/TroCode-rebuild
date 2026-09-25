import { createPreviewTeaching } from './preview-teaching';
import type { AuthStatus, RuntimeStatus } from '@tro/contracts';
import type { AuthClient } from './auth-client';
import type { DesktopClient } from './desktop-client';

export type PreviewAuthScenario =
  | 'signedOut'
  | 'checking'
  | 'signingIn'
  | 'authenticated'
  | 'membershipRequired'
  | 'offline'
  | 'error';

interface PreviewClientOptions {
  authScenario?: PreviewAuthScenario;
}

const previewUser = {
  accountId: '00000000-0000-0000-0000-000000000001',
  displayName: 'Ada Learner',
  email: 'ada@example.com',
};

const previewWorkspace = {
  workspaceId: '00000000-0000-0000-0000-000000000002',
  name: 'Northstar Robotics',
  role: 'student' as const,
};

export function createPreviewClient(
  options: PreviewClientOptions = {},
): DesktopClient {
  let status: RuntimeStatus = {
    state: 'stopped',
    generationId: null,
    revision: 0,
    message: 'Ready to preview a diagnostic session.',
  };
  const listeners = new Set<(value: RuntimeStatus) => void>();
  const auth = createPreviewAuth(options.authScenario ?? 'signedOut');
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

function createPreviewAuth(initial: PreviewAuthScenario): AuthClient {
  let revision = 0;
  let status = previewAuthStatus(initial, revision);
  const listeners = new Set<(value: AuthStatus) => void>();

  function publish(state: PreviewAuthScenario) {
    revision += 1;
    status = previewAuthStatus(state, revision);
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
    workspaces: state === 'authenticated' ? [previewWorkspace] : [],
    accessTokenExpiresAt: hasIdentity ? '2026-10-25T12:15:00Z' : null,
  };
}

function previewPause() {
  return new Promise<void>((resolve) => setTimeout(resolve, 180));
}
