import type { FormEvent } from 'react';
import { useEffect, useRef, useState } from 'react';
import type { VoiceStatus } from '@tro/contracts';
import type { VoiceClient } from '../../platform/voice-client';

const busyPhases = new Set([
  'listening',
  'transcribing',
  'dispatching',
  'executing',
  'confirmation',
]);

export function VoiceControlPanel({ client }: { client: VoiceClient }) {
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
      .catch(() => setError('Voice status is unavailable.'));
    let unsubscribe: (() => void) | undefined;
    void client
      .subscribe(accept, () => setError('Voice status could not be verified.'))
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
      setError('Voice control could not complete that request.');
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
        <p>Loading voice control…</p>
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
  return (
    <section className="voice-panel" aria-labelledby="voice-heading">
      <div className="voice-heading-row">
        <div>
          <p className="eyebrow">Fast computer control</p>
          <h2 id="voice-heading">Hold two keys. Speak. Release.</h2>
        </div>
        <span className={`voice-phase voice-phase-${status.phase}`}>
          {status.phase}
        </span>
      </div>
      <p className="voice-shortcut">
        <kbd>{status.shortcut}</kbd>
      </p>
      <p role="status" aria-live="polite">
        {status.message}
      </p>
      {!status.permissions.ready && status.phase !== 'disabled' && (
        <p className="voice-recovery">{status.permissions.recovery}</p>
      )}
      {error && (
        <p role="alert" className="voice-error">
          {error}
        </p>
      )}
      <div className="voice-actions">
        {status.phase === 'disabled' || !status.permissions.ready ? (
          <button onClick={() => void run(() => client.enable())}>
            {status.phase === 'disabled' ? 'Enable voice' : 'Check permissions'}
          </button>
        ) : (
          <button
            disabled={busy}
            onClick={() => void run(() => client.disable())}
          >
            Disable voice
          </button>
        )}
        {busy && (
          <button
            className="secondary"
            onClick={() => void run(() => client.cancel())}
          >
            Cancel instruction
          </button>
        )}
      </div>
      <form onSubmit={submit} className="voice-text-fallback">
        <label htmlFor="voice-instruction">Type instead of speaking</label>
        <div>
          <input
            id="voice-instruction"
            maxLength={2000}
            value={instruction}
            onChange={(event) => setInstruction(event.target.value)}
            placeholder="e.g. Open the settings panel"
          />
          <button disabled={!instruction.trim() || busy} type="submit">
            Run instruction
          </button>
        </div>
      </form>
      {transcript && (
        <div className="voice-transcript">
          <span>
            {status.finalTranscript
              ? 'Final instruction'
              : 'Listening transcript'}
          </span>
          <p>{transcript}</p>
          {!busy && (
            <button className="link-button" onClick={() => setCleared(true)}>
              Clear transcript
            </button>
          )}
        </div>
      )}
      {showQueue && (
        <section className="voice-queue" aria-labelledby="voice-queue-heading">
          <div className="voice-queue-heading">
            <h3 id="voice-queue-heading">Follow-up queue</h3>
            <span aria-label={`${status.queuedInstructions.length} queued`}>
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
                ? 'Your follow-up will appear here after transcription.'
                : `Hold ${status.shortcut} again to add a follow-up.`}
            </p>
          )}
        </section>
      )}
      {status.targetTitle && (
        <p className="voice-target">
          Selected window: <strong>{status.targetTitle}</strong>
        </p>
      )}
      {status.confirmation && status.runId && (
        <div className="voice-confirmation" role="alert">
          <h3>Approval required</h3>
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
            Approve action
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
            Reject action
          </button>
        </div>
      )}
    </section>
  );
}
