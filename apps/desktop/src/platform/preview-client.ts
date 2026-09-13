import type { RuntimeStatus } from '@tro/contracts';
import type { DesktopClient } from './desktop-client';
export function createPreviewClient(): DesktopClient {
  let status: RuntimeStatus = {
    state: 'stopped',
    generationId: null,
    revision: 0,
    message: 'Ready to preview a diagnostic session.',
  };
  const listeners = new Set<(value: RuntimeStatus) => void>();
  async function change(state: RuntimeStatus['state']) {
    status = {
      state,
      revision: status.revision + 1,
      generationId: state === 'running' ? 'preview' : null,
      message:
        state === 'running'
          ? 'Simulated runtime is running.'
          : 'Simulated runtime stopped.',
    };
    for (const listener of listeners) listener(status);
    return status;
  }
  return {
    preview: true,
    status: async () => status,
    start: () => change('running'),
    stop: () => change('stopped'),
    health: async () => status,
    restart: () => change('running'),
    selectAccount: () => change('stopped'),
    subscribe: async (listener) => {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
  };
}
