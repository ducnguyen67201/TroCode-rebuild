import { useEffect, useRef, useState } from 'react';
import type { TeachingState, TeachingCue } from '@tro/contracts';
import type { DesktopClient } from '../../platform/desktop-client';
import { TeachingOverlay } from './TeachingOverlay';

export function TeachingPanel({ client }: { client: DesktopClient }) {
  const [state, setState] = useState<TeachingState | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState('');
  const [elementId, setElementId] = useState('');
  const [destinationId, setDestinationId] = useState('');
  const [gesture, setGesture] = useState<TeachingCue['gesture']>('point');
  const [caption, setCaption] = useState(
    'Use your own mouse or keyboard on the highlighted control.',
  );
  const [locale, setLocale] = useState<TeachingCue['locale']>('en');
  const [direction, setDirection] =
    useState<NonNullable<TeachingCue['direction']>>('down');
  const [label, setLabel] = useState('');
  const [expected, setExpected] = useState('');
  const [question, setQuestion] = useState(
    'Increase the counter once, type hello in Practice answer, then scroll Practice scroll area to the end.',
  );
  const epoch = useRef(0);
  const activeSession = useRef<string | null>(null);
  useEffect(() => {
    let disposed = false;
    let cleanup: (() => void) | undefined;
    void client.teaching
      ?.subscribe?.((next) => {
        if (next.session_id !== activeSession.current) return;
        setState((previous) =>
          previous && next.revision > previous.revision ? next : previous,
        );
      })
      .then((unsubscribe) => {
        if (disposed) unsubscribe();
        else cleanup = unsubscribe;
      })
      .catch(() => setError('Live guidance updates are unavailable.'));
    return () => {
      disposed = true;
      cleanup?.();
    };
  }, [client]);
  useEffect(() => {
    let disposed = false;
    let unsubscribe: (() => void) | undefined;
    void client
      .subscribe((status) => {
        if (status.state !== 'running') {
          epoch.current++;
          activeSession.current = null;
          setState(null);
          setBusy(false);
        }
      })
      .then((cleanup) => {
        if (disposed) cleanup();
        else unsubscribe = cleanup;
      });
    return () => {
      disposed = true;
      epoch.current++;
      unsubscribe?.();
    };
  }, [client]);
  async function run(action: () => Promise<TeachingState>) {
    const ticket = ++epoch.current;
    setBusy(true);
    setError('');
    try {
      const next = await action();
      if (ticket === epoch.current) {
        activeSession.current = next.session_id;
        setState((previous) =>
          previous?.session_id === next.session_id &&
          previous.revision > next.revision
            ? previous
            : next,
        );
      }
    } catch {
      if (ticket === epoch.current)
        setError(
          'Guidance is unavailable. Start the runtime, check permissions, and observe again.',
        );
    } finally {
      if (ticket === epoch.current) setBusy(false);
    }
  }
  const teaching = client.teaching;
  if (!teaching) return null;
  return (
    <section className="teaching-panel" aria-label="Visual teaching">
      <h2>Follow the cursor. Try it yourself.</h2>
      <button
        onClick={() => {
          void teaching
            .permissions()
            .then(setError)
            .catch(() => setError('Permission check unavailable.'));
        }}
      >
        Observation permissions
      </button>
      <p>
        Tro's instructor cursor points, circles and demonstrates the path. You
        perform every real click, drag, keystroke and scroll.
      </p>
      <button
        disabled={busy}
        onClick={() => {
          void teaching
            .connectProof()
            .catch(() =>
              setError(
                'Configure a private proof account on this device first.',
              ),
            );
        }}
      >
        Connect proof account
      </button>
      <label>
        What would you like help with?
        <input
          maxLength={1000}
          value={question}
          onChange={(event) => setQuestion(event.target.value)}
        />
      </label>
      <button
        disabled={busy || !state?.target || !question.trim()}
        onClick={() => void run(() => teaching.ask(question, locale))}
      >
        Show me how / show another way
      </button>
      <button
        disabled={busy}
        onClick={() => void run(() => teaching.listTargets())}
      >
        Find windows
      </button>
      <button
        onClick={() => {
          epoch.current++;
          activeSession.current = null;
          setState(null);
          setBusy(false);
          void client.stop();
        }}
      >
        Stop guidance
      </button>
      <label>
        Practice window
        <select
          disabled={busy}
          value={
            state?.target ? `${state.target.pid}:${state.target.window_id}` : ''
          }
          onChange={(event) => {
            const [pid, windowId] = event.target.value.split(':').map(Number);
            setElementId('');
            void run(() => teaching.selectTarget(pid!, windowId!));
          }}
        >
          <option value="" disabled>
            Select a window you opened
          </option>
          {state?.targets.map((target) => (
            <option
              key={`${target.pid}:${target.window_id}`}
              value={`${target.pid}:${target.window_id}`}
            >
              {target.title || 'Untitled window'}
            </option>
          ))}
        </select>
      </label>
      <button
        disabled={busy || !state?.target}
        onClick={() => void run(() => teaching.observe())}
      >
        Observe
      </button>
      {state?.readiness && (
        <p role="status">
          Observation: {state.readiness.observation}. Screenshot:{' '}
          {state.readiness.screen}. Accessibility:{' '}
          {state.readiness.accessibility}. Model: {state.readiness.model}.
          {state.readiness.reason === 'connect_model' &&
            ' Connect your proof account first.'}
          {state.readiness.reason === 'observe_again' &&
            ' Check observation access, then Observe again.'}
          {state.readiness.reason === 'retry_plan' &&
            ' Model access or planning failed. Reconnect if your grant expired, then explicitly replan. Previous progress is preserved.'}
        </p>
      )}
      {state?.journey && (
        <section aria-label="Teaching plan">
          <h3>Your steps</h3>
          <ol>
            {state.journey.steps.map((step, index) => (
              <li
                key={index}
                aria-current={
                  index === state.journey!.index ? 'step' : undefined
                }
              >
                {index < state.journey!.index ? '✓ ' : ''}
                {step}
              </li>
            ))}
          </ol>
          <p role="status">{state.journey.message}</p>
          {client.preview && state.cue && (
            <div className="cue-preview">
              <TeachingOverlay
                cue={state.cue}
                origin={state.target?.bounds}
                stepIndex={state.journey.index}
                stepTotal={state.journey.steps.length}
              />
            </div>
          )}
          {teaching.planControl && state.journey.status !== 'completed' && (
            <button
              disabled={busy}
              onClick={() =>
                void run(() =>
                  teaching.planControl!(
                    state.journey!.status === 'paused' ? 'resume' : 'pause',
                  ),
                )
              }
            >
              {state.journey.status === 'paused'
                ? 'Resume guidance'
                : 'Pause guidance'}
            </button>
          )}
          {teaching.planControl &&
            state.journey.status === 'awaiting_confirmation' && (
              <button
                disabled={busy}
                onClick={() => void run(() => teaching.planControl!('confirm'))}
              >
                Continue — I’m ready
              </button>
            )}
        </section>
      )}
      <details>
        <summary>Try one visual demonstration</summary>
        {state?.observation && (
          <fieldset disabled={busy}>
            <legend>Visual guidance</legend>
            <label>
              Control
              <select
                value={elementId}
                onChange={(event) => setElementId(event.target.value)}
              >
                <option value="">Choose an observed control</option>
                {state.observation.elements.map((element) => (
                  <option key={element.id} value={element.id}>
                    {element.label || element.role}
                  </option>
                ))}
              </select>
            </label>
            <label>
              Gesture
              <select
                value={gesture}
                onChange={(event) =>
                  setGesture(event.target.value as TeachingCue['gesture'])
                }
              >
                {(['point', 'click', 'drag', 'type', 'scroll'] as const).map(
                  (item) => (
                    <option key={item}>{item}</option>
                  ),
                )}
              </select>
            </label>
            {gesture === 'drag' && (
              <label>
                Destination
                <select
                  value={destinationId}
                  onChange={(event) => setDestinationId(event.target.value)}
                >
                  <option value="">Choose destination</option>
                  {state.observation.elements.map((element) => (
                    <option key={element.id} value={element.id}>
                      {element.label || element.role}
                    </option>
                  ))}
                </select>
              </label>
            )}
            {gesture === 'scroll' && (
              <label>
                Direction
                <select
                  value={direction}
                  onChange={(event) =>
                    setDirection(event.target.value as typeof direction)
                  }
                >
                  {['up', 'down', 'left', 'right'].map((item) => (
                    <option key={item}>{item}</option>
                  ))}
                </select>
              </label>
            )}
            <label>
              Caption
              <input
                maxLength={400}
                value={caption}
                onChange={(event) => setCaption(event.target.value)}
              />
            </label>
            <label>
              Language
              <select
                value={locale}
                onChange={(event) =>
                  setLocale(event.target.value as typeof locale)
                }
              >
                <option value="en">English</option>
                <option value="vi">Tiếng Việt</option>
              </select>
            </label>
            <button
              disabled={
                !elementId ||
                !caption.trim() ||
                (gesture === 'drag' && !destinationId)
              }
              onClick={() =>
                void run(() =>
                  teaching.explain({
                    elementId,
                    gesture,
                    caption,
                    locale,
                    destinationId: gesture === 'drag' ? destinationId : null,
                    direction: gesture === 'scroll' ? direction : null,
                  }),
                )
              }
            >
              Show / repeat guidance
            </button>
          </fieldset>
        )}
      </details>
      {state?.cue && !state.journey && (
        <>
          <p lang={state.cue.locale}>{state.cue.caption}</p>
          {client.preview && (
            <div className="cue-preview">
              <TeachingOverlay cue={state.cue} origin={state.target?.bounds} />
            </div>
          )}
          <label>
            Expected control label
            <input
              value={label}
              onChange={(event) => setLabel(event.target.value)}
            />
          </label>
          <label>
            Expected value
            <input
              value={expected}
              onChange={(event) => setExpected(event.target.value)}
            />
          </label>
          <button
            disabled={busy || !label}
            onClick={() =>
              void run(() => teaching.check(state.cue!.id, label, expected))
            }
          >
            I tried it — check
          </button>
        </>
      )}
      {state?.check && (
        <p role="status">
          {state.check.outcome}: {state.check.message}
        </p>
      )}
      {error && <p role="alert">{error}</p>}
    </section>
  );
}
