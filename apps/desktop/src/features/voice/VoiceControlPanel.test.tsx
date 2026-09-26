import React from 'react';
import { afterEach, expect, it } from 'vitest';
import {
  act,
  cleanup,
  fireEvent,
  render,
  screen,
  waitFor,
} from '@testing-library/react';
import type { VoiceStatus } from '@tro/contracts';
import { createPreviewVoice } from '../../platform/preview-voice';
import { VoiceControlPanel } from './VoiceControlPanel';
import { LanguageProvider } from '../../i18n';

afterEach(cleanup);

it('auto-enables voice, supports consequential confirmation, and filters stale state', async () => {
  const client = createPreviewVoice();
  let publish: ((status: VoiceStatus) => void) | undefined;
  client.subscribe = async (listener) => {
    publish = listener;
    return () => {};
  };
  render(<VoiceControlPanel client={client} />);
  await screen.findByText(/Hold .* to speak/);
  fireEvent.change(screen.getByLabelText('Type instead of speaking'), {
    target: { value: 'send the message' },
  });
  fireEvent.click(screen.getByText('Run instruction'));
  await screen.findByText('Approval required');
  fireEvent.click(screen.getByText('Reject action'));
  await screen.findByText('Instruction cancelled.');
  const current = await client.status();
  await act(async () =>
    publish!({ ...current, revision: 100, message: 'Fresh voice state' }),
  );
  expect(screen.getByText('Fresh voice state')).toBeTruthy();
  await act(async () =>
    publish!({ ...current, revision: 99, message: 'Stale voice state' }),
  );
  expect(screen.queryByText('Stale voice state')).toBeNull();
});

it('keeps text fallback available after automatic voice startup', async () => {
  const client = createPreviewVoice();
  render(<VoiceControlPanel client={client} />);
  const input = await screen.findByLabelText('Type instead of speaking');
  fireEvent.change(input, { target: { value: 'open settings' } });
  fireEvent.click(screen.getByText('Run instruction'));
  await waitFor(() =>
    expect(screen.getByText('Instruction completed.')).toBeTruthy(),
  );
});

it('shows queued follow-ups in FIFO order while an instruction executes', async () => {
  const client = createPreviewVoice();
  let publish: ((status: VoiceStatus) => void) | undefined;
  client.subscribe = async (listener) => {
    publish = listener;
    return () => {};
  };
  render(<VoiceControlPanel client={client} />);
  await screen.findByText(/Hold .* to speak/);
  const current = await client.status();
  await act(async () =>
    publish!({
      ...current,
      revision: 10,
      phase: 'executing',
      runId: '00000000-0000-0000-0000-000000000011',
      finalTranscript: 'Third instruction',
      queuedInstructions: ['Second instruction', 'Third instruction'],
      message: 'Working on the current instruction. 2 follow-ups queued.',
    }),
  );

  expect(screen.getByText('Follow-up queue')).toBeTruthy();
  expect(screen.getByLabelText('2 queued')).toBeTruthy();
  expect(
    screen.getAllByRole('listitem').map((item) => item.textContent),
  ).toEqual(['Second instruction', 'Third instruction']);
});

it('localizes voice chrome without changing the transcript', async () => {
  const client = createPreviewVoice();
  render(
    <LanguageProvider initialLocale="vi">
      <VoiceControlPanel client={client} />
    </LanguageProvider>,
  );

  await screen.findByText('Giữ hai phím. Nói. Thả ra.');
  fireEvent.change(screen.getByLabelText('Nhập văn bản thay vì nói'), {
    target: { value: 'open settings' },
  });
  fireEvent.click(screen.getByText('Chạy chỉ dẫn'));

  await screen.findByText('open settings');
  expect(screen.getAllByText('Hoàn tất')).toHaveLength(2);
});
