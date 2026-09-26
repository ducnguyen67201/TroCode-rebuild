import type {
  DeviceCapability,
  DeviceReadiness,
  PermissionSettingsTarget,
} from '@tro/contracts';
import type { DeviceClient } from './device-client';

// Development-only staged projection. Native settings actions still cross the real
// Tauri bridge, while readiness begins from a deterministic fresh-device state.
export function createNativePermissionQaClient(
  native: DeviceClient,
): DeviceClient {
  let macStage = 0;
  let pendingTarget: PermissionSettingsTarget | null = null;

  function project(actual: DeviceReadiness): DeviceReadiness {
    if (actual.platform !== 'macos') return actual;
    return {
      ...actual,
      requiresRelaunch: false,
      message:
        'Permission QA is simulating a fresh Mac. Settings links still use the real native bridge.',
      screenCapture:
        macStage >= 1
          ? ready('Screen Recording is ready for this walkthrough.')
          : missing(
              'Open Screen Recording settings, add or enable Tro, then recheck.',
            ),
      accessibility:
        macStage >= 2
          ? ready('Accessibility is ready for this walkthrough.')
          : missing(
              'Open Accessibility settings, add or enable Tro, then recheck.',
            ),
      microphone:
        macStage >= 3
          ? actual.microphone
          : {
              status: 'notDetermined',
              required: false,
              canRequest: true,
              recovery: 'request',
              message: 'Microphone setup is optional.',
            },
    };
  }

  return {
    check: async () => {
      const actual = await native.check();
      if (pendingTarget === 'screenCapture') macStage = Math.max(macStage, 1);
      if (pendingTarget === 'accessibility') macStage = Math.max(macStage, 2);
      if (pendingTarget === 'microphone') macStage = Math.max(macStage, 3);
      pendingTarget = null;
      return project(actual);
    },
    request: async (kind) => {
      const actual = await native.request(kind);
      macStage = Math.max(macStage, 3);
      return project(actual);
    },
    openSettings: async (target) => {
      await native.openSettings(target);
      pendingTarget = target;
    },
    relaunch: () => native.relaunch(),
  };
}

function missing(message: string): DeviceCapability {
  return {
    status: 'unknown',
    required: true,
    canRequest: false,
    recovery: 'manualSettings',
    message,
  };
}

function ready(message: string): DeviceCapability {
  return {
    status: 'granted',
    required: true,
    canRequest: false,
    recovery: 'none',
    message,
  };
}
