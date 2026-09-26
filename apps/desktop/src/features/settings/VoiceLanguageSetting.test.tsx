import { afterEach, expect, it, vi } from 'vitest';
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
import { VoiceLanguageSetting } from './VoiceLanguageSetting';

afterEach(cleanup);

it('defaults to Vietnamese and persists each closed selection', async () => {
  const client = createPreviewVoice();
  render(<VoiceLanguageSetting client={client} />);

  const select = (await screen.findByLabelText(
    'Transcription language',
  )) as HTMLSelectElement;
  await waitFor(() => expect(select.disabled).toBe(false));
  expect(select.value).toBe('vi');
  expect(
    Array.from(select.options).map((option) => [option.text, option.value]),
  ).toEqual([
    ['Vietnamese', 'vi'],
    ['English', 'en'],
    ['Auto', 'auto'],
  ]);

  fireEvent.change(select, { target: { value: 'en' } });
  await waitFor(() => expect(select.value).toBe('en'));
  expect((await client.status()).transcriptionLanguage).toBe('en');

  fireEvent.change(select, { target: { value: 'auto' } });
  await waitFor(() => expect(select.value).toBe('auto'));
  expect((await client.status()).transcriptionLanguage).toBe('auto');
});

it('retains the last confirmed language when saving fails', async () => {
  const client = createPreviewVoice();
  client.setTranscriptionLanguage = vi.fn().mockRejectedValue(new Error('no'));
  render(<VoiceLanguageSetting client={client} />);

  const select = (await screen.findByLabelText(
    'Transcription language',
  )) as HTMLSelectElement;
  await waitFor(() => expect(select.disabled).toBe(false));
  fireEvent.change(select, { target: { value: 'en' } });

  expect(
    await screen.findByText(
      'Transcription language could not be saved. Try again.',
    ),
  ).toBeTruthy();
  expect(select.value).toBe('vi');
});

it('disables the setting while loading and saving', async () => {
  const client = createPreviewVoice();
  const current = await client.status();
  let resolveStatus: ((status: VoiceStatus) => void) | undefined;
  let resolveSave: ((status: VoiceStatus) => void) | undefined;
  client.status = () =>
    new Promise((resolve) => {
      resolveStatus = resolve;
    });
  client.setTranscriptionLanguage = () =>
    new Promise((resolve) => {
      resolveSave = resolve;
    });
  render(<VoiceLanguageSetting client={client} />);

  const select = screen.getByLabelText(
    'Transcription language',
  ) as HTMLSelectElement;
  expect(select.disabled).toBe(true);
  await act(async () => resolveStatus!(current));
  expect(select.disabled).toBe(false);

  fireEvent.change(select, { target: { value: 'en' } });
  expect(select.disabled).toBe(true);
  expect(screen.getByText('Saving…')).toBeTruthy();
  await act(async () =>
    resolveSave!({ ...current, revision: 1, transcriptionLanguage: 'en' }),
  );
  expect(select.disabled).toBe(false);
  expect(select.value).toBe('en');
});

it('increments preview revisions for each persisted selection', async () => {
  const client = createPreviewVoice();
  const before = await client.status();

  const english = await client.setTranscriptionLanguage('en');
  const automatic = await client.setTranscriptionLanguage('auto');
  const vietnamese = await client.setTranscriptionLanguage('vi');

  expect(english.revision).toBe(before.revision + 1);
  expect(automatic.revision).toBe(english.revision + 1);
  expect(vietnamese.revision).toBe(automatic.revision + 1);
});

it('ignores a status response after unmount', async () => {
  const client = createPreviewVoice();
  let resolveStatus: ((status: VoiceStatus) => void) | undefined;
  client.status = () =>
    new Promise((resolve) => {
      resolveStatus = resolve;
    });
  const view = render(<VoiceLanguageSetting client={client} />);
  view.unmount();

  await act(async () => {
    resolveStatus!({
      phase: 'idle',
      revision: 0,
      utteranceId: null,
      runId: null,
      partialTranscript: '',
      finalTranscript: '',
      queuedInstructions: [],
      targetTitle: null,
      message: 'Ready.',
      shortcut: 'Command+Control',
      transcriptionLanguage: 'vi',
      permissions: {
        microphone: 'granted',
        keyboardMonitoring: 'granted',
        ready: true,
        recovery: '',
      },
      confirmation: null,
      actionsUsed: 0,
    });
  });
});
