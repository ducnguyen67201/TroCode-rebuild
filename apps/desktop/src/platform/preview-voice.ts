import type { VoiceStatus } from '@tro/contracts';
import type { VoiceClient } from './voice-client';

const utteranceId = '00000000-0000-0000-0000-000000000010';
const runId = '00000000-0000-0000-0000-000000000011';
const confirmationId = '00000000-0000-0000-0000-000000000012';

export function createPreviewVoice(): VoiceClient {
  let status: VoiceStatus = {
    phase: 'idle',
    revision: 0,
    utteranceId: null,
    runId: null,
    partialTranscript: '',
    finalTranscript: '',
    queuedInstructions: [],
    targetTitle: null,
    message: `Hold ${previewShortcut()} to speak.`,
    shortcut: previewShortcut(),
    transcriptionLanguage: 'vi',
    permissions: {
      microphone: 'granted',
      keyboardMonitoring: 'granted',
      ready: true,
      recovery: '',
    },
    confirmation: null,
    actionsUsed: 0,
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
        message: `Hold ${status.shortcut} to speak.`,
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
        runId: null,
        partialTranscript: '',
        finalTranscript: '',
        queuedInstructions: [],
        confirmation: null,
        message: 'Voice control is disabled.',
      }),
    setTranscriptionLanguage: async (transcriptionLanguage) =>
      publish({ transcriptionLanguage }),
    executeText: async (instruction) => {
      publish({
        phase: 'dispatching',
        utteranceId,
        finalTranscript: instruction,
        message: 'Preparing the selected window…',
      });
      if (/send|delete|purchase|upload/i.test(instruction)) {
        return publish({
          phase: 'confirmation',
          runId,
          targetTitle: 'Preview browser',
          message: 'Approval is required before this action.',
          confirmation: {
            confirmationId,
            summary: 'Perform a consequential preview action',
          },
        });
      }
      publish({
        phase: 'executing',
        runId,
        targetTitle: 'Preview browser',
        message: 'Working in the selected window…',
      });
      return publish({
        phase: 'completed',
        message: 'Instruction completed.',
        actionsUsed: 1,
      });
    },
    cancel: async () =>
      publish({
        phase: 'cancelled',
        confirmation: null,
        message: 'Voice instruction cancelled.',
      }),
    decide: async (_runId, _confirmationId, approve) =>
      approve
        ? publish({
            phase: 'completed',
            confirmation: null,
            message: 'Instruction completed.',
            actionsUsed: 1,
          })
        : publish({
            phase: 'cancelled',
            confirmation: null,
            message: 'Instruction cancelled.',
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
