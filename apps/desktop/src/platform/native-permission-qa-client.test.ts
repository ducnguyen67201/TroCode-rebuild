import { expect, it, vi } from 'vitest';
import type { DeviceReadiness } from '@tro/contracts';
import type { DeviceClient } from './device-client';
import { createNativePermissionQaClient } from './native-permission-qa-client';

const ready: DeviceReadiness = {
  platform: 'macos',
  requiresRelaunch: false,
  message: 'Native ready.',
  screenCapture: capability(true),
  accessibility: capability(true),
  microphone: capability(false),
};

it('stages a fresh Mac while delegating real settings actions', async () => {
  const native: DeviceClient = {
    check: vi.fn(async () => ready),
    request: vi.fn(async () => ready),
    openSettings: vi.fn(async () => undefined),
    relaunch: vi.fn(async () => undefined),
  };
  const qa = createNativePermissionQaClient(native);

  expect((await qa.check()).screenCapture.status).toBe('unknown');
  await qa.openSettings('screenCapture');
  expect((await qa.check()).screenCapture.status).toBe('granted');
  expect((await qa.check()).accessibility.status).toBe('unknown');
  await qa.openSettings('accessibility');
  expect((await qa.check()).accessibility.status).toBe('granted');
  expect(native.openSettings).toHaveBeenNthCalledWith(1, 'screenCapture');
  expect(native.openSettings).toHaveBeenNthCalledWith(2, 'accessibility');
});

function capability(required: boolean) {
  return {
    status: 'granted' as const,
    required,
    canRequest: false,
    recovery: 'none' as const,
    message: 'Ready.',
  };
}
