import { useEffect, useRef, useState } from 'react';
import type { TeachingState, TeachingCue } from '@tro/contracts';
import type { DesktopClient } from '../../platform/desktop-client';
import { TeachingOverlay } from './TeachingOverlay';
import { localeKey, useLanguage } from '../../i18n';

export function TeachingPanel({ client }: { client: DesktopClient }) {
  const { t } = useLanguage();
  const [state, setState] = useState<TeachingState | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState('');
  const [elementId, setElementId] = useState('');
  const [destinationId, setDestinationId] = useState('');
  const [gesture, setGesture] = useState<TeachingCue['gesture']>('point');
  const [caption, setCaption] = useState(
    'Use your own mouse or keyboard on the highlighted control.',
  );
  const [lessonLocale, setLessonLocale] = useState<TeachingCue['locale']>('en');
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
      .catch(() => setError(t('teaching.liveUnavailable')));
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
      if (ticket === epoch.current) setError(t('teaching.unavailable'));
    } finally {
      if (ticket === epoch.current) setBusy(false);
    }
  }
  const teaching = client.teaching;
  if (!teaching) return null;
  const readiness = (value: string) => {
    const key = localeKey('teaching.readiness', value);
    return key ? t(key) : value;
  };
  return (
    <section className="teaching-panel" aria-label={t('teaching.aria')}>
      <h2>{t('teaching.heading')}</h2>
      <p>{t('teaching.description')}</p>
      <button
        disabled={busy}
        onClick={() => {
          void teaching
            .connectProof()
            .catch(() => setError(t('teaching.proofUnavailable')));
        }}
      >
        {t('teaching.connectProof')}
      </button>
      <label>
        {t('teaching.question')}
        <input
          maxLength={1000}
          value={question}
          onChange={(event) => setQuestion(event.target.value)}
        />
      </label>
      <button
        disabled={busy || !state?.target || !question.trim()}
        onClick={() => void run(() => teaching.ask(question, lessonLocale))}
      >
        {t('teaching.plan')}
      </button>
      <button
        disabled={busy}
        onClick={() => void run(() => teaching.listTargets())}
      >
        {t('teaching.findWindows')}
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
        {t('teaching.stop')}
      </button>
      <label>
        {t('teaching.practiceWindow')}
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
            {t('teaching.selectWindow')}
          </option>
          {state?.targets.map((target) => (
            <option
              key={`${target.pid}:${target.window_id}`}
              value={`${target.pid}:${target.window_id}`}
            >
              {target.title || t('teaching.untitledWindow')}
            </option>
          ))}
        </select>
      </label>
      <button
        disabled={busy || !state?.target}
        onClick={() => void run(() => teaching.observe())}
      >
        {t('teaching.observe')}
      </button>
      {state?.readiness && (
        <p role="status">
          {t('teaching.observation')}: {readiness(state.readiness.observation)}.{' '}
          {t('teaching.screenshot')}: {readiness(state.readiness.screen)}.{' '}
          {t('teaching.accessibility')}:{' '}
          {readiness(state.readiness.accessibility)}. {t('teaching.model')}:{' '}
          {readiness(state.readiness.model)}.
          {state.readiness.reason === 'connect_model' &&
            ` ${t('teaching.connectModel')}`}
          {state.readiness.reason === 'observe_again' &&
            ` ${t('teaching.observeAgain')}`}
          {state.readiness.reason === 'retry_plan' &&
            ` ${t('teaching.retryPlan')}`}
        </p>
      )}
      {state?.journey && (
        <section aria-label={t('teaching.planAria')}>
          <h3>{t('teaching.steps')}</h3>
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
                ? t('teaching.resume')
                : t('teaching.pause')}
            </button>
          )}
          {teaching.planControl &&
            state.journey.status === 'awaiting_confirmation' && (
              <button
                disabled={busy}
                onClick={() => void run(() => teaching.planControl!('confirm'))}
              >
                {t('teaching.continue')}
              </button>
            )}
        </section>
      )}
      <details>
        <summary>{t('teaching.manualTools')}</summary>
        {state?.observation && (
          <fieldset disabled={busy}>
            <legend>{t('teaching.visualGuidance')}</legend>
            <label>
              {t('teaching.control')}
              <select
                value={elementId}
                onChange={(event) => setElementId(event.target.value)}
              >
                <option value="">{t('teaching.chooseControl')}</option>
                {state.observation.elements.map((element) => (
                  <option key={element.id} value={element.id}>
                    {element.label || element.role}
                  </option>
                ))}
              </select>
            </label>
            <label>
              {t('teaching.gesture')}
              <select
                value={gesture}
                onChange={(event) =>
                  setGesture(event.target.value as TeachingCue['gesture'])
                }
              >
                {(['point', 'click', 'drag', 'type', 'scroll'] as const).map(
                  (item) => (
                    <option key={item} value={item}>
                      {t(`gesture.${item}`)}
                    </option>
                  ),
                )}
              </select>
            </label>
            {gesture === 'drag' && (
              <label>
                {t('teaching.destination')}
                <select
                  value={destinationId}
                  onChange={(event) => setDestinationId(event.target.value)}
                >
                  <option value="">{t('teaching.chooseDestination')}</option>
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
                {t('teaching.direction')}
                <select
                  value={direction}
                  onChange={(event) =>
                    setDirection(event.target.value as typeof direction)
                  }
                >
                  {(['up', 'down', 'left', 'right'] as const).map((item) => (
                    <option key={item} value={item}>
                      {t(`direction.${item}`)}
                    </option>
                  ))}
                </select>
              </label>
            )}
            <label>
              {t('teaching.caption')}
              <input
                maxLength={400}
                value={caption}
                onChange={(event) => setCaption(event.target.value)}
              />
            </label>
            <label>
              {t('teaching.guidanceLanguage')}
              <select
                value={lessonLocale}
                onChange={(event) =>
                  setLessonLocale(event.target.value as typeof lessonLocale)
                }
              >
                <option value="en">English</option>
                <option value="vi">Tiếng Việt</option>
              </select>
              <small className="field-note">
                {t('teaching.guidanceLanguageNote')}
              </small>
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
                    locale: lessonLocale,
                    destinationId: gesture === 'drag' ? destinationId : null,
                    direction: gesture === 'scroll' ? direction : null,
                  }),
                )
              }
            >
              {t('teaching.showGuidance')}
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
            {t('teaching.expectedLabel')}
            <input
              value={label}
              onChange={(event) => setLabel(event.target.value)}
            />
          </label>
          <label>
            {t('teaching.expectedValue')}
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
            {t('teaching.check')}
          </button>
        </>
      )}
      {state?.check && (
        <p role="status">
          {(() => {
            const key = localeKey('teaching.outcome', state.check.outcome);
            return key ? t(key) : state.check.outcome;
          })()}
          : {state.check.message}
        </p>
      )}
      {error && <p role="alert">{error}</p>}
    </section>
  );
}
