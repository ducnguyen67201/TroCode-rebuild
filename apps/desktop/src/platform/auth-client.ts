import type { AuthStatus } from '@tro/contracts';

export type AuthStatusListener = (status: AuthStatus) => void;
export type AuthBoundaryErrorListener = () => void;

export interface AuthClient {
  status(): Promise<AuthStatus>;
  signIn(): Promise<AuthStatus>;
  retry(): Promise<AuthStatus>;
  signOut(): Promise<AuthStatus>;
  subscribe(
    listener: AuthStatusListener,
    onBoundaryError?: AuthBoundaryErrorListener,
  ): Promise<() => void>;
}
