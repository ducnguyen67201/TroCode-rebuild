import type { TeachingClient } from './teaching-client';
import type { RuntimeStatus } from '@tro/contracts';
export type Profile = 'teacher' | 'student-a' | 'student-b';
export interface DesktopClient {
  readonly preview: boolean;
  readonly teaching?: TeachingClient;
  status(): Promise<RuntimeStatus>;
  start(): Promise<RuntimeStatus>;
  stop(): Promise<RuntimeStatus>;
  health(): Promise<RuntimeStatus>;
  restart(): Promise<RuntimeStatus>;
  selectAccount(profile: Profile): Promise<RuntimeStatus>;
  subscribe(listener: (status: RuntimeStatus) => void): Promise<() => void>;
}
