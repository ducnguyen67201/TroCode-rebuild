/* Generated. Do not edit. */

export type PermissionState = 'unknown' | 'notNeeded' | 'prompt' | 'granted' | 'denied' | 'unavailable';

export interface VoiceStatus {
  phase:
    | 'disabled'
    | 'idle'
    | 'listening'
    | 'transcribing'
    | 'dispatching'
    | 'executing'
    | 'confirmation'
    | 'completed'
    | 'cancelled'
    | 'failed';
  revision: number;
  utteranceId: string | null;
  runId: string | null;
  partialTranscript: string;
  finalTranscript: string;
  targetTitle: string | null;
  message: string;
  shortcut: 'Command+Control' | 'Left Control+Left Alt' | 'Unavailable';
  permissions: VoicePermissions;
  confirmation: VoiceConfirmation | null;
  actionsUsed: number;
}
export interface VoicePermissions {
  microphone: PermissionState;
  keyboardMonitoring: PermissionState;
  ready: boolean;
  recovery: string;
}
export interface VoiceConfirmation {
  confirmationId: string;
  summary: string;
}
