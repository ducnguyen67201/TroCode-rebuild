import React from 'react';
import { afterEach, expect, it, vi } from 'vitest';
import {
  act,
  cleanup,
  fireEvent,
  render,
  screen,
  waitFor,
} from '@testing-library/react';
import type { TeachingState } from '@tro/contracts';
import { createPreviewClient } from '../../platform/preview-client';
import { TeachingPanel } from './TeachingPanel';
import { LanguageProvider } from '../../i18n';

afterEach(cleanup);

it('shows prepared steps, applies only fresh session updates, and clears on stop', async () => {
  const client = createPreviewClient();
  let publish: ((state: TeachingState) => void) | undefined;
  client.teaching!.subscribe = async (listener) => {
    publish = listener;
    return () => {};
  };
  render(<TeachingPanel client={client} />);
  fireEvent.click(screen.getByText('Find windows'));
  await waitFor(() =>
    expect(screen.getByText('Simulated practice window')).toBeTruthy(),
  );
  fireEvent.change(screen.getByLabelText('Practice window'), {
    target: { value: '1:1' },
  });
  await waitFor(() =>
    expect((screen.getByText('Observe') as HTMLButtonElement).disabled).toBe(
      false,
    ),
  );
  fireEvent.change(screen.getByLabelText('What would you like help with?'), {
    target: { value: 'Teach me' },
  });
  fireEvent.click(screen.getByText('Plan guidance / replan'));
  await screen.findByText('Your steps');
  fireEvent.click(screen.getByText('Pause guidance'));
  await screen.findByText('Resume guidance');
  fireEvent.click(screen.getByText('Resume guidance'));
  await screen.findByText('Continue — I’m ready');
  const current = await client.teaching!.observe();
  await act(async () =>
    publish!({
      ...current,
      revision: 100,
      journey: { ...current.journey!, message: 'Fresh progress' },
    }),
  );
  expect(screen.getByText('Fresh progress')).toBeTruthy();
  await act(async () =>
    publish!({
      ...current,
      revision: 99,
      journey: { ...current.journey!, message: 'Old progress' },
    }),
  );
  expect(screen.queryByText('Old progress')).toBeNull();
  await act(async () =>
    publish!({
      ...current,
      session_id: crypto.randomUUID(),
      revision: 101,
      journey: { ...current.journey!, message: 'Wrong session' },
    }),
  );
  expect(screen.queryByText('Wrong session')).toBeNull();
  fireEvent.click(screen.getByText('Stop guidance'));
  await act(async () => publish!({ ...current, revision: 102 }));
  expect(screen.queryByText('Your steps')).toBeNull();
});

it('shows bounded recovery guidance from runtime readiness', async () => {
  const client = createPreviewClient();
  const find = client.teaching!.listTargets;
  client.teaching!.listTargets = async () => ({
    ...(await find()),
    readiness: {
      observation: 'unknown',
      screen: 'unknown',
      accessibility: 'unknown',
      model: 'unconfigured',
      reason: 'connect_model',
    },
  });
  render(<TeachingPanel client={client} />);
  expect(screen.getByDisplayValue(/Increase the counter once/)).toBeTruthy();
  fireEvent.click(screen.getByText('Find windows'));
  await screen.findByText(/Connect your proof account first/);
});

it('keeps Vietnamese app copy independent from the lesson language', async () => {
  const client = createPreviewClient();
  const ask = vi.fn(client.teaching!.ask);
  client.teaching!.ask = ask;
  render(
    <LanguageProvider initialLocale="vi">
      <TeachingPanel client={client} />
    </LanguageProvider>,
  );

  fireEvent.click(screen.getByText('Tìm cửa sổ'));
  await screen.findByText('Simulated practice window');
  fireEvent.change(screen.getByLabelText('Cửa sổ thực hành'), {
    target: { value: '1:1' },
  });
  await waitFor(() =>
    expect((screen.getByText('Quan sát') as HTMLButtonElement).disabled).toBe(
      false,
    ),
  );
  fireEvent.click(screen.getByText('Lập / lập lại hướng dẫn'));

  await waitFor(() => expect(ask).toHaveBeenCalled());
  expect(ask.mock.calls[0]?.[1]).toBe('en');
});
