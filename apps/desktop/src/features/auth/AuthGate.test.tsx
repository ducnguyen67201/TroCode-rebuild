import React from 'react';
import { afterEach, expect, it, vi } from 'vitest';
import {
  act,
  cleanup,
  fireEvent,
  render,
  screen,
  waitFor,
} from '@testing-library/react';
import type { AuthStatus } from '@tro/contracts';
import type { AuthClient } from '../../platform/auth-client';
import { createPreviewClient } from '../../platform/preview-client';
import { AuthGate } from './AuthGate';
import { LanguageProvider } from '../../i18n';

afterEach(() => {
  cleanup();
  localStorage.clear();
});

it('keeps every protected control out of the DOM until workspace access exists', async () => {
  const client = createPreviewClient();
  render(
    <AuthGate auth={client.auth} preview>
      {() => <button>Protected teaching control</button>}
    </AuthGate>,
  );

  await screen.findByRole('button', { name: 'Continue with Google' });
  expect(screen.queryByText('Protected teaching control')).toBeNull();

  fireEvent.click(screen.getByRole('button', { name: 'Continue with Google' }));
  await screen.findByText('Protected teaching control');
});

it('shows the verified email while waiting and enters after access is added', async () => {
  const client = createPreviewClient({ authScenario: 'membershipRequired' });
  render(
    <AuthGate auth={client.auth} preview>
      {({ user, workspaces }) => (
        <p>
          {user.displayName} · {workspaces[0]?.name}
        </p>
      )}
    </AuthGate>,
  );

  expect(await screen.findByText('ada@example.com')).toBeTruthy();
  fireEvent.click(screen.getByRole('button', { name: 'Check for access' }));
  await screen.findByText('Ada Learner · Northstar Robotics');
});

it('prevents a duplicate sign-in while the browser handoff is pending', async () => {
  let finishSignIn: ((status: AuthStatus) => void) | undefined;
  const signIn = vi.fn(
    () =>
      new Promise<AuthStatus>((resolve) => {
        finishSignIn = resolve;
      }),
  );
  const auth = clientWith({ signIn });

  render(
    <AuthGate auth={auth} preview={false}>
      {() => <p>Inside</p>}
    </AuthGate>,
  );
  const button = await screen.findByRole('button', {
    name: 'Continue with Google',
  });
  fireEvent.click(button);
  fireEvent.click(button);

  expect(signIn).toHaveBeenCalledTimes(1);
  expect(
    (
      screen.getByRole('button', {
        name: 'Please wait…',
      }) as HTMLButtonElement
    ).disabled,
  ).toBe(true);
  finishSignIn?.(authenticatedStatus(2));
  await screen.findByText('Inside');
});

it('starts a fresh Google sign-in after a non-retryable exchange failure', async () => {
  const signIn = vi.fn(async () => authenticatedStatus(2));
  const retry = vi.fn(async () => signedOutStatus(2));
  const auth = clientWith({
    status: async () => ({
      ...signedOutStatus(1),
      state: 'error',
      message: 'Google could not verify this sign-in attempt.',
    }),
    signIn,
    retry,
  });

  render(
    <AuthGate auth={auth} preview={false}>
      {() => <p>Inside</p>}
    </AuthGate>,
  );

  fireEvent.click(await screen.findByRole('button', { name: 'Try again' }));
  await screen.findByText('Inside');
  expect(signIn).toHaveBeenCalledOnce();
  expect(retry).not.toHaveBeenCalled();
});

it('fails closed when the native event boundary reports invalid data', async () => {
  let failBoundary: (() => void) | undefined;
  const auth = clientWith({
    status: async () => authenticatedStatus(1),
    subscribe: async (_listener, onBoundaryError) => {
      failBoundary = onBoundaryError;
      return () => undefined;
    },
  });

  render(
    <AuthGate auth={auth} preview={false}>
      {() => <button>Private lesson</button>}
    </AuthGate>,
  );
  await screen.findByText('Private lesson');
  failBoundary?.();

  await waitFor(() => expect(screen.queryByText('Private lesson')).toBeNull());
  expect(screen.getByRole('alert').textContent).toContain(
    'could not verify the secure session response',
  );
});

it('removes protected UI on a signed-out event and cleans up its subscription', async () => {
  let receive: ((status: AuthStatus) => void) | undefined;
  const unsubscribe = vi.fn();
  const auth = clientWith({
    status: async () => authenticatedStatus(1),
    subscribe: async (listener) => {
      receive = listener;
      return unsubscribe;
    },
  });
  const rendered = render(
    <AuthGate auth={auth} preview={false}>
      {() => <button>Private lesson</button>}
    </AuthGate>,
  );
  await screen.findByText('Private lesson');

  act(() => receive?.(signedOutStatus(2)));
  await screen.findByRole('button', { name: 'Continue with Google' });
  expect(screen.queryByText('Private lesson')).toBeNull();

  rendered.unmount();
  expect(unsubscribe).toHaveBeenCalledOnce();
});

it('allows choosing Vietnamese before signing in', async () => {
  const client = createPreviewClient();
  render(
    <LanguageProvider initialLocale="en">
      <AuthGate auth={client.auth} preview>
        {() => <p>Protected</p>}
      </AuthGate>
    </LanguageProvider>,
  );

  await screen.findByRole('button', { name: 'Continue with Google' });
  fireEvent.change(screen.getByLabelText('App language'), {
    target: { value: 'vi' },
  });

  expect(
    screen.getByRole('button', { name: 'Tiếp tục với Google' }),
  ).toBeTruthy();
  expect(screen.getByText('Không gian học tập của bạn')).toBeTruthy();
});

function clientWith(overrides: Partial<AuthClient>): AuthClient {
  const signedOut = signedOutStatus(1);
  return {
    status: async () => signedOut,
    signIn: async () => authenticatedStatus(2),
    retry: async () => signedOut,
    signOut: async () => signedOut,
    subscribe: async () => () => undefined,
    ...overrides,
  };
}

function signedOutStatus(revision: number): AuthStatus {
  return {
    state: 'signedOut',
    revision,
    message: 'Sign in to continue to your Tro workspace.',
    configured: true,
    retryable: false,
    user: null,
    workspaces: [],
    accessTokenExpiresAt: null,
  };
}

function authenticatedStatus(revision: number): AuthStatus {
  return {
    state: 'authenticated',
    revision,
    message: 'Signed in securely.',
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
        name: 'Northstar Robotics',
        role: 'student',
      },
    ],
    accessTokenExpiresAt: '2026-10-25T12:15:00Z',
  };
}
