import type { VoiceStatus } from '@tro/contracts';
import type { VoiceClient } from './voice-client';

const utteranceId = '00000000-0000-0000-0000-000000000010';
const guidanceId = '00000000-0000-0000-0000-000000000011';

export function createPreviewVoice(): VoiceClient {
  let status: VoiceStatus = {
    phase: 'idle',
    revision: 0,
    utteranceId: null,
    guidanceId: null,
    partialTranscript: '',
    finalTranscript: '',
    targetTitle: null,
    message: `Hold ${previewShortcut()} to ask for help.`,
    shortcut: previewShortcut(),
    permissions: {
      microphone: 'granted',
      keyboardMonitoring: 'granted',
      ready: true,
      recovery: '',
    },
  };
  const listeners = new Set<(value: VoiceStatus) => void>();
  const publish = (next: Partial<VoiceStatus>) => {
    status = { ...status, ...next, revision: status.revision + 1 };
    for (const listener of listeners) listener({ ...status });
    return { ...status };
  };
  return {
    status: async () => ({ ...status }),
    enable: async () =>
      publish({
        phase: 'idle',
        message: `Hold ${status.shortcut} to ask for help.`,
        permissions: {
          microphone: 'granted',
          keyboardMonitoring: 'granted',
          ready: true,
          recovery: '',
        },
      }),
    disable: async () =>
      publish({
        phase: 'disabled',
        utteranceId: null,
        guidanceId: null,
        partialTranscript: '',
        finalTranscript: '',
        message: 'Voice guidance is disabled.',
      }),
    executeText: async (instruction) => {
      publish({
        phase: 'dispatching',
        utteranceId,
        guidanceId: null,
        finalTranscript: instruction,
        message: 'Preparing the selected window…',
      });
      publish({
        phase: 'planning',
        message: 'Preparing a simple walkthrough…',
      });
      return publish({
        phase: 'guiding',
        guidanceId,
        targetTitle: 'Preview browser',
        message: 'Follow the cursor in the selected window.',
      });
    },
    cancel: async () =>
      publish({
        phase: 'cancelled',
        message: 'Voice guidance cancelled.',
      }),
    subscribe: async (listener) => {
      listeners.add(listener);
      return () => listeners.delete(listener);
    },
  };
}

function previewShortcut() {
  return navigator.platform.toLowerCase().includes('mac')
    ? 'Command+Control'
    : 'Left Control+Left Alt';
}
