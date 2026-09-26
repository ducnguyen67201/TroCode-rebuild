import type { FormEvent } from 'react';
import { useEffect, useRef, useState } from 'react';
import type { VoiceStatus } from '@tro/contracts';
import type { VoiceClient } from '../../platform/voice-client';

const busyPhases = new Set([
  'listening',
  'transcribing',
  'dispatching',
  'planning',
  'guiding',
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
      setError('Voice guidance could not complete that request.');
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
        <p>Loading voice guidance…</p>
      </section>
    );
  const busy = busyPhases.has(status.phase);
  const transcript = cleared
    ? ''
    : status.finalTranscript || status.partialTranscript;
  return (
    <section className="voice-panel" aria-labelledby="voice-heading">
      <div className="voice-heading-row">
        <div>
          <p className="eyebrow">Ask Tro to show you</p>
          <h2 id="voice-heading">Hold two keys. Ask. Release.</h2>
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
            Cancel guidance
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
            placeholder="e.g. Show me how to open the settings panel"
          />
          <button disabled={!instruction.trim() || busy} type="submit">
            Show me how
          </button>
        </div>
      </form>
      {transcript && (
        <div className="voice-transcript">
          <span>
            {status.finalTranscript ? 'Your request' : 'Listening transcript'}
          </span>
          <p>{transcript}</p>
          {!busy && (
            <button className="link-button" onClick={() => setCleared(true)}>
              Clear transcript
            </button>
          )}
        </div>
      )}
      {status.targetTitle && (
        <p className="voice-target">
          Selected window: <strong>{status.targetTitle}</strong>
        </p>
      )}
    </section>
  );
}
