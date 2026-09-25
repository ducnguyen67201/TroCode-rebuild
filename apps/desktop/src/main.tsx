import { OverlayWindow } from './features/teaching/OverlayWindow';
import { AuthGate } from './features/auth/AuthGate';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { App } from './App';
import { createPreviewClient } from './platform/preview-client';
import type { PreviewAuthScenario } from './platform/preview-client';
import type { PreviewWorkspaceRole } from './platform/preview-client';
import { tauriClient } from './platform/tauri-client';
import './styles.css';
import { VoiceHud } from './features/voice/VoiceHud';
// Preview is explicitly requested by the dev:ui command, never a bridge fallback.
const search = new URLSearchParams(location.search);
const previewScenario = search.get('auth');
const previewWorkspaceRole = search.get('workspaceRole');
const client =
  import.meta.env.VITE_TRO_PREVIEW === '1'
    ? createPreviewClient({
        authScenario: isPreviewAuthScenario(previewScenario)
          ? previewScenario
          : undefined,
        workspaceRole: isPreviewWorkspaceRole(previewWorkspaceRole)
          ? previewWorkspaceRole
          : undefined,
      })
    : tauriClient;
ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    {search.has('voiceHud') ? (
      <VoiceHud />
    ) : search.has('overlay') ? (
      <OverlayWindow />
    ) : (
      <AuthGate auth={client.auth} preview={client.preview}>
        {(session) => <App client={client} session={session} />}
      </AuthGate>
    )}
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
