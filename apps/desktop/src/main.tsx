import { OverlayWindow } from './features/teaching/OverlayWindow';
import { AuthGate } from './features/auth/AuthGate';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { App } from './App';
import { createPreviewClient } from './platform/preview-client';
import type { PreviewAuthScenario } from './platform/preview-client';
import type { PreviewWorkspaceRole } from './platform/preview-client';
import type { PreviewPermissionScenario } from './platform/preview-client';
import { DeviceOnboardingGate } from './features/onboarding/PermissionOnboarding';
import { PermissionSettingsGuideWindow } from './features/onboarding/PermissionSettingsGuide';
import { tauriClient } from './platform/tauri-client';
import { createNativePermissionQaClient } from './platform/native-permission-qa-client';
import './styles.css';
import { VoiceHud } from './features/voice/VoiceHud';
import { LanguageProvider } from './i18n';
// Preview is explicitly requested by the dev:ui command, never a bridge fallback.
const search = new URLSearchParams(location.search);
if (search.has('voiceHud') || search.has('overlay'))
  document.documentElement.classList.add('transparent-window');
const previewScenario = search.get('auth');
const previewWorkspaceRole = search.get('workspaceRole');
const previewPermissions = search.get('permissions');
const nativePermissionQa =
  import.meta.env.DEV && search.has('nativePermissionQa');
const nativePermissionQaClient = nativePermissionQa
  ? {
      ...createPreviewClient({
        authScenario: 'authenticated',
        permissionScenario: 'ready',
      }),
      device: createNativePermissionQaClient(tauriClient.device),
    }
  : null;
const client =
  import.meta.env.VITE_TRO_PREVIEW === '1'
    ? createPreviewClient({
        authScenario: isPreviewAuthScenario(previewScenario)
          ? previewScenario
          : undefined,
        workspaceRole: isPreviewWorkspaceRole(previewWorkspaceRole)
          ? previewWorkspaceRole
          : undefined,
        permissionScenario: isPreviewPermissionScenario(previewPermissions)
          ? previewPermissions
          : undefined,
      })
    : tauriClient;
ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <LanguageProvider>
      {search.has('voiceHud') ? (
        <VoiceHud />
      ) : search.has('permissionGuide') ? (
        <PermissionSettingsGuideWindow />
      ) : search.has('overlay') ? (
        <OverlayWindow />
      ) : nativePermissionQa ? (
        <DeviceOnboardingGate client={nativePermissionQaClient!.device}>
          <AuthGate auth={nativePermissionQaClient!.auth} preview>
            {(session) => (
              <App client={nativePermissionQaClient!} session={session} />
            )}
          </AuthGate>
        </DeviceOnboardingGate>
      ) : (
        <AuthGate auth={client.auth} preview={client.preview}>
          {(session) => (
            <DeviceOnboardingGate
              client={client.device}
              skipIntro={
                client.preview &&
                !isPreviewPermissionScenario(previewPermissions)
              }
            >
              <App client={client} session={session} />
            </DeviceOnboardingGate>
          )}
        </AuthGate>
      )}
    </LanguageProvider>
  </React.StrictMode>,
);

function isPreviewAuthScenario(
  value: string | null,
): value is PreviewAuthScenario {
  return [
    'signedOut',
    'checking',
    'signingIn',
    'authenticated',
    'membershipRequired',
    'offline',
    'error',
  ].includes(value ?? '');
}

function isPreviewWorkspaceRole(
  value: string | null,
): value is PreviewWorkspaceRole {
  return ['owner', 'teacher', 'student'].includes(value ?? '');
}

function isPreviewPermissionScenario(
  value: string | null,
): value is PreviewPermissionScenario {
  return [
    'fresh',
    'ready',
    'screenDenied',
    'relaunchRequired',
    'microphoneUnavailable',
    'windowsMicrophoneDenied',
  ].includes(value ?? '');
}
