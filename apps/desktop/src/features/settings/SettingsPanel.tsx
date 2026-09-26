import type { AuthenticatedContext } from '../auth/AuthGate';
import type { DesktopClient } from '../../platform/desktop-client';
import { PermissionCheckup } from '../onboarding/PermissionOnboarding';
import { VoiceLanguageSetting } from './VoiceLanguageSetting';

export function SettingsPanel({
  client,
  session,
  workspaceName,
}: {
  client: DesktopClient;
  session: AuthenticatedContext;
  workspaceName: string;
}) {
  const workspace = session.workspaces[0]!;
  return (
    <section className="settings-panel" aria-labelledby="settings-heading">
      <div className="section-intro">
        <p className="eyebrow">Account, voice &amp; access</p>
        <h1 id="settings-heading">Settings</h1>
        <p>
          Your profile, voice preference and secure device session for this
          workspace.
        </p>
      </div>

      <div className="settings-grid">
        <article>
          <p className="settings-label">Profile</p>
          <strong>{session.user.displayName}</strong>
          <span>{session.user.email}</span>
        </article>
        <article>
          <p className="settings-label">Workspace</p>
          <strong>{workspaceName}</strong>
          <span className="settings-role">{workspace.role}</span>
        </article>
      </div>

      <VoiceLanguageSetting client={client.voice} />

      <div className="device-permissions-card">
        <PermissionCheckup client={client.device} compact />
      </div>

      <div className="device-session">
        <div>
          <strong>Secure device session</strong>
          <p>
            Your sign-in stays on this device and is checked against current
            workspace access.
          </p>
        </div>
        <button
          className="sign-out-button"
          disabled={session.signingOut}
          onClick={session.signOut}
          type="button"
        >
          {session.signingOut ? 'Signing out…' : 'Sign out'}
        </button>
      </div>
    </section>
  );
}
