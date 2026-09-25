import type { AuthStatus, AuthUser } from '@tro/contracts';

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
    eyebrow: string;
    title: string;
    note: string;
    action?: string;
    waypoint: number;
  }
> = {
  checking: {
    eyebrow: 'Finding your place',
    title: 'Picking up\nwhere you left off.',
    note: 'Tro is checking the secure session kept by this device.',
    waypoint: 0,
  },
  signedOut: {
    eyebrow: 'Your learning space',
    title: 'Begin with\nwho you are.',
    note: 'Use the Google account your workspace owner added to Tro.',
    action: 'Continue with Google',
    waypoint: 0,
  },
  signingIn: {
    eyebrow: 'One small detour',
    title: 'Your browser has\nthe next step.',
    note: 'Finish signing in there. This window will continue on its own.',
    waypoint: 1,
  },
  membershipRequired: {
    eyebrow: 'Account confirmed',
    title: 'Your place is\nalmost ready.',
    note: 'Ask a workspace owner to add this exact Google email, then try again.',
    action: 'Check for access',
    waypoint: 1,
  },
  offline: {
    eyebrow: 'Path interrupted',
    title: 'We lost the\nconnection.',
    note: 'Your secure device session is still here. Reconnect and try again.',
    action: 'Try again',
    waypoint: 1,
  },
  error: {
    eyebrow: 'Sign-in needs attention',
    title: 'This path is not\nready just yet.',
    note: 'Nothing private was shared. Ask your Tro administrator if this continues.',
    action: 'Try again',
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
  const content = stateContent[state];
  const isAlert = state === 'error';

  return (
    <div className={`auth-threshold auth-threshold--${state}`}>
      <header className="auth-masthead">
        <span className="auth-wordmark" aria-label="Tro">
          tro<span className="auth-wordmark-dot">.</span>
        </span>
        <span className="auth-device-note">
          <span className="auth-device-dot" aria-hidden="true" />
          Secure device session
        </span>
      </header>

      <main className="auth-layout">
        <section className="auth-copy" aria-labelledby="auth-title">
          <p className="auth-eyebrow">{content.eyebrow}</p>
          <h1 id="auth-title">
            {content.title.split('\n').map((line) => (
              <span key={line}>{line}</span>
            ))}
          </h1>
          <p className="auth-intro">{content.note}</p>

          {user && state === 'membershipRequired' && (
            <div
              className="auth-identity"
              aria-label="Signed-in Google account"
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
                <span>{busy ? 'Please wait…' : content.action}</span>
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
                Sign out
              </button>
            )}
          </div>

          {preview && (
            <p className="auth-preview-note">Preview · simulated sign-in</p>
          )}
        </section>

        <Wayfinding state={state} waypoint={content.waypoint} />
      </main>

      <footer className="auth-footer">
        <span>Private by design</span>
        <span aria-hidden="true">—</span>
        <span>Ready when you are</span>
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
  return (
    <aside className="auth-wayfinding" aria-label="Sign-in progress">
      <span className="auth-map-caption">A path into the lesson</span>
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
          <strong>Identify</strong>
          <small>Your trusted account</small>
        </li>
        <li className={waypoint === 1 ? 'is-current' : ''}>
          <span>02</span>
          <strong>Find your studio</strong>
          <small>
            {state === 'membershipRequired'
              ? 'Waiting for access'
              : 'Your shared workspace'}
          </small>
        </li>
        <li>
          <span>03</span>
          <strong>Keep learning</strong>
          <small>Return to your work</small>
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
