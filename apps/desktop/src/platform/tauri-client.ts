import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { parseStatus } from '@tro/contracts';
import type { DesktopClient } from './desktop-client';
async function call(command: string, args?: Record<string, unknown>) {
  return parseStatus(await invoke(command, args));
}
export const tauriClient: DesktopClient = {
  preview: false,
  status: () => call('runtime_status'),
  start: () => call('runtime_start'),
  stop: () => call('runtime_stop'),
  health: () => call('runtime_health'),
  restart: () => call('runtime_restart'),
  selectAccount: (profile) => call('account_select', { profile }),
  subscribe: (listener) =>
    listen('runtime-status', (event) => listener(parseStatus(event.payload))),
};
