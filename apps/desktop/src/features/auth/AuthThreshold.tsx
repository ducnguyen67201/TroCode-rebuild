import type { AuthStatus, AuthUser } from '@tro/contracts';
import { LanguageSelect, useLanguage, type TranslationKey } from '../../i18n';

type PublicAuthState = Exclude<AuthStatus['state'], 'authenticated'>;

interface AuthThresholdProps {
  state: PublicAuthState;
  message: string;
  busy: boolean;
  preview: boolean;
  user?: AuthUser | null;
  onPrimary?: () => void;
  onSignOut?: () => void;
}

const stateContent: Record<
  PublicAuthState,
  {
    eyebrow: TranslationKey;
    title: TranslationKey;
    note: TranslationKey;
    action?: TranslationKey;
    waypoint: number;
  }
> = {
  checking: {
    eyebrow: 'auth.checking.eyebrow',
    title: 'auth.checking.title',
    note: 'auth.checking.note',
    waypoint: 0,
  },
  signedOut: {
    eyebrow: 'auth.signedOut.eyebrow',
    title: 'auth.signedOut.title',
    note: 'auth.signedOut.note',
    action: 'auth.signedOut.action',
    waypoint: 0,
  },
  signingIn: {
    eyebrow: 'auth.signingIn.eyebrow',
    title: 'auth.signingIn.title',
    note: 'auth.signingIn.note',
    waypoint: 1,
  },
  membershipRequired: {
    eyebrow: 'auth.membershipRequired.eyebrow',
    title: 'auth.membershipRequired.title',
    note: 'auth.membershipRequired.note',
    action: 'auth.membershipRequired.action',
    waypoint: 1,
  },
  offline: {
    eyebrow: 'auth.offline.eyebrow',
    title: 'auth.offline.title',
    note: 'auth.offline.note',
    action: 'auth.offline.action',
    waypoint: 1,
  },
  error: {
    eyebrow: 'auth.error.eyebrow',
    title: 'auth.error.title',
    note: 'auth.error.note',
    action: 'auth.error.action',
    waypoint: 0,
  },
};

export function AuthThreshold({
  state,
  message,
  busy,
  preview,
  user,
  onPrimary,
  onSignOut,
}: AuthThresholdProps) {
  const { t } = useLanguage();
  const content = stateContent[state];
  const isAlert = state === 'error';

  return (
    <div className={`auth-threshold auth-threshold--${state}`}>
      <header className="auth-masthead">
        <span className="auth-wordmark" aria-label="Tro">
          tro<span className="auth-wordmark-dot">.</span>
        </span>
        <div className="auth-masthead-actions">
          <LanguageSelect compact />
          <span className="auth-device-note">
            <span className="auth-device-dot" aria-hidden="true" />
            {t('auth.secureSession')}
          </span>
        </div>
      </header>

      <main className="auth-layout">
        <section className="auth-copy" aria-labelledby="auth-title">
          <p className="auth-eyebrow">{t(content.eyebrow)}</p>
          <h1 id="auth-title">
            {t(content.title)
              .split('\n')
              .map((line) => (
                <span key={line}>{line}</span>
              ))}
          </h1>
          <p className="auth-intro">{t(content.note)}</p>

          {user && state === 'membershipRequired' && (
            <div
              className="auth-identity"
              aria-label={t('auth.signedInAccount')}
            >
              <span className="auth-avatar" aria-hidden="true">
                {initials(user.displayName)}
              </span>
              <span>
                <strong>{user.displayName}</strong>
                <small>{user.email}</small>
              </span>
            </div>
          )}

          <div className="auth-announcement">
            <span className="auth-announcement-mark" aria-hidden="true">
              {isAlert ? '!' : content.waypoint + 1}
            </span>
            <p role={isAlert ? 'alert' : 'status'} aria-live="polite">
              {message}
            </p>
          </div>

          <div className="auth-actions">
            {onPrimary && (
              <button
                className="auth-primary"
                disabled={busy}
                onClick={onPrimary}
              >
                <span className="auth-action-icon" aria-hidden="true">
                  {state === 'signedOut' && <GoogleMark />}
                </span>
                <span>
                  {busy
                    ? t('common.pleaseWait')
                    : content.action
                      ? t(content.action)
                      : ''}
                </span>
                <span className="auth-action-arrow" aria-hidden="true">
                  ↗
                </span>
              </button>
            )}
            {onSignOut && (
              <button
                className="auth-text-action"
                disabled={busy}
                onClick={onSignOut}
              >
                {t('common.signOut')}
              </button>
            )}
          </div>

          {preview && <p className="auth-preview-note">{t('auth.preview')}</p>}
        </section>

        <Wayfinding state={state} waypoint={content.waypoint} />
      </main>

      <footer className="auth-footer">
        <span>{t('common.privateByDesign')}</span>
        <span aria-hidden="true">—</span>
        <span>{t('auth.ready')}</span>
      </footer>
    </div>
  );
}

