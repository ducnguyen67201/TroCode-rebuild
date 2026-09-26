import type {
  DeviceReadiness,
  PermissionRequestKind,
  PermissionSettingsTarget,
} from '@tro/contracts';

export interface DeviceClient {
  check(): Promise<DeviceReadiness>;
  request(kind: PermissionRequestKind): Promise<DeviceReadiness>;
  openSettings(target: PermissionSettingsTarget): Promise<void>;
  relaunch(): Promise<void>;
}
