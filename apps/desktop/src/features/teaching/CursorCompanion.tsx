import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useRef } from 'react';
import { InstructorPointer } from './InstructorPointer';

interface PointerPosition {
  x: number;
  y: number;
  width: number;
  height: number;
}

function parsePosition(value: unknown): PointerPosition | null {
  if (!value || typeof value !== 'object') return null;
  const point = value as Record<string, unknown>;
  const { x, y, width, height } = point;
  if (
    ![x, y, width, height].every(
      (entry) => typeof entry === 'number' && Number.isFinite(entry),
    )
  )
    return null;
  const position = { x, y, width, height } as PointerPosition;
  return position.width > 0 && position.height > 0 ? position : null;
}

/** Native-window adapter. It batches pointer events into one transform per frame. */
export function CursorCompanion() {
  const pointer = useRef<HTMLDivElement>(null);

  useEffect(() => {
    let disposed = false;
    let unlisten: (() => void) | undefined;
    let frame = 0;
    let pending: PointerPosition | null = null;

    function update(value: unknown) {
      const next = parsePosition(value);
      if (!next || disposed) return;
      pending = next;
      if (frame) return;
      frame = requestAnimationFrame(() => {
        frame = 0;
        const node = pointer.current;
        const position = pending;
        if (!node || !position) return;
        const size = 28;
        const gap = 10;
        const left =
          position.x + gap + size <= position.width
            ? position.x + gap
            : position.x - gap - size;
        const top =
          position.y + gap + size <= position.height
            ? position.y + gap
            : position.y - gap - size;
        node.style.transform = `translate3d(${Math.max(0, left)}px, ${Math.max(0, top)}px, 0)`;
        node.style.opacity = '1';
      });
    }

    void listen('cursor-companion-position', (event) => update(event.payload)).then(
      (cleanup) => {
        if (disposed) cleanup();
        else {
          unlisten = cleanup;
          void invoke('cursor_companion_current').then(update);
        }
      },
    );

    return () => {
      disposed = true;
      unlisten?.();
      cancelAnimationFrame(frame);
    };
  }, []);

  return (
    <div className="cursor-companion-layer" aria-hidden="true">
      <InstructorPointer ref={pointer} />
    </div>
  );
}
