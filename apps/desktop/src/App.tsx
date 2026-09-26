import { useState } from 'react';
import { TeachingPanel } from './features/teaching/TeachingPanel';
import type { DesktopClient } from './platform/desktop-client';
import { RuntimeStatus } from './features/runtime/RuntimeStatus';
import type { AuthenticatedContext } from './features/auth/AuthGate';
import { WorkspaceAccessPanel } from './features/workspace/WorkspaceAccessPanel';
import { AppSidebar, type AppSection } from './features/navigation/AppSidebar';
import { VoiceControlPanel } from './features/voice/VoiceControlPanel';
import { SettingsPanel } from './features/settings/SettingsPanel';
import { useLanguage } from './i18n';

export function App({
  client,
  session,
}: {
  client: DesktopClient;
  session: AuthenticatedContext;
}) {
  const { t } = useLanguage();
  const workspace = session.workspaces[0]!;
  const [section, setSection] = useState<AppSection>('learn');
  const pageTitle =
    section === 'team'
      ? t('nav.team')
      : section === 'settings'
        ? t('nav.settings')
        : t('nav.learn');

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
                <p className="eyebrow">{t('app.learn.eyebrow')}</p>
                <MultilineText as="h1" text={t('app.learn.title')} />
                <MultilineText as="p" text={t('app.learn.description')} />
              </div>
              {client.preview && (
                <p className="preview">{t('common.previewRuntime')}</p>
              )}
              <RuntimeStatus client={client} />
              <VoiceControlPanel client={client.voice} />
              <TeachingPanel client={client} />
            </>
          )}

          {section === 'team' && workspace.role === 'owner' && (
            <>
              <div className="section-intro">
                <p className="eyebrow">{t('app.team.eyebrow')}</p>
                <h1>{t('app.team.title')}</h1>
                <p>{t('app.team.description')}</p>
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

          <footer>{t('common.presentationStack')}</footer>
        </div>
      </section>
    </main>
  );
}

function MultilineText({
  as: Element,
  text,
}: {
  as: 'h1' | 'p';
  text: string;
}) {
  return (
    <Element>
      {text.split('\n').map((line, index) => (
        <span key={`${index}-${line}`}>
          {index > 0 && <br />}
          {line}
        </span>
      ))}
    </Element>
  );
}
