import { OverlayWindow } from './features/teaching/OverlayWindow';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { App } from './App';
import { createPreviewClient } from './platform/preview-client';
import { tauriClient } from './platform/tauri-client';
import './styles.css';
// Preview is explicitly requested by the dev:ui command, never a bridge fallback.
const client =
  import.meta.env.VITE_TRO_PREVIEW === '1'
    ? createPreviewClient()
    : tauriClient;
ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    {new URLSearchParams(location.search).has('overlay') ? (
      <OverlayWindow />
    ) : (
      <App client={client} />
    )}
  </React.StrictMode>,
);
