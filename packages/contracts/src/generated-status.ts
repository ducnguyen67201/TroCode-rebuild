/* Generated. Do not edit. */

export interface RuntimeStatus {
  state: 'starting' | 'ready' | 'running' | 'stopping' | 'stopped' | 'failed';
  generationId: string | null;
  revision: number;
  message: string;
}
