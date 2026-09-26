import { describe, expect, it } from 'vitest';
import type { DeviceReadiness } from '@tro/contracts';
import {
  canComplete,
  nextStep,
  parseOnboardingMarker,
  requiredScreenReady,
  shouldShowOnboarding,
} from './onboarding-state';

const ready: DeviceReadiness = {
  platform: 'macos',
  requiresRelaunch: false,
  message: 'Checked.',
  screenCapture: {
    status: 'granted',
    required: true,
    canRequest: false,
    recovery: 'none',
    message: 'Ready.',
  },
  accessibility: {
    status: 'granted',
    required: true,
    canRequest: false,
    recovery: 'none',
    message: 'Ready.',
  },
  microphone: {
    status: 'notDetermined',
    required: false,
    canRequest: true,
    recovery: 'request',
    message: 'Optional.',
  },
};

describe('device onboarding policy', () => {
  it('treats malformed or expanded local markers as absent', () => {
    expect(parseOnboardingMarker('{nope')).toBeNull();
    expect(
      parseOnboardingMarker(
        JSON.stringify({ version: 1, microphoneChoice: 'text', granted: true }),
      ),
    ).toBeNull();
    expect(
      parseOnboardingMarker(
        JSON.stringify({ version: 1, microphoneChoice: 'text' }),
      ),
    ).toEqual({ version: 1, microphoneChoice: 'text' });
  });

  it('requires both macOS grants and reopens after revocation', () => {
    const marker = { version: 1, microphoneChoice: 'text' } as const;
    expect(requiredScreenReady(ready)).toBe(true);
    expect(shouldShowOnboarding(ready, marker)).toBe(false);
    expect(
      shouldShowOnboarding(
        {
          ...ready,
          screenCapture: { ...ready.screenCapture, status: 'denied' },
        },
        marker,
      ),
    ).toBe(true);
  });

  it('allows honest Windows support but not unavailable capture', () => {
    const windows: DeviceReadiness = {
      ...ready,
      platform: 'windows',
      screenCapture: { ...ready.screenCapture, status: 'available' },
      accessibility: {
        ...ready.accessibility,
        required: false,
        status: 'available',
      },
    };
    expect(requiredScreenReady(windows)).toBe(true);
    expect(
      requiredScreenReady({
        ...windows,
        screenCapture: { ...windows.screenCapture, status: 'unavailable' },
      }),
    ).toBe(false);
  });

  it('keeps microphone optional and blocks completion during relaunch', () => {
    expect(nextStep(ready, null)).toBe('microphone');
    expect(canComplete(ready, 'text')).toBe(true);
    expect(canComplete(ready, null)).toBe(false);
    expect(canComplete({ ...ready, requiresRelaunch: true }, 'text')).toBe(
      false,
    );
    expect(nextStep({ ...ready, requiresRelaunch: true }, null)).toBe('ready');
  });
});
