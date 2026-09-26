import { afterEach, expect, it } from 'vitest';
import { cleanup, render, screen } from '@testing-library/react';
import { PermissionSettingsGuideView } from './PermissionSettingsGuide';
import { LanguageProvider } from '../../i18n';

afterEach(cleanup);

it('shows click-through macOS drag guidance without interactive controls', () => {
  const { container } = render(
    <PermissionSettingsGuideView
      guide={{ platform: 'macos', target: 'screenCapture' }}
    />,
  );

  expect(screen.getByText('Screen Recording')).toBeTruthy();
  expect(
    screen.getByText('Privacy & Security → Screen & System Audio Recording'),
  ).toBeTruthy();
  expect(
    screen.getByText(/Drag Tro\.app from Applications into the app list/),
  ).toBeTruthy();
  expect(screen.getByText('Find Tro in this list')).toBeTruthy();
  expect(
    container
      .querySelector('.permission-guide-pointer path')
      ?.getAttribute('d'),
  ).toBe('M58 26H10m0 0 13-13M10 26l13 13');
  expect(
    container.querySelector('button, input, select, textarea, a'),
  ).toBeNull();
});

it('shows the shared Windows desktop-app microphone toggle', () => {
  render(
    <PermissionSettingsGuideView
      guide={{ platform: 'windows', target: 'microphone' }}
    />,
  );

  expect(screen.getByText('Privacy & security → Microphone')).toBeTruthy();
  expect(
    screen.getByText(/Let desktop apps access your microphone/),
  ).toBeTruthy();
});

it('renders the auxiliary permission guide in the selected app language', () => {
  render(
    <LanguageProvider initialLocale="vi">
      <PermissionSettingsGuideView
        guide={{ platform: 'macos', target: 'accessibility' }}
      />
    </LanguageProvider>,
  );

  expect(screen.getByLabelText('Hướng dẫn cấp quyền Tro')).toBeTruthy();
  expect(screen.getByRole('heading', { name: 'Trợ năng' })).toBeTruthy();
  expect(screen.getByText('Tìm Tro trong danh sách này')).toBeTruthy();
  expect(
    screen.getByText('Bạn tự thực hiện mọi thay đổi. Tro chỉ chỉ ra vị trí.'),
  ).toBeTruthy();
});
