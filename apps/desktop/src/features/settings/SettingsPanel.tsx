import type { AuthenticatedContext } from '../auth/AuthGate';
import type { DesktopClient } from '../../platform/desktop-client';
import { PermissionCheckup } from '../onboarding/PermissionOnboarding';
import { LanguageSelect, localeKey, useLanguage } from '../../i18n';

export function SettingsPanel({
  client,
  session,
  workspaceName,
}: {
  client: DesktopClient;
  session: AuthenticatedContext;
  workspaceName: string;
}) {
  const { t } = useLanguage();
  const workspace = session.workspaces[0]!;
  const roleKey = localeKey('role', workspace.role);
  return (
    <section className="settings-panel" aria-labelledby="settings-heading">
      <div className="section-intro">
        <p className="eyebrow">{t('settings.eyebrow')}</p>
        <h1 id="settings-heading">{t('settings.title')}</h1>
        <p>{t('settings.description')}</p>
      </div>

      <div className="settings-grid">
        <article>
          <p className="settings-label">{t('settings.profile')}</p>
          <strong>{session.user.displayName}</strong>
          <span>{session.user.email}</span>
        </article>
        <article>
          <p className="settings-label">{t('settings.workspace')}</p>
          <strong>{workspaceName}</strong>
          <span className="settings-role">
            {roleKey ? t(roleKey) : workspace.role}
          </span>
        </article>
      </div>

      <div className="language-setting">
        <div>
          <strong>{t('language.label')}</strong>
          <p>{t('language.description')}</p>
        </div>
        <LanguageSelect compact />
      </div>

      <div className="device-permissions-card">
        <PermissionCheckup client={client.device} compact />
      </div>

      <div className="device-session">
        <div>
          <strong>{t('settings.secureSession')}</strong>
          <p>{t('settings.secureSessionDescription')}</p>
        </div>
        <button
          className="sign-out-button"
          disabled={session.signingOut}
          onClick={session.signOut}
          type="button"
        >
          {session.signingOut ? t('common.signingOut') : t('common.signOut')}
        </button>
      </div>
    </section>
  );
}
