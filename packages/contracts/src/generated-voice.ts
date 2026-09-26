/* Generated. Do not edit. */

export type PermissionState = 'unknown' | 'notNeeded' | 'prompt' | 'granted' | 'denied' | 'unavailable';

export interface VoiceStatus {
  phase:
    | 'disabled'
    | 'idle'
    | 'listening'
    | 'transcribing'
    | 'dispatching'
    | 'planning'
    | 'guiding'
    | 'completed'
    | 'cancelled'
    | 'failed';
  revision: number;
  utteranceId: string | null;
  guidanceId: string | null;
  partialTranscript: string;
  finalTranscript: string;
  targetTitle: string | null;
  message: string;
  shortcut: 'Command+Control' | 'Left Control+Left Alt' | 'Unavailable';
  permissions: VoicePermissions;
}
export interface VoicePermissions {
  microphone: PermissionState;
  keyboardMonitoring: PermissionState;
  ready: boolean;
  recovery: string;
}
