import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { parseStatus, parseTeaching } from '@tro/contracts';
import type { DesktopClient } from './desktop-client';
async function call(command: string, args?: Record<string, unknown>) {
  return parseStatus(await invoke(command, args));
}
export const tauriClient: DesktopClient = {
  preview: false,
  teaching: {
    subscribe: (listener) =>
      listen('teaching-state', (event) =>
        listener(parseTeaching(event.payload)),
      ),
    planControl: (action) => teaching('planControl', { action }),
    permissions: async () => {
      const value = await invoke<{
        screenCapture: boolean;
        accessibility: boolean;
        message: string;
      }>('observation_permissions', { request: true });
      if (typeof value.message !== 'string')
        throw new Error('Invalid permission response');
      return value.message;
    },
    ask: (question, locale) => teaching('ask', { question, locale }),
    connectProof: async () => {
      await call('proof_connect');
    },
    listTargets: () => teaching('listTargets', {}),
    selectTarget: (pid, windowId) =>
      teaching('selectTarget', { pid, windowId }),
    observe: () => teaching('observe', {}),
    explain: (request) => teaching('explain', { ...request }),
    check: (cueId, label, expected) =>
      teaching('check', { cueId, label, expected }),
  },
  status: () => call('runtime_status'),
  start: () => call('runtime_start'),
  stop: () => call('runtime_stop'),
  health: () => call('runtime_health'),
  restart: () => call('runtime_restart'),
  selectAccount: (profile) => call('account_select', { profile }),
  subscribe: (listener) =>
    listen('runtime-status', (event) => listener(parseStatus(event.payload))),
};

async function teaching(kind: string, payload: Record<string, unknown>) {
  return parseTeaching(await invoke('teaching_request', { kind, payload }));
}
