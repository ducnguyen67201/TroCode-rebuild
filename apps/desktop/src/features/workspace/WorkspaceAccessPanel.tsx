import type { WorkspaceMember, WorkspaceSummary } from '@tro/contracts';
import { useEffect, useRef, useState, type FormEvent } from 'react';
import type {
  AddWorkspaceMemberRole,
  WorkspaceClient,
} from '../../platform/workspace-client';
import { localeKey, useLanguage } from '../../i18n';

export function WorkspaceAccessPanel({
  client,
  workspace,
}: {
  client: WorkspaceClient;
  workspace: WorkspaceSummary;
}) {
  const { t } = useLanguage();
  const [members, setMembers] = useState<WorkspaceMember[]>([]);
  const [email, setEmail] = useState('');
  const [role, setRole] = useState<AddWorkspaceMemberRole>('student');
  const [busy, setBusy] = useState(false);
  const [loading, setLoading] = useState(true);
  const [message, setMessage] = useState('');
  const [error, setError] = useState('');
  const [confirming, setConfirming] = useState<string | null>(null);
  const epoch = useRef(0);

  async function load(ticket = ++epoch.current) {
    setLoading(true);
    setError('');
    try {
      const result = await client.members(workspace.workspaceId);
      if (ticket === epoch.current) setMembers(result.members);
    } catch {
      if (ticket === epoch.current) setError(t('workspace.loadError'));
    } finally {
      if (ticket === epoch.current) setLoading(false);
    }
  }

  useEffect(() => {
    const ticket = ++epoch.current;
    void load(ticket);
    return () => {
      epoch.current += 1;
    };
  }, [client, workspace.workspaceId]);

  async function add(event: FormEvent) {
    event.preventDefault();
    if (busy) return;
    const normalized = email.trim();
    if (!validEmail(normalized)) {
      setError(t('workspace.invalidEmail'));
      return;
    }
    const ticket = ++epoch.current;
    setBusy(true);
    setError('');
    setMessage('');
    try {
      await client.addMember(workspace.workspaceId, normalized, role);
      if (ticket !== epoch.current) return;
      setEmail('');
      setMessage(t('workspace.added'));
      await load(ticket);
    } catch {
      if (ticket === epoch.current) setError(t('workspace.addError'));
    } finally {
      if (ticket === epoch.current) setBusy(false);
    }
  }

  async function remove(member: WorkspaceMember) {
    if (confirming !== member.membershipId) {
      setConfirming(member.membershipId);
      setMessage(t('workspace.confirmMessage', { email: member.email }));
      return;
    }
    const ticket = ++epoch.current;
    setBusy(true);
    setError('');
    setMessage('');
    try {
      await client.removeMember(workspace.workspaceId, member.membershipId);
      if (ticket !== epoch.current) return;
      setConfirming(null);
      setMessage(t('workspace.removed'));
      await load(ticket);
    } catch {
      if (ticket === epoch.current) setError(t('workspace.removeError'));
    } finally {
      if (ticket === epoch.current) setBusy(false);
    }
  }

  return (
    <section
      className="workspace-access"
      aria-labelledby="workspace-access-heading"
    >
      <div className="card-heading">
        <div>
          <p className="eyebrow">{t('workspace.eyebrow')}</p>
          <h2 id="workspace-access-heading">{t('workspace.heading')}</h2>
        </div>
        <span className="badge">
          {t('workspace.assigned', { count: members.length })}
        </span>
      </div>
      <p>{t('workspace.description')}</p>

      <form
        className="workspace-access-form"
        onSubmit={(event) => void add(event)}
      >
        <label>
          {t('workspace.email')}
          <input
            autoComplete="email"
            disabled={busy}
            maxLength={254}
            onChange={(event) => setEmail(event.target.value)}
            placeholder="student@example.com"
            type="email"
            value={email}
          />
        </label>
        <label>
          {t('workspace.role')}
          <select
            disabled={busy}
            onChange={(event) =>
              setRole(event.target.value as AddWorkspaceMemberRole)
            }
            value={role}
          >
            <option value="student">{t('role.student')}</option>
            <option value="teacher">{t('role.teacher')}</option>
          </select>
        </label>
        <button disabled={busy} type="submit">
          {busy ? t('workspace.saving') : t('workspace.add')}
        </button>
      </form>

      {error && <p role="alert">{error}</p>}
      {message && <p role="status">{message}</p>}
      {loading ? (
        <p>{t('workspace.loading')}</p>
      ) : (
        <ul className="workspace-member-list">
          {members.map((member) => {
            const roleKey = localeKey('role', member.role);
            const stateKey = localeKey('member', member.state);
            return (
              <li key={member.membershipId}>
                <span className="workspace-member-identity">
                  <strong>{member.displayName ?? member.email}</strong>
                  {member.displayName && <small>{member.email}</small>}
                </span>
                <span className="workspace-member-meta">
                  <span>{roleKey ? t(roleKey) : member.role}</span>
                  <span
                    className={`member-state member-state--${member.state}`}
                  >
                    {stateKey ? t(stateKey) : member.state}
                  </span>
                </span>
                {member.role !== 'owner' && (
                  <button
                    className={
                      confirming === member.membershipId
                        ? 'remove-access is-confirming'
                        : 'remove-access'
                    }
                    disabled={busy}
                    onClick={() => void remove(member)}
                    type="button"
                  >
                    {confirming === member.membershipId
                      ? t('workspace.confirmRemoval')
                      : t('workspace.remove')}
                  </button>
                )}
              </li>
            );
          })}
        </ul>
      )}
    </section>
  );
}

function validEmail(value: string) {
  const parts = value.split('@');
  return (
    value.length >= 3 &&
    value.length <= 254 &&
    parts.length === 2 &&
    Boolean(parts[0]) &&
    Boolean(parts[1]) &&
    !/\s/.test(value)
  );
}
