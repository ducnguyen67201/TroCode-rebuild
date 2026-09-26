import type { DeviceReadiness } from '@tro/contracts';

export const ONBOARDING_MARKER_KEY = 'tro.device-onboarding.v1';

export interface OnboardingMarker {
  version: 1;
  microphoneChoice: 'granted' | 'text';
}

export type OnboardingStep = 'screen' | 'microphone' | 'ready';

export function parseOnboardingMarker(
  value: string | null,
): OnboardingMarker | null {
  if (!value) return null;
  try {
    const parsed: unknown = JSON.parse(value);
    if (
      typeof parsed === 'object' &&
      parsed !== null &&
      Object.keys(parsed).length === 2 &&
      'version' in parsed &&
      parsed.version === 1 &&
      'microphoneChoice' in parsed &&
      (parsed.microphoneChoice === 'granted' ||
        parsed.microphoneChoice === 'text')
    ) {
      return parsed as OnboardingMarker;
    }
  } catch {
    // A malformed local hint is treated as absent; native state stays authoritative.
  }
  return null;
}

export function readOnboardingMarker(
  storage: Pick<Storage, 'getItem'> = window.localStorage,
): OnboardingMarker | null {
  return parseOnboardingMarker(storage.getItem(ONBOARDING_MARKER_KEY));
}

export function writeOnboardingMarker(
  marker: OnboardingMarker,
  storage: Pick<Storage, 'setItem'> = window.localStorage,
) {
  storage.setItem(ONBOARDING_MARKER_KEY, JSON.stringify(marker));
}

export function requiredScreenReady(readiness: DeviceReadiness): boolean {
  if (readiness.platform === 'macos') {
    return (
      readiness.screenCapture.status === 'granted' &&
      readiness.accessibility.status === 'granted'
    );
  }
  if (readiness.platform === 'windows') {
    return ['available', 'granted', 'unknown'].includes(
      readiness.screenCapture.status,
    );
  }
  return false;
}

export function shouldShowOnboarding(
  readiness: DeviceReadiness,
  marker: OnboardingMarker | null,
): boolean {
  return (
    !marker || !requiredScreenReady(readiness) || readiness.requiresRelaunch
  );
}

export function canComplete(
  readiness: DeviceReadiness,
  microphoneChoice: OnboardingMarker['microphoneChoice'] | null,
): boolean {
  if (!requiredScreenReady(readiness) || readiness.requiresRelaunch)
    return false;
  return (
    microphoneChoice !== null ||
    ['granted', 'available'].includes(readiness.microphone.status)
  );
}

export function nextStep(
  readiness: DeviceReadiness,
  microphoneChoice: OnboardingMarker['microphoneChoice'] | null,
): OnboardingStep {
  if (!requiredScreenReady(readiness)) return 'screen';
  if (readiness.requiresRelaunch) return 'ready';
  if (
    microphoneChoice ||
    ['granted', 'available'].includes(readiness.microphone.status)
  ) {
    return 'ready';
  }
  return 'microphone';
}
