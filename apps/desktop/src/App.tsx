import { useState } from 'react';
import { TeachingPanel } from './features/teaching/TeachingPanel';
import type { DesktopClient } from './platform/desktop-client';
import { RuntimeStatus } from './features/runtime/RuntimeStatus';
import type { AuthenticatedContext } from './features/auth/AuthGate';
import { WorkspaceAccessPanel } from './features/workspace/WorkspaceAccessPanel';
import { AppSidebar, type AppSection } from './features/navigation/AppSidebar';
import { VoiceControlPanel } from './features/voice/VoiceControlPanel';
import { SettingsPanel } from './features/settings/SettingsPanel';

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
              <VoiceControlPanel client={client.voice} />
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
            <SettingsPanel
              client={client}
              session={session}
              workspaceName={workspace.name}
            />
          )}

          <footer>
            React presentation · Rust supervision · Python runtime
          </footer>
        </div>
      </section>
    </main>
  );
}
