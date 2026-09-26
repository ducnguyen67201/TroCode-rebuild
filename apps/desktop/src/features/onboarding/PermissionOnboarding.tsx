import type {
  DeviceReadiness,
  PermissionRequestKind,
  PermissionSettingsTarget,
} from '@tro/contracts';
import { useEffect, useRef, useState, type ReactNode } from 'react';
import type { DeviceClient } from '../../platform/device-client';
import { GuidedAction } from './GuidedAction';
import { permissionSettingsCopy } from './PermissionSettingsGuide';
import {
  canComplete,
  nextStep,
  readOnboardingMarker,
  requiredScreenReady,
  shouldShowOnboarding,
  writeOnboardingMarker,
  type OnboardingMarker,
} from './onboarding-state';

const boundaryMessage =
  'Tro could not verify device readiness. Check your connection to the desktop host and try again.';

export function DeviceOnboardingGate({
  client,
  skipIntro = false,
  children,
}: {
  client: DeviceClient;
  skipIntro?: boolean;
  children: ReactNode;
}) {
  const [readiness, setReadiness] = useState<DeviceReadiness | null>(null);
  const [marker, setMarker] = useState(() => readOnboardingMarker());
  const [failed, setFailed] = useState(false);
  const epoch = useRef(0);

  useEffect(() => {
    let active = true;
    const ticket = ++epoch.current;
    void client
      .check()
      .then((value) => {
        if (active && ticket === epoch.current) {
          setReadiness(value);
          setFailed(false);
        }
      })
      .catch(() => {
        if (active && ticket === epoch.current) setFailed(true);
      });
    return () => {
      active = false;
      epoch.current += 1;
    };
  }, [client]);

  if (!readiness) {
    return (
      <main className="device-onboarding-shell">
        <section className="device-onboarding-card" aria-live="polite">
          <p className="eyebrow">Device checkup</p>
          <h1>
            {failed ? 'Device check unavailable' : 'Checking this device…'}
          </h1>
          <p>
            {failed ? boundaryMessage : 'No permission prompt will appear.'}
          </p>
          {failed && (
            <button
              type="button"
              onClick={() => {
                setFailed(false);
                const ticket = ++epoch.current;
                void client
                  .check()
                  .then((value) => {
                    if (ticket === epoch.current) setReadiness(value);
                  })
                  .catch(() => {
                    if (ticket === epoch.current) setFailed(true);
                  });
              }}
            >
              Try again
            </button>
          )}
        </section>
      </main>
    );
  }

  if (skipIntro || !shouldShowOnboarding(readiness, marker)) return children;

  return (
    <PermissionCheckup
      client={client}
      initialReadiness={readiness}
      onReadiness={setReadiness}
      onComplete={(choice) => {
        const nextMarker: OnboardingMarker = {
          version: 1,
          microphoneChoice: choice,
        };
        writeOnboardingMarker(nextMarker);
        setMarker(nextMarker);
      }}
    />
  );
}

