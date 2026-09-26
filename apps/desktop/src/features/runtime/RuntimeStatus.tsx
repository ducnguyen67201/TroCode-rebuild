import { useEffect, useState } from 'react';
import type { RuntimeStatus as Status } from '@tro/contracts';
import type { DesktopClient, Profile } from '../../platform/desktop-client';
import { latestStatus, publicErrorKey } from './runtime-state';
import { localeKey, useLanguage } from '../../i18n';
const initial: Status = {
  state: 'stopped',
  generationId: null,
  revision: 0,
  message: 'Connecting to runtime status…',
};
export function RuntimeStatus({ client }: { client: DesktopClient }) {
  const { localizeMessage, t } = useLanguage();
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
        if (active) setError(t('runtime.connectError'));
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
      setError(t(publicErrorKey(error)));
    } finally {
      setBusy(false);
    }
  }
  const stateKey = localeKey('runtime.state', status.state);
  return (
    <section className="runtime-card" aria-labelledby="runtime-heading">
      <div className="card-heading">
        <h2 id="runtime-heading">{t('runtime.heading')}</h2>
        <span className={'badge ' + status.state}>
          {stateKey ? t(stateKey) : status.state}
        </span>
      </div>
      <p role="status">
        {localizeMessage(status.message, 'runtime.status.fallback')}
      </p>
      <div className="actions">
        <button
          disabled={busy || status.state === 'running'}
          onClick={() => void perform(() => client.start())}
        >
          {t('runtime.start')}
        </button>
        <button
          className="secondary"
          disabled={busy || status.state !== 'running'}
          onClick={() => void perform(() => client.health())}
        >
          {t('runtime.check')}
        </button>
        <button
          className="secondary"
          disabled={status.state === 'stopped'}
          onClick={() => void perform(() => client.stop())}
        >
          {t('runtime.stop')}
        </button>
        <button
          className="secondary"
          disabled={busy}
          onClick={() => void perform(() => client.restart())}
        >
          {t('runtime.restart')}
        </button>
      </div>
      <label className="profile">
        {t('runtime.profile')}
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
            {t('runtime.diagnosticOnly')}
          </option>
          <option value="teacher">{t('role.teacher')}</option>
          <option value="student-a">{t('runtime.studentA')}</option>
          <option value="student-b">{t('runtime.studentB')}</option>
        </select>
      </label>
      {error && (
        <p role="alert" className="error">
          {error}
        </p>
      )}
      <p className="fine-print">{t('runtime.finePrint')}</p>
    </section>
  );
}
