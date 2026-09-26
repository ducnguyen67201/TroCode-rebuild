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
import { localeKey, translate, useLanguage, type AppLocale } from '../../i18n';

export function DeviceOnboardingGate({
  client,
  skipIntro = false,
  children,
}: {
  client: DeviceClient;
  skipIntro?: boolean;
  children: ReactNode;
}) {
  const { t } = useLanguage();
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
          <p className="eyebrow">{t('onboarding.eyebrow')}</p>
          <h1>
            {failed ? t('onboarding.unavailable') : t('onboarding.checking')}
          </h1>
          <p>
            {failed ? t('onboarding.boundaryError') : t('onboarding.noPrompt')}
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
              {t('onboarding.tryAgain')}
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
  const { locale, localizeMessage, t } = useLanguage();
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
      if (ticket === epoch.current) setError(t('onboarding.checkFailed'));
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
        if (ticket === epoch.current) setError(t('onboarding.boundaryError'));
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
        setError(t('onboarding.openSettingsFailed'));
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
      setError(t('onboarding.relaunchFailed'));
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
          <h2>{t('onboarding.checkingPermissions')}</h2>
          {error && (
            <>
              <p role="alert">{error}</p>
              <button
                type="button"
                onClick={() => void perform(() => client.check())}
              >
                {t('onboarding.tryAgain')}
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
        {!compact && <p className="eyebrow">{t('onboarding.privateSetup')}</p>}
        {compact ? (
          <h2 id="device-checkup-heading" ref={heading} tabIndex={-1}>
            {t('onboarding.permissions')}
          </h2>
        ) : (
          <h1 id="device-checkup-heading" ref={heading} tabIndex={-1}>
            {t('onboarding.setup')}
          </h1>
        )}
        <p>{t('onboarding.description')}</p>
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
                <h2>{t('onboarding.screenControls')}</h2>
                <strong>{t('onboarding.required')}</strong>
              </div>
            </div>
            <CapabilityStatus
              label={t('onboarding.screenCapture')}
              capability={readiness.screenCapture}
            />
            {readiness.platform === 'macos' && (
              <CapabilityStatus
                label={t('onboarding.accessibility')}
                capability={readiness.accessibility}
              />
            )}
            {readiness.platform === 'windows' && (
              <p className="permission-note">{t('onboarding.windowsPicker')}</p>
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
                    ? t('onboarding.nextAccessibility')
                    : t('onboarding.openScreenRecording')
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
                      ? t('onboarding.rechecking')
                      : t('onboarding.openingSettings')
                    : settingsTarget && settingsTarget !== 'microphone'
                      ? t('onboarding.changedRecheck')
                      : screenSettingsTarget
                        ? t('onboarding.openSettings', {
                            target: settingsTitle(screenSettingsTarget, locale),
                          })
                        : t('onboarding.recheckScreen')}
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
                <h2>{t('onboarding.microphone')}</h2>
                <strong>{t('onboarding.optional')}</strong>
              </div>
            </div>
            <CapabilityStatus
              label={t('onboarding.microphone')}
              capability={readiness.microphone}
            />
            <p className="permission-note">{t('onboarding.microphoneNote')}</p>
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
                    caption={t('onboarding.microphoneChoice')}
                  >
                    <button
                      type="button"
                      disabled={busy}
                      onClick={() => void request('microphone')}
                    >
                      {busy
                        ? t('onboarding.checkingMicrophone')
                        : t('onboarding.checkMicrophone')}
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
                      caption={t('onboarding.openMicrophone')}
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
                            ? t('onboarding.rechecking')
                            : t('onboarding.openingSettings')
                          : settingsTarget === 'microphone'
                            ? t('onboarding.changedRecheck')
                            : t('onboarding.openSettings', {
                                target: t('onboarding.microphone'),
                              })}
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
                    {t('onboarding.useText')}
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
                  <h2>{t('onboarding.ready')}</h2>
                  <strong>
                    {readiness.requiresRelaunch
                      ? t('onboarding.relaunchNeeded')
                      : t('onboarding.finalStep')}
                  </strong>
                </div>
              </div>
              {readiness.requiresRelaunch ? (
                <button
                  type="button"
                  disabled={busy}
                  onClick={() => void relaunch()}
                >
                  {t('onboarding.relaunch')}
                </button>
              ) : (
                <button
                  type="button"
                  disabled={!canComplete(readiness, effectiveChoice)}
                  onClick={() => onComplete?.(effectiveChoice ?? 'text')}
                >
                  {t('onboarding.continue')}
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
          {localizeMessage(readiness.message, 'onboarding.statusFallback')}
        </p>
        {compact && (
          <div className="permission-actions">
            <button
              className="button-secondary"
              type="button"
              disabled={busy}
              onClick={() => void perform(() => client.check())}
            >
              {t('onboarding.recheckDevice')}
            </button>
            {readiness.requiresRelaunch && (
              <button
                type="button"
                disabled={busy}
                onClick={() => void relaunch()}
              >
                {t('onboarding.relaunch')}
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

function settingsTitle(target: PermissionSettingsTarget, locale: AppLocale) {
  return target === 'screenCapture'
    ? translate(locale, 'permissionGuide.screenCapture.title')
    : target === 'accessibility'
      ? translate(locale, 'permissionGuide.accessibility.title')
      : translate(locale, 'permissionGuide.microphone.title');
}

function SettingsHandoff({
  platform,
  target,
}: {
  platform: DeviceReadiness['platform'];
  target: PermissionSettingsTarget;
}) {
  const { locale, t } = useLanguage();
  const copy = permissionSettingsCopy(platform, target, locale);
  return (
    <div className="permission-settings-handoff" aria-live="polite">
      <strong>{t('onboarding.settingsOpen', { title: copy.title })}</strong>
      <span>{copy.path}</span>
      <p>{copy.instruction}</p>
      <small>{t('onboarding.pointerBoundary')}</small>
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
  const { localizeMessage, t } = useLanguage();
  const statusKey = localeKey('onboarding.status', capability.status);
  return (
    <div className="capability-status">
      <span
        className={`status-dot status-${capability.status}`}
        aria-hidden="true"
      />
      <div>
        <strong>
          {label}: {statusKey ? t(statusKey) : capability.status}
        </strong>
        <p>
          {localizeMessage(capability.message, 'onboarding.capabilityFallback')}
        </p>
      </div>
    </div>
  );
}
