import type { TeachingState, TeachingCue } from '@tro/contracts';
export interface GuidanceRequest {
  elementId: string;
  gesture: TeachingCue['gesture'];
  caption: string;
  locale: TeachingCue['locale'];
  destinationId: string | null;
  direction: TeachingCue['direction'];
}
export interface TeachingClient {
  subscribe?(listener: (state: TeachingState) => void): Promise<() => void>;
  planControl?(action: 'pause' | 'resume' | 'confirm'): Promise<TeachingState>;
  ask(question: string, locale: 'en' | 'vi'): Promise<TeachingState>;
  connectProof(): Promise<void>;
  listTargets(): Promise<TeachingState>;
  selectTarget(pid: number, windowId: number): Promise<TeachingState>;
  observe(): Promise<TeachingState>;
  explain(request: GuidanceRequest): Promise<TeachingState>;
  check(cueId: string, label: string, expected: string): Promise<TeachingState>;
}