function Wayfinding({
  state,
  waypoint,
}: {
  state: PublicAuthState;
  waypoint: number;
}) {
  const { t } = useLanguage();
  return (
    <aside className="auth-wayfinding" aria-label={t('auth.progress')}>
      <span className="auth-map-caption">{t('auth.pathCaption')}</span>
      <svg
        className="auth-map"
        viewBox="0 0 520 600"
        aria-hidden="true"
        focusable="false"
      >
        <path
          className="auth-contour auth-contour--one"
          d="M38 81C134 14 250 43 302 112c51 68 35 142 112 184 63 35 69 112 21 162-48 51-131 31-184 72-58 44-151 30-188-39-32-59 17-112-6-171C28 241-37 135 38 81Z"
        />
        <path
          className="auth-contour auth-contour--two"
          d="M89 124c72-48 153-21 181 38 30 65-6 122 60 162 55 34 51 98 6 131-48 36-102 9-150 43-46 32-106 1-115-50-10-56 38-88 0-147-42-65-43-136 18-177Z"
        />
        <path
          className="auth-route-shadow"
          d="M118 492c-42-64-10-114 51-139 76-30 20-98 92-121 62-20 129-44 143-122"
        />
        <path
          className="auth-route"
          pathLength="1"
          d="M118 492c-42-64-10-114 51-139 76-30 20-98 92-121 62-20 129-44 143-122"
        />
        <g className={waypoint === 0 ? 'auth-stop is-active' : 'auth-stop'}>
          <circle cx="118" cy="492" r="20" />
          <circle cx="118" cy="492" r="5" />
        </g>
        <g className={waypoint === 1 ? 'auth-stop is-active' : 'auth-stop'}>
          <circle cx="243" cy="267" r="20" />
          <circle cx="243" cy="267" r="5" />
        </g>
        <g className="auth-stop">
          <circle cx="404" cy="110" r="20" />
          <circle cx="404" cy="110" r="5" />
        </g>
        <path
          className="auth-pencil"
          d="m92 521-8 29 28-10m-20-19 20 19M380 80l26-25 15 15-25 26"
        />
      </svg>
      <ol className="auth-waypoints">
        <li className={waypoint === 0 ? 'is-current' : ''}>
          <span>01</span>
          <strong>{t('auth.identify')}</strong>
          <small>{t('auth.trustedAccount')}</small>
        </li>
        <li className={waypoint === 1 ? 'is-current' : ''}>
          <span>02</span>
          <strong>{t('auth.findStudio')}</strong>
          <small>
            {state === 'membershipRequired'
              ? t('auth.waitingAccess')
              : t('auth.sharedWorkspace')}
          </small>
        </li>
        <li>
          <span>03</span>
          <strong>{t('auth.keepLearning')}</strong>
          <small>{t('auth.returnToWork')}</small>
        </li>
      </ol>
    </aside>
  );
}

function GoogleMark() {
  return (
    <svg
      className="google-mark"
      viewBox="0 0 18 18"
      aria-hidden="true"
      focusable="false"
    >
      <path
        fill="#4285F4"
        d="M17.6 9.2c0-.6-.1-1.2-.2-1.7H9v3.4h4.8a4 4 0 0 1-1.8 2.6v2.2h2.9c1.7-1.6 2.7-3.8 2.7-6.5Z"
      />
      <path
        fill="#34A853"
        d="M9 18c2.4 0 4.5-.8 6-2.2l-2.9-2.3c-.8.6-1.9.9-3.1.9a5.3 5.3 0 0 1-5-3.7H1v2.3A9 9 0 0 0 9 18Z"
      />
      <path
        fill="#FBBC05"
        d="M4 10.7a5.4 5.4 0 0 1 0-3.4V5H1a9 9 0 0 0 0 8l3-2.3Z"
      />
      <path
        fill="#EA4335"
        d="M9 3.6c1.4 0 2.6.5 3.5 1.4L15 2.5A8.5 8.5 0 0 0 9 0a9 9 0 0 0-8 5l3 2.3a5.3 5.3 0 0 1 5-3.7Z"
      />
    </svg>
  );
}

function initials(name: string) {
  return name
    .split(/\s+/)
    .filter(Boolean)
    .slice(0, 2)
    .map((part) => part[0]?.toUpperCase())
    .join('');
}
