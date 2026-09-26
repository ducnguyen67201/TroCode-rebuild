import { afterEach, expect, it, vi } from 'vitest';
import {
  cleanup,
  fireEvent,
  render,
  screen,
  waitFor,
} from '@testing-library/react';
import { createPreviewClient } from '../../platform/preview-client';
import { DeviceOnboardingGate } from './PermissionOnboarding';
import { ONBOARDING_MARKER_KEY } from './onboarding-state';
import { LanguageProvider } from '../../i18n';

afterEach(() => {
  cleanup();
  localStorage.clear();
});

it('guides a first run without controlling the real pointer', async () => {
  const client = createPreviewClient({ permissionScenario: 'fresh' });
  render(
    <DeviceOnboardingGate client={client.device}>
      <p>Learn surface</p>
    </DeviceOnboardingGate>,
  );

  const setup = await screen.findByRole('button', {
    name: 'Open Screen Recording settings',
  });
  expect(setup.getAttribute('aria-describedby')).toBeTruthy();
  expect(
    document
      .querySelector('.guided-action-cue svg')
      ?.getAttribute('aria-hidden'),
  ).toBe('true');

  fireEvent.click(setup);
  expect(
    await screen.findByText(
      /Drag Tro\.app from Applications into the app list/,
    ),
  ).toBeTruthy();
  expect(
    screen.getByRole('button', { name: 'I changed it — recheck' }),
  ).toBeTruthy();
  expect(document.querySelector('.guided-action-cue')).toBeNull();
  fireEvent.click(
    screen.getByRole('button', { name: 'I changed it — recheck' }),
  );
  fireEvent.click(
    await screen.findByRole('button', {
      name: 'Open Accessibility settings',
    }),
  );
  fireEvent.click(
    await screen.findByRole('button', { name: 'I changed it — recheck' }),
  );
  fireEvent.click(
    await screen.findByRole('button', { name: 'Use text instead' }),
  );
  fireEvent.click(
    await screen.findByRole('button', { name: 'Continue to Learn' }),
  );

  expect(await screen.findByText('Learn surface')).toBeTruthy();
  expect(JSON.parse(localStorage.getItem(ONBOARDING_MARKER_KEY)!)).toEqual({
    version: 1,
    microphoneChoice: 'text',
  });
});

it('reopens for revoked screen access despite a completion marker', async () => {
  localStorage.setItem(
    ONBOARDING_MARKER_KEY,
    JSON.stringify({ version: 1, microphoneChoice: 'text' }),
  );
  const client = createPreviewClient({ permissionScenario: 'screenDenied' });
  render(
    <DeviceOnboardingGate client={client.device}>
      <p>Learn surface</p>
    </DeviceOnboardingGate>,
  );

  expect((await screen.findAllByText(/Needs attention/)).length).toBe(2);
  expect(screen.queryByText('Learn surface')).toBeNull();
  expect(
    screen.getByRole('button', { name: 'Open Screen Recording settings' }),
  ).toBeTruthy();
});

it('opens each macOS settings page in sequence and fences duplicate opens', async () => {
  const client = createPreviewClient({ permissionScenario: 'fresh' });
  const openSettings = vi.spyOn(client.device, 'openSettings');
  render(
    <DeviceOnboardingGate client={client.device}>
      <p>Learn surface</p>
    </DeviceOnboardingGate>,
  );
  const button = await screen.findByRole('button', {
    name: 'Open Screen Recording settings',
  });
  fireEvent.click(button);
  fireEvent.click(button);
  await waitFor(() => expect(openSettings).toHaveBeenCalledTimes(1));
  expect(openSettings).toHaveBeenLastCalledWith('screenCapture');

  fireEvent.click(
    await screen.findByRole('button', { name: 'I changed it — recheck' }),
  );
  fireEvent.click(
    await screen.findByRole('button', {
      name: 'Open Accessibility settings',
    }),
  );
  await waitFor(() => expect(openSettings).toHaveBeenCalledTimes(2));
  expect(openSettings).toHaveBeenLastCalledWith('accessibility');
});

it('routes Windows microphone recovery to its exact settings page', async () => {
  const client = createPreviewClient({
    permissionScenario: 'windowsMicrophoneDenied',
  });
  const openSettings = vi.spyOn(client.device, 'openSettings');
  render(
    <DeviceOnboardingGate client={client.device}>
      <p>Learn surface</p>
    </DeviceOnboardingGate>,
  );

  fireEvent.click(
    await screen.findByRole('button', { name: 'Open Microphone settings' }),
  );
  await waitFor(() => expect(openSettings).toHaveBeenCalledWith('microphone'));
  expect(
    await screen.findByText(/Let desktop apps access your microphone/),
  ).toBeTruthy();
});

it('localizes the complete device setup surface in Vietnamese', async () => {
  const client = createPreviewClient({ permissionScenario: 'fresh' });
  render(
    <LanguageProvider initialLocale="vi">
      <DeviceOnboardingGate client={client.device}>
        <p>Learn surface</p>
      </DeviceOnboardingGate>
    </LanguageProvider>,
  );

  expect(
    await screen.findByRole('heading', { name: 'Thiết lập thiết bị này' }),
  ).toBeTruthy();
  expect(screen.getByText('Màn hình và điều khiển')).toBeTruthy();
  expect(
    screen.getByText(
      (_, element) =>
        element?.tagName === 'STRONG' &&
        element.textContent === 'Micrô: Chưa kiểm tra',
    ),
  ).toBeTruthy();

  fireEvent.click(
    screen.getByRole('button', { name: 'Mở cài đặt Ghi màn hình' }),
  );

  expect(await screen.findByText('Cài đặt Ghi màn hình đang mở')).toBeTruthy();
  expect(
    screen.getByText(/Bạn cũng có thể kéo Tro\.app từ thư mục Ứng dụng/),
  ).toBeTruthy();
  expect(
    screen.getByText(
      'Hướng dẫn nổi của Tro chỉ trỏ vị trí. Bạn tự thực hiện mọi thay đổi.',
    ),
  ).toBeTruthy();
});
