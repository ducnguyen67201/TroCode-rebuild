import type { FormEvent } from 'react';
import { useEffect, useRef, useState } from 'react';
import type { VoiceStatus } from '@tro/contracts';
import type { VoiceClient } from '../../platform/voice-client';
import { localeKey, useLanguage } from '../../i18n';

const busyPhases = new Set([
  'listening',
  'transcribing',
  'dispatching',
  'executing',
  'confirmation',
]);

export function VoiceControlPanel({ client }: { client: VoiceClient }) {
  const { localizeMessage, t } = useLanguage();
  const [status, setStatus] = useState<VoiceStatus | null>(null);
  const [instruction, setInstruction] = useState('');
  const [error, setError] = useState('');
  const [cleared, setCleared] = useState(false);
  const latest = useRef<VoiceStatus | null>(null);

  useEffect(() => {
    let active = true;
    const accept = (next: VoiceStatus) => {
      const current = latest.current;
      if (!active || (current && next.revision <= current.revision)) return;
      latest.current = next;
      setStatus(next);
      setCleared(false);
    };
    void client
      .status()
      .then(accept)
      .catch(() => setError(t('voice.statusUnavailable')));
    let unsubscribe: (() => void) | undefined;
    void client
      .subscribe(accept, () => setError(t('voice.statusUnverified')))
      .then((value) => {
        unsubscribe = value;
      });
    return () => {
      active = false;
      unsubscribe?.();
    };
  }, [client]);

  async function run(operation: () => Promise<VoiceStatus>) {
    setError('');
    try {
      const next = await operation();
      if (!latest.current || next.revision > latest.current.revision) {
        latest.current = next;
        setStatus(next);
      }
    } catch {
      setError(t('voice.requestFailed'));
    }
  }

  function submit(event: FormEvent) {
    event.preventDefault();
    const value = instruction.trim();
    if (!value) return;
    setInstruction('');
    void run(() => client.executeText(value));
  }

  if (!status)
    return (
      <section className="voice-panel" aria-busy="true">
        <p>{t('voice.loading')}</p>
      </section>
    );
  const busy = busyPhases.has(status.phase);
  const latestFinalIsQueued =
    status.queuedInstructions.at(-1) === status.finalTranscript;
  const transcript = cleared
    ? ''
    : latestFinalIsQueued
      ? ''
      : status.finalTranscript || status.partialTranscript;
  const showQueue =
    (busy && Boolean(status.runId)) || status.queuedInstructions.length > 0;
  const phaseKey = localeKey('voice.phase', status.phase);
  return (
    <section
      className="voice-panel"
      aria-label={t('voice.aria')}
      aria-labelledby="voice-heading"
    >
      <div className="voice-heading-row">
        <div>
          <p className="eyebrow">{t('voice.eyebrow')}</p>
          <h2 id="voice-heading">{t('voice.heading')}</h2>
        </div>
        <span className={`voice-phase voice-phase-${status.phase}`}>
          {phaseKey ? t(phaseKey) : status.phase}
        </span>
      </div>
      <p className="voice-shortcut">
        <kbd>{status.shortcut}</kbd>
      </p>
      <p role="status" aria-live="polite">
        {localizeMessage(status.message, 'voice.status.fallback')}
      </p>
      {!status.permissions.ready && status.phase !== 'disabled' && (
        <p className="voice-recovery">
          {localizeMessage(
            status.permissions.recovery,
            'voice.permissionRecovery',
          )}
        </p>
      )}
      {error && (
        <p role="alert" className="voice-error">
          {error}
        </p>
      )}
      <div className="voice-actions">
        {status.phase === 'disabled' || !status.permissions.ready ? (
          <button onClick={() => void run(() => client.enable())}>
            {status.phase === 'disabled'
              ? t('voice.enable')
              : t('voice.permissions')}
          </button>
        ) : (
          <button
            disabled={busy}
            onClick={() => void run(() => client.disable())}
          >
            {t('voice.disable')}
          </button>
        )}
        {busy && (
          <button
            className="secondary"
            onClick={() => void run(() => client.cancel())}
          >
            {t('voice.cancel')}
          </button>
        )}
      </div>
      <form onSubmit={submit} className="voice-text-fallback">
        <label htmlFor="voice-instruction">{t('voice.textLabel')}</label>
        <div>
          <input
            id="voice-instruction"
            maxLength={2000}
            value={instruction}
            onChange={(event) => setInstruction(event.target.value)}
            placeholder={t('voice.textPlaceholder')}
          />
          <button disabled={!instruction.trim() || busy} type="submit">
            {t('voice.run')}
          </button>
        </div>
      </form>
      {transcript && (
        <div className="voice-transcript">
          <span>
            {status.finalTranscript
              ? t('voice.finalInstruction')
              : t('voice.listeningTranscript')}
          </span>
          <p>{transcript}</p>
          {!busy && (
            <button className="link-button" onClick={() => setCleared(true)}>
              {t('voice.clearTranscript')}
            </button>
          )}
        </div>
      )}
      {showQueue && (
        <section className="voice-queue" aria-labelledby="voice-queue-heading">
          <div className="voice-queue-heading">
            <h3 id="voice-queue-heading">{t('voice.queue')}</h3>
            <span
              aria-label={t('voice.queued', {
                count: status.queuedInstructions.length,
              })}
            >
              {status.queuedInstructions.length}
            </span>
          </div>
          {status.queuedInstructions.length > 0 ? (
            <ol>
              {status.queuedInstructions.map((queued, index) => (
                <li key={`${index}-${queued}`}>{queued}</li>
              ))}
            </ol>
          ) : (
            <p>
              {status.phase === 'listening' || status.phase === 'transcribing'
                ? t('voice.followUpPending')
                : t('voice.followUpHint', { shortcut: status.shortcut })}
            </p>
          )}
        </section>
      )}
      {status.targetTitle && (
        <p className="voice-target">
          {t('voice.selectedWindow')} <strong>{status.targetTitle}</strong>
        </p>
      )}
      {status.confirmation && status.runId && (
        <div className="voice-confirmation" role="alert">
          <h3>{t('voice.approvalRequired')}</h3>
          <p>{status.confirmation.summary}</p>
          <button
            onClick={() =>
              void run(() =>
                client.decide(
                  status.runId!,
                  status.confirmation!.confirmationId,
                  true,
                ),
              )
            }
          >
            {t('voice.approve')}
          </button>
          <button
            className="secondary"
            onClick={() =>
              void run(() =>
                client.decide(
                  status.runId!,
                  status.confirmation!.confirmationId,
                  false,
                ),
              )
            }
          >
            {t('voice.reject')}
          </button>
        </div>
      )}
    </section>
  );
}
