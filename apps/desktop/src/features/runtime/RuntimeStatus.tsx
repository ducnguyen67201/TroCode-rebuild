import { useEffect, useState } from 'react';
import type { RuntimeStatus as Status } from '@tro/contracts';
import type { DesktopClient, Profile } from '../../platform/desktop-client';
import { latestStatus, publicError } from './runtime-state';
const initial: Status = {
  state: 'stopped',
  generationId: null,
  revision: 0,
  message: 'Connecting to runtime status…',
};
export function RuntimeStatus({ client }: { client: DesktopClient }) {
  const [status, setStatus] = useState(initial);
  const [error, setError] = useState('');
  const [busy, setBusy] = useState(false);
  useEffect(() => {
    let active = true;
    let unsubscribe: (() => void) | undefined;
    const receive = (incoming: Status) => {
      if (active) setStatus((current) => latestStatus(current, incoming));
    };
    void client
      .subscribe(receive)
      .then(async (cleanup) => {
        if (!active) {
          cleanup();
          return;
        }
        unsubscribe = cleanup;
        receive(await client.status());
      })
      .catch(() => {
        if (active) setError('Unable to connect to runtime status.');
      });
    return () => {
      active = false;
      unsubscribe?.();
    };
  }, [client]);
  async function perform(action: () => Promise<Status>) {
    setBusy(true);
    setError('');
    try {
      const incoming = await action();
      setStatus((current) => latestStatus(current, incoming));
    } catch (error) {
      setError(publicError(error));
    } finally {
      setBusy(false);
    }
  }
  return (
    <section className="runtime-card" aria-labelledby="runtime-heading">
      <div className="card-heading">
        <h2 id="runtime-heading">Private teaching runtime</h2>
        <span className={'badge ' + status.state}>{status.state}</span>
      </div>
      <p role="status">{status.message}</p>
      <div className="actions">
        <button
          disabled={busy || status.state === 'running'}
          onClick={() => void perform(() => client.start())}
        >
          Start session
        </button>
        <button
          className="secondary"
          disabled={busy || status.state !== 'running'}
          onClick={() => void perform(() => client.health())}
        >
          Check connection
        </button>
        <button
          className="secondary"
          disabled={status.state === 'stopped'}
          onClick={() => void perform(() => client.stop())}
        >
          Stop
        </button>
        <button
          className="secondary"
          disabled={busy}
          onClick={() => void perform(() => client.restart())}
        >
          Restart runtime
        </button>
      </div>
      <label className="profile">
        Development profile
        <select
          defaultValue=""
          disabled={busy}
          onChange={(event) =>
            void perform(() =>
              client.selectAccount(event.target.value as Profile),
            )
          }
        >
          <option value="" disabled>
            Diagnostic only
          </option>
          <option value="teacher">Teacher</option>
          <option value="student-a">Student A</option>
          <option value="student-b">Student B</option>
        </select>
      </label>
      {error && (
        <p role="alert" className="error">
          {error}
        </p>
      )}
      <p className="fine-print">
        A diagnostic session checks the process connection. Teaching guidance
        observes a selected window and never performs computer input.
      </p>
    </section>
  );
}
