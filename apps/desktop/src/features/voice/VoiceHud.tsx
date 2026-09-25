import { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';

interface HudStatus {
  phase: string;
  revision: number;
  message: string;
}

export function VoiceHud() {
  const [status, setStatus] = useState<HudStatus | null>(null);
  useEffect(() => {
    document.body.classList.add('voice-hud-page');
    let disposed = false;
    let revision = -1;
    let cleanup: (() => void) | undefined;
    void listen('voice-hud-status', (event) => {
      if (disposed || !event.payload || typeof event.payload !== 'object')
        return;
      const value = event.payload as Partial<HudStatus>;
      if (
        typeof value.phase !== 'string' ||
        typeof value.message !== 'string' ||
        typeof value.revision !== 'number' ||
        value.revision <= revision
      )
        return;
      revision = value.revision;
      setStatus({ phase: value.phase, message: value.message, revision });
    }).then((unlisten) => {
      if (disposed) unlisten();
      else cleanup = unlisten;
    });
    return () => {
      disposed = true;
      cleanup?.();
    };
  }, []);
  if (!status) return null;
  return (
    <div
      className={`voice-hud voice-hud-${status.phase}`}
      role="status"
      aria-live="polite"
    >
      <span aria-hidden="true" />
      {status.message}
    </div>
  );
}
