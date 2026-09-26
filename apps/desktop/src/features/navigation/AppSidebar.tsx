import type { WorkspaceSummary } from '@tro/contracts';
import { localeKey, useLanguage } from '../../i18n';

export type AppSection = 'learn' | 'team' | 'settings';

export function AppSidebar({
  activeSection,
  onNavigate,
  workspace,
}: {
  activeSection: AppSection;
  onNavigate: (section: AppSection) => void;
  workspace: WorkspaceSummary;
}) {
  const { t } = useLanguage();
  const isOwner = workspace.role === 'owner';
  const roleKey = localeKey('role', workspace.role);

  return (
    <aside className="app-sidebar" aria-label={t('nav.workspaceNavigation')}>
      <div className="sidebar-top">
        <div className="sidebar-wordmark" aria-label="Tro">
          tro<span>.</span>
        </div>

        <div className="sidebar-workspace" title={workspace.name}>
          <span className="workspace-mark" aria-hidden="true">
            {workspace.name.charAt(0).toUpperCase()}
          </span>
          <span className="workspace-details">
            <small>{t('nav.workspace')}</small>
            <strong>{workspace.name}</strong>
            <span>{roleKey ? t(roleKey) : workspace.role}</span>
          </span>
        </div>

        <nav className="sidebar-primary" aria-label={t('nav.learning')}>
          <SidebarButton
            active={activeSection === 'learn'}
            icon="learn"
            label={t('nav.learn')}
            onClick={() => onNavigate('learn')}
          />
        </nav>
      </div>

      <div className="sidebar-bottom">
        <nav aria-label={t('nav.workspaceTools')}>
          {isOwner && (
            <SidebarButton
              active={activeSection === 'team'}
              icon="team"
              label={t('nav.team')}
              onClick={() => onNavigate('team')}
            />
          )}
          <SidebarButton
            active={activeSection === 'settings'}
            icon="settings"
            label={t('nav.settings')}
            onClick={() => onNavigate('settings')}
          />
        </nav>
        <p className="sidebar-note">{t('common.privateByDesign')}</p>
      </div>
    </aside>
  );
}

function SidebarButton({
  active,
  icon,
  label,
  onClick,
}: {
  active: boolean;
  icon: 'learn' | 'team' | 'settings';
  label: string;
  onClick: () => void;
}) {
  return (
    <button
      aria-current={active ? 'page' : undefined}
      className={active ? 'sidebar-link is-active' : 'sidebar-link'}
      onClick={onClick}
      title={label}
      type="button"
    >
      <SidebarIcon name={icon} />
      <span className="sidebar-link-label">{label}</span>
    </button>
  );
}

function SidebarIcon({ name }: { name: 'learn' | 'team' | 'settings' }) {
  if (name === 'team') {
    return (
      <svg aria-hidden="true" viewBox="0 0 24 24">
        <path d="M16 20v-1.5a3.5 3.5 0 0 0-3.5-3.5h-5A3.5 3.5 0 0 0 4 18.5V20" />
        <circle cx="10" cy="7" r="3" />
        <path d="M17 11a3 3 0 0 1 3 3v1M17 4.2a3 3 0 0 1 0 5.6" />
      </svg>
    );
  }

  if (name === 'settings') {
    return (
      <svg aria-hidden="true" viewBox="0 0 24 24">
        <circle cx="12" cy="12" r="3" />
        <path d="M19.4 15a1.7 1.7 0 0 0 .3 1.9l.1.1-2.8 2.8-.1-.1a1.7 1.7 0 0 0-1.9-.3 1.7 1.7 0 0 0-1 1.6v.2h-4V21a1.7 1.7 0 0 0-1-1.6 1.7 1.7 0 0 0-1.9.3l-.1.1L4.2 17l.1-.1a1.7 1.7 0 0 0 .3-1.9A1.7 1.7 0 0 0 3 14H2.8v-4H3a1.7 1.7 0 0 0 1.6-1 1.7 1.7 0 0 0-.3-1.9L4.2 7 7 4.2l.1.1A1.7 1.7 0 0 0 9 4.6 1.7 1.7 0 0 0 10 3V2.8h4V3a1.7 1.7 0 0 0 1 1.6 1.7 1.7 0 0 0 1.9-.3l.1-.1L19.8 7l-.1.1a1.7 1.7 0 0 0-.3 1.9 1.7 1.7 0 0 0 1.6 1h.2v4H21a1.7 1.7 0 0 0-1.6 1Z" />
      </svg>
    );
  }

  return (
    <svg aria-hidden="true" viewBox="0 0 24 24">
      <path d="M5 4.5h8.5A2.5 2.5 0 0 1 16 7v12H7.5A2.5 2.5 0 0 1 5 16.5v-12Z" />
      <path d="M8 8h5M8 11h5M8 14h3M16 7h1.5A1.5 1.5 0 0 1 19 8.5V19h-3" />
    </svg>
  );
}
