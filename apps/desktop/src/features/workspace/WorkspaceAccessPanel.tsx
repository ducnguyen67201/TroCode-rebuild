import type { WorkspaceMember, WorkspaceSummary } from '@tro/contracts';
import { useEffect, useRef, useState, type FormEvent } from 'react';
import type {
  AddWorkspaceMemberRole,
  WorkspaceClient,
} from '../../platform/workspace-client';

export function WorkspaceAccessPanel({
  client,
  workspace,
}: {
  client: WorkspaceClient;
  workspace: WorkspaceSummary;
}) {
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
      if (ticket === epoch.current)
        setError('Workspace members could not be loaded. Try again.');
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
      setError('Enter the exact Google email for this member.');
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
      setMessage(
        'Access added. Tro will connect this account after Google verifies that email.',
      );
      await load(ticket);
    } catch {
      if (ticket === epoch.current)
        setError(
          'Access could not be added. Check the email and existing role.',
        );
    } finally {
      if (ticket === epoch.current) setBusy(false);
    }
  }

  async function remove(member: WorkspaceMember) {
    if (confirming !== member.membershipId) {
      setConfirming(member.membershipId);
      setMessage(`Confirm removal for ${member.email}.`);
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
      setMessage('Workspace access removed.');
      await load(ticket);
    } catch {
      if (ticket === epoch.current)
        setError('Workspace access could not be removed. Try again.');
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
          <p className="eyebrow">Workspace authority</p>
          <h2 id="workspace-access-heading">People with access</h2>
        </div>
        <span className="badge">{members.length} assigned</span>
      </div>
      <p>
        Add the exact Google email. Tro connects it automatically after the
        account signs in—no invitation link required.
      </p>

      <form
        className="workspace-access-form"
        onSubmit={(event) => void add(event)}
      >
        <label>
          Google email
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
          Role
          <select
            disabled={busy}
            onChange={(event) =>
              setRole(event.target.value as AddWorkspaceMemberRole)
            }
            value={role}
          >
            <option value="student">Student</option>
            <option value="teacher">Teacher</option>
          </select>
        </label>
        <button disabled={busy} type="submit">
          {busy ? 'Saving…' : 'Add access'}
        </button>
      </form>

      {error && <p role="alert">{error}</p>}
      {message && <p role="status">{message}</p>}
      {loading ? (
        <p>Loading workspace members…</p>
      ) : (
        <ul className="workspace-member-list">
          {members.map((member) => (
            <li key={member.membershipId}>
              <span className="workspace-member-identity">
                <strong>{member.displayName ?? member.email}</strong>
                {member.displayName && <small>{member.email}</small>}
              </span>
              <span className="workspace-member-meta">
                <span>{member.role}</span>
                <span className={`member-state member-state--${member.state}`}>
                  {member.state}
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
                    ? 'Confirm removal'
                    : 'Remove access'}
                </button>
              )}
            </li>
          ))}
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
