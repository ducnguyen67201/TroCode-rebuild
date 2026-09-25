import { useState } from 'react';
import { TeachingPanel } from './features/teaching/TeachingPanel';
import type { DesktopClient } from './platform/desktop-client';
import { RuntimeStatus } from './features/runtime/RuntimeStatus';
import type { AuthenticatedContext } from './features/auth/AuthGate';
import { WorkspaceAccessPanel } from './features/workspace/WorkspaceAccessPanel';
import { AppSidebar, type AppSection } from './features/navigation/AppSidebar';

export function App({
  client,
  session,
}: {
  client: DesktopClient;
  session: AuthenticatedContext;
}) {
  const workspace = session.workspaces[0]!;
  const [section, setSection] = useState<AppSection>('learn');
  const pageTitle =
    section === 'team' ? 'Team' : section === 'settings' ? 'Settings' : 'Learn';

  return (
    <main className="app-shell">
      <AppSidebar
        activeSection={section}
        onNavigate={setSection}
        workspace={workspace}
      />

      <section className="app-content">
        <header className="app-header">
          <div>
            <p className="app-header-kicker">{workspace.name}</p>
            <strong className="app-page-title">{pageTitle}</strong>
          </div>
          <div className="account-context">
            <span className="account-avatar" aria-hidden="true">
              {session.user.displayName.charAt(0).toUpperCase()}
            </span>
            <span className="account-name">{session.user.displayName}</span>
          </div>
        </header>

        <div className="app-page">
          {section === 'learn' && (
            <>
              <div className="intro">
                <p className="eyebrow">A solid place to begin</p>
                <h1>
                  Make room
                  <br />
                  for learning.
                </h1>
                <p>
                  One desktop. One private runtime.
                  <br />A clear foundation for what comes next.
                </p>
              </div>
              {client.preview && (
                <p className="preview">Preview — simulated runtime</p>
              )}
              <RuntimeStatus client={client} />
              <TeachingPanel client={client} />
            </>
          )}

          {section === 'team' && workspace.role === 'owner' && (
            <>
              <div className="section-intro">
                <p className="eyebrow">Workspace access</p>
                <h1>Your workspace, your people.</h1>
                <p>
                  Keep the roster intentional. Access begins only after Google
                  verifies the exact email you add here.
                </p>
              </div>
              <WorkspaceAccessPanel
                client={client.workspace}
                workspace={workspace}
              />
            </>
          )}

          {section === 'settings' && (
            <SettingsPanel session={session} workspaceName={workspace.name} />
          )}

          <footer>
            React presentation · Rust supervision · Python runtime
          </footer>
        </div>
      </section>
    </main>
  );
}

function SettingsPanel({
  session,
  workspaceName,
}: {
  session: AuthenticatedContext;
  workspaceName: string;
}) {
  const workspace = session.workspaces[0]!;
  return (
    <section className="settings-panel" aria-labelledby="settings-heading">
      <div className="section-intro">
        <p className="eyebrow">Account & access</p>
        <h1 id="settings-heading">Settings</h1>
        <p>Your profile and secure device session for this workspace.</p>
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
