import { afterEach, expect, it, vi } from 'vitest';
import {
  cleanup,
  fireEvent,
  render,
  screen,
  within,
} from '@testing-library/react';
import { App } from './App';
import { createPreviewClient } from './platform/preview-client';
import { LanguageProvider } from './i18n';

afterEach(() => {
  cleanup();
  localStorage.clear();
});

it('keeps the active workspace visible and navigates owner tools', async () => {
  const signOut = vi.fn();
  render(
    <App
      client={createPreviewClient({ workspaceRole: 'owner' })}
      session={{
        user: {
          accountId: '00000000-0000-0000-0000-000000000001',
          displayName: 'Ada Owner',
          email: 'ada@example.com',
        },
        workspaces: [
          {
            workspaceId: '00000000-0000-0000-0000-000000000002',
            name: 'Northstar Robotics',
            role: 'owner',
          },
        ],
        signOut,
        signingOut: false,
      }}
    />,
  );

  const sidebar = screen.getByLabelText('Workspace navigation');
  expect(within(sidebar).getByText('Northstar Robotics')).toBeTruthy();
  expect(within(sidebar).getByText('Owner')).toBeTruthy();
  expect(
    screen.getByRole('button', { name: 'Learn' }).getAttribute('aria-current'),
  ).toBe('page');
  expect(
    screen.queryByRole('heading', { name: 'People with access' }),
  ).toBeNull();

  fireEvent.click(screen.getByRole('button', { name: 'Team' }));
  expect(
    await screen.findByRole('heading', { name: 'People with access' }),
  ).toBeTruthy();

  fireEvent.click(screen.getByRole('button', { name: 'Settings' }));
  expect(screen.getByRole('heading', { name: 'Settings' })).toBeTruthy();
  expect(screen.getByText('ada@example.com')).toBeTruthy();
  fireEvent.click(screen.getByRole('button', { name: 'Sign out' }));
  expect(signOut).toHaveBeenCalledOnce();

  fireEvent.click(screen.getByRole('button', { name: 'Learn' }));
  expect(screen.getByRole('button', { name: 'Start session' })).toBeTruthy();
});

it('does not present owner-only Team navigation to a student', () => {
  render(
    <App
      client={createPreviewClient({ workspaceRole: 'student' })}
      session={{
        user: {
          accountId: '00000000-0000-0000-0000-000000000001',
          displayName: 'Ada Learner',
          email: 'ada@example.com',
        },
        workspaces: [
          {
            workspaceId: '00000000-0000-0000-0000-000000000002',
            name: 'Northstar Robotics',
            role: 'student',
          },
        ],
        signOut: vi.fn(),
        signingOut: false,
      }}
    />,
  );

  expect(screen.queryByRole('button', { name: 'Team' })).toBeNull();
  expect(screen.getByRole('button', { name: 'Settings' })).toBeTruthy();
});

it('switches and persists the complete application interface in Vietnamese', () => {
  render(
    <LanguageProvider initialLocale="en">
      <App
        client={createPreviewClient({ workspaceRole: 'student' })}
        session={{
          user: {
            accountId: '00000000-0000-0000-0000-000000000001',
            displayName: 'Ada Learner',
            email: 'ada@example.com',
          },
          workspaces: [
            {
              workspaceId: '00000000-0000-0000-0000-000000000002',
              name: 'Northstar Robotics',
              role: 'student',
            },
          ],
          signOut: vi.fn(),
          signingOut: false,
        }}
      />
    </LanguageProvider>,
  );

  fireEvent.click(screen.getByRole('button', { name: 'Settings' }));
  fireEvent.change(screen.getByLabelText('App language'), {
    target: { value: 'vi' },
  });

  expect(screen.getByRole('heading', { name: 'Cài đặt' })).toBeTruthy();
  expect(screen.getByRole('button', { name: 'Đăng xuất' })).toBeTruthy();
  expect(screen.getByRole('button', { name: 'Học tập' })).toBeTruthy();
  expect(document.documentElement.lang).toBe('vi');
  expect(localStorage.getItem('tro.app-locale')).toBe('vi');
});