export function PermissionCheckup({
  client,
  initialReadiness = null,
  compact = false,
  onReadiness,
  onComplete,
}: {
  client: DeviceClient;
  initialReadiness?: DeviceReadiness | null;
  compact?: boolean;
  onReadiness?: (readiness: DeviceReadiness) => void;
  onComplete?: (choice: OnboardingMarker['microphoneChoice']) => void;
}) {
  const [readiness, setReadiness] = useState<DeviceReadiness | null>(
    initialReadiness,
  );
  const [choice, setChoice] = useState<
    OnboardingMarker['microphoneChoice'] | null
  >(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState('');
  const [settingsTarget, setSettingsTarget] =
    useState<PermissionSettingsTarget | null>(null);
  const epoch = useRef(0);
  const busyRef = useRef(false);
  const heading = useRef<HTMLHeadingElement>(null);
  const step = readiness ? nextStep(readiness, choice) : 'screen';
  const Shell = compact ? 'section' : 'main';
  const Card = compact ? 'div' : 'section';

  function accept(value: DeviceReadiness) {
    setReadiness(value);
    onReadiness?.(value);
  }

  async function perform(action: () => Promise<DeviceReadiness>) {
    if (busyRef.current) return;
    busyRef.current = true;
    const ticket = ++epoch.current;
    setBusy(true);
    setError('');
    try {
      // Yield a paint so the local pointer is gone before a native prompt can appear.
      await new Promise<void>((resolve) => setTimeout(resolve, 0));
      if (ticket !== epoch.current) return;
      const value = await action();
      if (ticket === epoch.current) {
        setSettingsTarget(null);
        accept(value);
      }
    } catch {
      if (ticket === epoch.current)
        setError(
          'The device check did not finish. Your previous status is unchanged.',
        );
    } finally {
      if (ticket === epoch.current) {
        busyRef.current = false;
        setBusy(false);
      }
    }
  }

  useEffect(() => {
    if (initialReadiness) return;
    const ticket = ++epoch.current;
    void client
      .check()
      .then((value) => {
        if (ticket === epoch.current) accept(value);
      })
      .catch(() => {
        if (ticket === epoch.current) setError(boundaryMessage);
      });
    return () => {
      epoch.current += 1;
    };
  }, [client, initialReadiness]);

  useEffect(
    () => () => {
      epoch.current += 1;
      busyRef.current = false;
    },
    [],
  );

  useEffect(() => {
    if (!compact) heading.current?.focus();
  }, [compact, step]);

  const request = (kind: PermissionRequestKind) =>
    perform(() => client.request(kind));

  async function openSettings(target: PermissionSettingsTarget) {
    if (busyRef.current) return;
    busyRef.current = true;
    const ticket = ++epoch.current;
    setBusy(true);
    setError('');
    try {
      await new Promise<void>((resolve) => setTimeout(resolve, 0));
      if (ticket !== epoch.current) return;
      await client.openSettings(target);
      if (ticket === epoch.current) setSettingsTarget(target);
    } catch {
      if (ticket === epoch.current)
        setError(
          'Tro could not open device settings. Open the page manually, then recheck.',
        );
    } finally {
      if (ticket === epoch.current) {
        busyRef.current = false;
        setBusy(false);
      }
    }
  }

  async function relaunch() {
    if (busyRef.current) return;
    busyRef.current = true;
    setBusy(true);
    setError('');
    try {
      await client.relaunch();
      busyRef.current = false;
      setBusy(false);
    } catch {
      setError(
        'Tro could not relaunch. Close and reopen the app, then recheck.',
      );
      busyRef.current = false;
      setBusy(false);
    }
  }

  if (!readiness) {
    return (
      <Shell
        className={
          compact ? 'permission-checkup compact' : 'device-onboarding-shell'
        }
      >
        <Card
          className={compact ? '' : 'device-onboarding-card'}
          aria-live="polite"
        >
          <h2>Checking device permissions…</h2>
          {error && (
            <>
              <p role="alert">{error}</p>
              <button
                type="button"
                onClick={() => void perform(() => client.check())}
              >
                Try again
              </button>
            </>
          )}
        </Card>
      </Shell>
    );
  }

  const screenReady = requiredScreenReady(readiness);
  const screenSettingsTarget = nextScreenSettingsTarget(readiness);
  const effectiveChoice =
    choice ??
    (['granted', 'available'].includes(readiness.microphone.status)
      ? 'granted'
      : null);

  return (
    <Shell
      className={
        compact ? 'permission-checkup compact' : 'device-onboarding-shell'
      }
    >
      <Card
        className={compact ? '' : 'device-onboarding-card'}
        aria-labelledby="device-checkup-heading"
      >
        {!compact && <p className="eyebrow">Private device setup</p>}
        {compact ? (
          <h2 id="device-checkup-heading" ref={heading} tabIndex={-1}>
            Device permissions
          </h2>
        ) : (
          <h1 id="device-checkup-heading" ref={heading} tabIndex={-1}>
            Set up this device
          </h1>
        )}
        <p>
          Tro observes only the window you choose. It does not save screen
          video, and this check retains no audio.
        </p>
        <div className="permission-steps">
          <article
            className={
              step === 'screen'
                ? 'permission-step is-current'
                : 'permission-step'
            }
          >
            <div className="permission-step-heading">
              <span aria-hidden="true">1</span>
              <div>
                <h2>Screen &amp; controls</h2>
                <strong>Required</strong>
              </div>
            </div>
            <CapabilityStatus
              label="Screen capture"
              capability={readiness.screenCapture}
            />
            {readiness.platform === 'macos' && (
              <CapabilityStatus
                label="Accessibility"
                capability={readiness.accessibility}
              />
            )}
            {readiness.platform === 'windows' && (
              <p className="permission-note">
                Windows uses its secure window picker when you start Observe.
                There is no app-list setting to change here.
              </p>
            )}
            {settingsTarget && settingsTarget !== 'microphone' && (
              <SettingsHandoff
                platform={readiness.platform}
                target={settingsTarget}
              />
            )}
            {(!screenReady || compact) && (
              <GuidedAction
                active={
                  !compact &&
                  step === 'screen' &&
                  !busy &&
                  settingsTarget === null
                }
                caption={
                  screenSettingsTarget === 'accessibility'
                    ? 'Next, let’s add Tro to Accessibility.'
                    : 'Let’s open the exact Screen Recording page.'
                }
              >
                <button
                  type="button"
                  disabled={busy}
                  onClick={() =>
                    void (settingsTarget && settingsTarget !== 'microphone'
                      ? perform(() => client.check())
                      : screenSettingsTarget
                        ? openSettings(screenSettingsTarget)
                        : perform(() => client.check()))
                  }
                >
                  {busy && step === 'screen'
                    ? settingsTarget
                      ? 'Rechecking…'
                      : 'Opening settings…'
                    : settingsTarget && settingsTarget !== 'microphone'
                      ? 'I changed it — recheck'
                      : screenSettingsTarget
                        ? `Open ${settingsTitle(screenSettingsTarget)} settings`
                        : 'Recheck screen access'}
                </button>
              </GuidedAction>
            )}
          </article>

          <article
            className={
              step === 'microphone'
                ? 'permission-step is-current'
                : 'permission-step'
            }
          >
            <div className="permission-step-heading">
              <span aria-hidden="true">2</span>
              <div>
                <h2>Microphone</h2>
                <strong>Optional</strong>
              </div>
            </div>
            <CapabilityStatus
              label="Microphone"
              capability={readiness.microphone}
            />
            <p className="permission-note">
              This prepares optional push-to-talk. Text always works, and the
              probe stores no samples.
            </p>
            {settingsTarget === 'microphone' && (
              <SettingsHandoff
                platform={readiness.platform}
                target="microphone"
              />
            )}
            {(step === 'microphone' || compact) && (
              <div className="permission-actions">
                {readiness.microphone.canRequest && (
                  <GuidedAction
                    active={!compact && step === 'microphone' && !busy}
                    caption="You can check your microphone here, or keep using text."
                  >
                    <button
                      type="button"
                      disabled={busy}
                      onClick={() => void request('microphone')}
                    >
                      {busy ? 'Checking…' : 'Check microphone'}
                    </button>
                  </GuidedAction>
                )}
                {!readiness.microphone.canRequest &&
                  readiness.microphone.recovery === 'manualSettings' &&
                  readiness.platform !== 'unsupported' && (
                    <GuidedAction
                      active={
                        !compact &&
                        step === 'microphone' &&
                        !busy &&
                        settingsTarget === null
                      }
                      caption="Open the exact microphone privacy page."
                    >
                      <button
                        type="button"
                        disabled={busy}
                        onClick={() =>
                          void (settingsTarget === 'microphone'
                            ? perform(() => client.check())
                            : openSettings('microphone'))
                        }
                      >
                        {busy
                          ? settingsTarget === 'microphone'
                            ? 'Rechecking…'
                            : 'Opening settings…'
                          : settingsTarget === 'microphone'
                            ? 'I changed it — recheck'
                            : 'Open Microphone settings'}
                      </button>
                    </GuidedAction>
                  )}
                {!compact && (
                  <button
                    className="button-secondary"
                    type="button"
                    disabled={busy}
                    onClick={() => {
                      setSettingsTarget(null);
                      setChoice('text');
                    }}
                  >
                    Use text instead
                  </button>
                )}
              </div>
            )}
          </article>

          {!compact && (
            <article
              className={
                step === 'ready'
                  ? 'permission-step is-current'
                  : 'permission-step'
              }
            >
              <div className="permission-step-heading">
                <span aria-hidden="true">3</span>
                <div>
                  <h2>Ready to learn</h2>
                  <strong>
                    {readiness.requiresRelaunch
                      ? 'Relaunch needed'
                      : 'Final step'}
                  </strong>
                </div>
              </div>
              {readiness.requiresRelaunch ? (
                <button
                  type="button"
                  disabled={busy}
                  onClick={() => void relaunch()}
                >
                  Relaunch Tro
                </button>
              ) : (
                <button
                  type="button"
                  disabled={!canComplete(readiness, effectiveChoice)}
                  onClick={() => onComplete?.(effectiveChoice ?? 'text')}
                >
                  Continue to Learn
                </button>
              )}
            </article>
          )}
        </div>
        {error && (
          <p className="permission-error" role="alert">
            {error}
          </p>
        )}
        <p className="permission-summary" aria-live="polite">
          {readiness.message}
        </p>
        {compact && (
          <div className="permission-actions">
            <button
              className="button-secondary"
              type="button"
              disabled={busy}
              onClick={() => void perform(() => client.check())}
            >
              Recheck device
            </button>
            {readiness.requiresRelaunch && (
              <button
                type="button"
                disabled={busy}
                onClick={() => void relaunch()}
              >
                Relaunch Tro
              </button>
            )}
          </div>
        )}
      </Card>
    </Shell>
  );
}

function nextScreenSettingsTarget(
  readiness: DeviceReadiness,
): PermissionSettingsTarget | null {
  if (readiness.platform !== 'macos') return null;
  if (!capabilityReady(readiness.screenCapture.status)) return 'screenCapture';
  if (!capabilityReady(readiness.accessibility.status)) return 'accessibility';
  return null;
}

function capabilityReady(status: DeviceReadiness['screenCapture']['status']) {
  return status === 'granted' || status === 'available';
}

function settingsTitle(target: PermissionSettingsTarget) {
  return target === 'screenCapture'
    ? 'Screen Recording'
    : target === 'accessibility'
      ? 'Accessibility'
      : 'Microphone';
}

function SettingsHandoff({
  platform,
  target,
}: {
  platform: DeviceReadiness['platform'];
  target: PermissionSettingsTarget;
}) {
  const copy = permissionSettingsCopy(platform, target);
  return (
    <div className="permission-settings-handoff" aria-live="polite">
      <strong>{copy.title} settings are open</strong>
      <span>{copy.path}</span>
      <p>{copy.instruction}</p>
      <small>Tro’s floating guide only points. You make every change.</small>
    </div>
  );
}

function CapabilityStatus({
  label,
  capability,
}: {
  label: string;
  capability: DeviceReadiness['screenCapture'];
}) {
  return (
    <div className="capability-status">
      <span
        className={`status-dot status-${capability.status}`}
        aria-hidden="true"
      />
      <div>
        <strong>
          {label}: {statusLabel(capability.status)}
        </strong>
        <p>{capability.message}</p>
      </div>
    </div>
  );
}

function statusLabel(status: DeviceReadiness['screenCapture']['status']) {
  return {
    granted: 'Ready',
    available: 'Available',
    notDetermined: 'Not checked',
    denied: 'Needs attention',
    unavailable: 'Unavailable',
    unknown: 'Not confirmed',
  }[status];
}
