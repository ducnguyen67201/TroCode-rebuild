import type { CSSProperties } from 'react';
import type { TeachingCue } from '@tro/contracts';
/** SVG pixels only: no DOM input dispatch, pointer movement, or application focus. */
export function TeachingOverlay({
  cue,
  origin = { x: 0, y: 0 },
}: {
  cue: TeachingCue;
  origin?: { x: number; y: number };
}) {
  const source = cue.source;
  const x = source.x - origin.x;
  const y = source.y - origin.y;
  const center = { x: x + source.width / 2, y: y + source.height / 2 };
  const destination = cue.destination;
  return (
    <svg
      className="teaching-overlay"
      aria-label={`${cue.gesture} guidance: ${cue.caption}`}
    >
      <rect
        x={x}
        y={y}
        width={source.width}
        height={source.height}
        rx="8"
        className="cue-target"
      />
      {cue.gesture === 'drag' && destination && (
        <>
          <rect
            x={destination.x - origin.x}
            y={destination.y - origin.y}
            width={destination.width}
            height={destination.height}
            rx="8"
            className="cue-destination"
          />
          <path
            className="cue-path"
            d={`M ${center.x} ${center.y} L ${destination.x - origin.x + destination.width / 2} ${destination.y - origin.y + destination.height / 2}`}
          />
        </>
      )}
      <circle
        className={
          cue.gesture === 'click'
            ? 'cue-pointer cue-pulse'
            : cue.gesture === 'drag'
              ? 'cue-pointer cue-drag'
              : 'cue-pointer'
        }
        style={
          destination
            ? ({
                '--drag-x': `${destination.x + destination.width / 2 - source.x - source.width / 2}px`,
                '--drag-y': `${destination.y + destination.height / 2 - source.y - source.height / 2}px`,
              } as CSSProperties)
            : undefined
        }
        cx={center.x}
        cy={center.y}
        r="12"
      />
      {cue.gesture === 'scroll' && (
        <text x={center.x + 20} y={center.y} className="cue-symbol">
          {
            { up: '↑', down: '↓', left: '←', right: '→' }[
              cue.direction ?? 'down'
            ]
          }
        </text>
      )}
      {cue.gesture === 'type' && (
        <text x={center.x + 20} y={center.y} className="cue-symbol">
          Aa
        </text>
      )}
    </svg>
  );
}
