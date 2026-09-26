/* Generated. Do not edit. */

/**
 * This interface was referenced by `DeviceReadiness`'s JSON-Schema
 * via the `definition` "DevicePlatform".
 */
export type DevicePlatform = 'macos' | 'windows' | 'unsupported';
/**
 * This interface was referenced by `DeviceReadiness`'s JSON-Schema
 * via the `definition` "CapabilityStatus".
 */
export type CapabilityStatus = 'granted' | 'available' | 'notDetermined' | 'denied' | 'unavailable' | 'unknown';
/**
 * This interface was referenced by `DeviceReadiness`'s JSON-Schema
 * via the `definition` "RecoveryAction".
 */
export type RecoveryAction = 'none' | 'request' | 'manualSettings' | 'recheck' | 'relaunch';
/**
 * This interface was referenced by `DeviceReadiness`'s JSON-Schema
 * via the `definition` "PermissionRequestKind".
 */
export type PermissionRequestKind = 'microphone';
/**
 * This interface was referenced by `DeviceReadiness`'s JSON-Schema
 * via the `definition` "PermissionSettingsTarget".
 */
export type PermissionSettingsTarget = 'screenCapture' | 'accessibility' | 'microphone';

export interface DeviceReadiness {
  platform: DevicePlatform;
  requiresRelaunch: boolean;
  message: string;
  screenCapture: DeviceCapability;
  accessibility: DeviceCapability;
  microphone: DeviceCapability;
}
/**
 * This interface was referenced by `DeviceReadiness`'s JSON-Schema
 * via the `definition` "DeviceCapability".
 */
export interface DeviceCapability {
  status: CapabilityStatus;
  required: boolean;
  canRequest: boolean;
  recovery: RecoveryAction;
  message: string;
}
/**
 * This interface was referenced by `DeviceReadiness`'s JSON-Schema
 * via the `definition` "PermissionSettingsGuide".
 */
export interface PermissionSettingsGuide {
  platform: DevicePlatform;
  target: PermissionSettingsTarget;
}
