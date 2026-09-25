import { TeachingPanel } from './features/teaching/TeachingPanel';
import type { DesktopClient } from './platform/desktop-client';
import { RuntimeStatus } from './features/runtime/RuntimeStatus';
import type { AuthenticatedContext } from './features/auth/AuthGate';
import { WorkspaceAccessPanel } from './features/workspace/WorkspaceAccessPanel';

export function App({
  client,
  session,
}: {
  client: DesktopClient;
  session: AuthenticatedContext;
}) {
  const workspace = session.workspaces[0]!;
  return (
    <main className="app-shell">
      <header className="app-header">
        <a className="wordmark" href="#">
          tro<span> / foundation</span>
        </a>
        <div className="account-context">
          <span className="workspace-context">
            <strong>{workspace.name}</strong>
            <small>{workspace.role}</small>
          </span>
          <span className="account-avatar" aria-hidden="true">
            {session.user.displayName.charAt(0).toUpperCase()}
          </span>
          <span className="account-name">{session.user.displayName}</span>
          <button
            className="sign-out-button"
            disabled={session.signingOut}
            onClick={session.signOut}
          >
            {session.signingOut ? 'Signing out…' : 'Sign out'}
          </button>
        </div>
      </header>
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
      {client.preview && <p className="preview">Preview — simulated runtime</p>}
      {session.workspaces
        .filter((candidate) => candidate.role === 'owner')
        .map((candidate) => (
          <WorkspaceAccessPanel
            client={client.workspace}
            key={candidate.workspaceId}
            workspace={candidate}
          />
        ))}
      <RuntimeStatus client={client} />
      <TeachingPanel client={client} />
      <footer>React presentation · Rust supervision · Python runtime</footer>
    </main>
  );
}
