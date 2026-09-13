import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { parseTeaching } from '@tro/contracts';
import type { TeachingCue } from '@tro/contracts';
import { TeachingOverlay } from './TeachingOverlay';
interface Presentation {
  cue: TeachingCue;
  origin: { x: number; y: number };
  units: number;
}
function parse(value: unknown): Presentation | null {
  if (!value || typeof value !== 'object') return null;
  const data = value as Record<string, unknown>;
  const state = parseTeaching(data.state);
  const origin = data.origin as { x?: unknown; y?: unknown } | undefined;
  if (
    !state.cue ||
    state.cue.expires_at <= Date.now() / 1000 ||
    !origin ||
    typeof origin.x !== 'number' ||
    !Number.isFinite(origin.x) ||
    typeof origin.y !== 'number' ||
    !Number.isFinite(origin.y) ||
    typeof data.units !== 'number' ||
    !Number.isFinite(data.units) ||
    data.units <= 0
  )
    return null;
  return {
    cue: state.cue,
    origin: { x: origin.x, y: origin.y },
    units: data.units,
  };
}
export function OverlayWindow() {
  const [presentation, setPresentation] = useState<Presentation | null>(null);
  useEffect(() => {
    document.body.classList.add('overlay-page');
    let disposed = false;
    let cleanup: (() => void) | undefined;
    let expiry: ReturnType<typeof setTimeout>;
    function update(value: unknown) {
      if (disposed) return;
      try {
        const next = parse(value);
        setPresentation(next);
        clearTimeout(expiry);
        if (next)
          expiry = setTimeout(
            () => setPresentation(null),
            Math.max(0, next.cue.expires_at * 1000 - Date.now()),
          );
      } catch {
        setPresentation(null);
      }
    }
    void listen('teaching-cue', (event) => update(event.payload)).then(
      (unlisten) => {
        if (disposed) unlisten();
        else {
          cleanup = unlisten;
          void invoke('overlay_current').then(update);
        }
      },
    );
    return () => {
      disposed = true;
      cleanup?.();
      clearTimeout(expiry);
    };
  }, []);
  if (!presentation) return null;
  return (
    <div
      style={{
        position: 'absolute',
        inset: 0,
        transformOrigin: 'top left',
        transform: `scale(${1 / presentation.units})`,
        width: `${presentation.units * 100}%`,
        height: `${presentation.units * 100}%`,
      }}
    >
      <TeachingOverlay cue={presentation.cue} origin={presentation.origin} />
    </div>
  );
}
