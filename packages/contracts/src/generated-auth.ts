/* Generated. Do not edit. */

export type AuthStatus = {
  state: 'checking' | 'signedOut' | 'signingIn' | 'authenticated' | 'membershipRequired' | 'offline' | 'error';
  revision: number;
  message: string;
  configured: boolean;
  retryable: boolean;
  user: AuthUser | null;
  /**
   * @maxItems 100
   */
  workspaces: WorkspaceSummary[];
  accessTokenExpiresAt: string | null;
};

export interface AuthUser {
  accountId: string;
  displayName: string;
  email: string;
}
export interface WorkspaceSummary {
  workspaceId: string;
  name: string;
  role: 'owner' | 'teacher' | 'student';
}
