import type { VoiceStatus } from '@tro/contracts';

export interface VoiceClient {
  status(): Promise<VoiceStatus>;
  enable(): Promise<VoiceStatus>;
  disable(): Promise<VoiceStatus>;
  executeText(instruction: string): Promise<VoiceStatus>;
  cancel(): Promise<VoiceStatus>;
  decide(
    runId: string,
    confirmationId: string,
    approve: boolean,
  ): Promise<VoiceStatus>;
  subscribe(
    listener: (status: VoiceStatus) => void,
    onBoundaryError?: () => void,
  ): Promise<() => void>;
}
