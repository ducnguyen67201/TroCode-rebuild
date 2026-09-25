import type { AuthStatus, AuthUser, WorkspaceSummary } from '@tro/contracts';
import { useEffect, useRef, useState, type ReactNode } from 'react';
import type { AuthClient } from '../../platform/auth-client';
import { AuthThreshold } from './AuthThreshold';
import {
  checkingStatus,
  hasWorkspaceAccess,
  latestAuthStatus,
} from './auth-state';

export interface AuthenticatedContext {
  user: AuthUser;
  workspaces: WorkspaceSummary[];
  signOut: () => void;
  signingOut: boolean;
}

interface AuthGateProps {
  auth: AuthClient;
  preview: boolean;
  children: (context: AuthenticatedContext) => ReactNode;
}

const boundaryMessage =
  'Tro could not verify the secure session response. Try again in a moment.';

export function AuthGate({ auth, preview, children }: AuthGateProps) {
  const [status, setStatus] = useState<AuthStatus>(checkingStatus);
  const [busy, setBusy] = useState(false);
  const [boundaryFailed, setBoundaryFailed] = useState(false);
  const busyRef = useRef(false);
  const actionEpoch = useRef(0);

  useEffect(() => {
    let active = true;
    let unsubscribe: (() => void) | undefined;

    const receive = (incoming: AuthStatus) => {
      if (!active) return;
      setBoundaryFailed(false);
      setStatus((current) => latestAuthStatus(current, incoming));
    };
    const failClosed = () => {
      if (active) setBoundaryFailed(true);
    };

    void auth
      .subscribe(receive, failClosed)
      .then(async (cleanup) => {
        if (!active) {
          cleanup();
          return;
        }
        unsubscribe = cleanup;
        receive(await auth.status());
      })
      .catch(failClosed);

    return () => {
      active = false;
      actionEpoch.current += 1;
      busyRef.current = false;
      unsubscribe?.();
    };
  }, [auth]);

  async function perform(action: () => Promise<AuthStatus>) {
    if (busyRef.current) return;
    busyRef.current = true;
    const ticket = ++actionEpoch.current;
    setBusy(true);
    setBoundaryFailed(false);
    try {
      const incoming = await action();
      if (ticket === actionEpoch.current) {
        setBoundaryFailed(false);
        setStatus((current) => latestAuthStatus(current, incoming));
      }
    } catch {
      if (ticket === actionEpoch.current) setBoundaryFailed(true);
    } finally {
      if (ticket === actionEpoch.current) {
        busyRef.current = false;
        setBusy(false);
      }
    }
  }

  if (boundaryFailed) {
    return (
      <AuthThreshold
        busy={busy}
        message={boundaryMessage}
        preview={preview}
        state="error"
        onPrimary={() => void perform(() => auth.retry())}
      />
    );
  }

  if (hasWorkspaceAccess(status) && status.user) {
    return children({
      user: status.user,
      workspaces: status.workspaces,
      signOut: () => void perform(() => auth.signOut()),
      signingOut: busy,
    });
  }

  const publicState =
    !status.configured || status.state === 'authenticated'
      ? 'error'
      : status.state;
  const publicMessage = !status.configured
    ? 'Google sign-in is not configured for this build.'
    : status.state === 'authenticated'
      ? boundaryMessage
      : status.message;

  return (
    <AuthThreshold
      busy={busy}
      message={publicMessage}
      preview={preview}
      state={publicState}
      user={status.user}
      onPrimary={
        status.state === 'signedOut' && status.configured
          ? () => void perform(() => auth.signIn())
          : status.retryable || status.state === 'membershipRequired'
            ? () => void perform(() => auth.retry())
            : undefined
      }
      onSignOut={
        status.user ? () => void perform(() => auth.signOut()) : undefined
      }
    />
  );
}
