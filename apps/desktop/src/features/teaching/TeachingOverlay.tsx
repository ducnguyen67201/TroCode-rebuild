import type { CSSProperties } from 'react';
import type { TeachingCue } from '@tro/contracts';
import { useLanguage } from '../../i18n';

const CALLOUT_WIDTH = 320;
const LINE_HEIGHT = 22;

function wrapCaption(caption: string, width = 42) {
  const words = caption
    .trim()
    .split(/\s+/)
    .flatMap((word) =>
      word.length <= width
        ? [word]
        : Array.from({ length: Math.ceil(word.length / width) }, (_, index) =>
            word.slice(index * width, (index + 1) * width),
          ),
    );
  const lines: string[] = [];
  for (const word of words) {
    const current = lines.at(-1);
    if (!current || current.length + word.length + 1 > width) lines.push(word);
    else lines[lines.length - 1] = `${current} ${word}`;
  }
  return lines.slice(0, 10);
}

/** SVG pixels only: no DOM input dispatch, pointer movement, or application focus. */
export function TeachingOverlay({
  cue,
  origin = { x: 0, y: 0 },
  stepIndex = 0,
  stepTotal = 1,
  viewport,
}: {
  cue: TeachingCue;
  origin?: { x: number; y: number };
  stepIndex?: number;
  stepTotal?: number;
  viewport?: { width: number; height: number };
}) {
  const { t } = useLanguage();
  const source = cue.source;
  const x = source.x - origin.x;
  const y = source.y - origin.y;
  const center = { x: x + source.width / 2, y: y + source.height / 2 };
  const destination = cue.destination
    ? {
        ...cue.destination,
        x: cue.destination.x - origin.x,
        y: cue.destination.y - origin.y,
      }
    : null;
  const bounds = viewport ?? {
    width:
      typeof window === 'undefined' || window.innerWidth <= 0
        ? 1280
        : window.innerWidth,
    height:
      typeof window === 'undefined' || window.innerHeight <= 0
        ? 720
        : window.innerHeight,
  };
  const captionLines = wrapCaption(cue.caption);
  const calloutHeight = 54 + captionLines.length * LINE_HEIGHT;
  const rightX = x + source.width + 24;
  const leftX = x - CALLOUT_WIDTH - 24;
  const rightFits = rightX + CALLOUT_WIDTH <= bounds.width - 12;
  const leftFits = leftX >= 12;
  const sideFits = rightFits || leftFits;
  const calloutX = Math.max(
    12,
    Math.min(
      rightFits ? rightX : leftFits ? leftX : center.x - CALLOUT_WIDTH / 2,
      bounds.width - CALLOUT_WIDTH - 12,
    ),
  );
  const verticalY =
    y + source.height + 24 + calloutHeight <= bounds.height - 12
      ? y + source.height + 24
      : y - calloutHeight - 24;
  const calloutY = Math.max(
    12,
    Math.min(sideFits ? y : verticalY, bounds.height - calloutHeight - 12),
  );
  const cursorClass =
    cue.gesture === 'drag' ? 'cue-cursor cue-drag' : 'cue-cursor cue-arrive';
  const cursorStyle = destination
    ? ({
        '--drag-x': `${destination.x + destination.width / 2 - center.x}px`,
        '--drag-y': `${destination.y + destination.height / 2 - center.y}px`,
      } as CSSProperties)
    : undefined;
  return (
    <svg
      className="teaching-overlay"
      role="img"
      focusable="false"
      aria-label={`${t('teaching.overlayAria', {
        gesture: t(`gesture.${cue.gesture}`),
        caption: cue.caption,
      })}, ${stepIndex + 1}/${stepTotal}`}
    >
      <path
        className="cue-approach"
        d={`M ${center.x - 52} ${center.y - 44} Q ${center.x - 22} ${center.y - 18} ${center.x} ${center.y}`}
      />
      <rect
        x={x}
        y={y}
        width={source.width}
        height={source.height}
        rx="10"
        className="cue-target"
      />
      {cue.gesture === 'drag' && destination && (
        <>
          <rect
            x={destination.x}
            y={destination.y}
            width={destination.width}
            height={destination.height}
            rx="10"
            className="cue-destination"
          />
          <path
            className="cue-path"
            d={`M ${center.x} ${center.y} L ${destination.x + destination.width / 2} ${destination.y + destination.height / 2}`}
          />
        </>
      )}
      <g className={cursorClass} style={cursorStyle}>
        <circle
          className={
            cue.gesture === 'click' ? 'cue-pointer cue-pulse' : 'cue-pointer'
          }
          cx={center.x}
          cy={center.y}
          r="14"
        />
        <path
          className="cue-cursor-glyph"
          d={`M ${center.x - 7} ${center.y - 16} L ${center.x + 11} ${center.y + 8} L ${center.x + 3} ${center.y + 7} L ${center.x} ${center.y + 17} L ${center.x - 7} ${center.y + 14} L ${center.x - 3} ${center.y + 5} L ${center.x - 12} ${center.y + 9} Z`}
        />
      </g>
      {cue.gesture === 'scroll' && (
        <text x={center.x + 24} y={center.y + 8} className="cue-symbol">
          {
            { up: '↑', down: '↓', left: '←', right: '→' }[
              cue.direction ?? 'down'
            ]
          }
        </text>
      )}
      {cue.gesture === 'type' && (
        <text x={center.x + 24} y={center.y + 8} className="cue-symbol">
          Aa
        </text>
      )}
      <g
        className="cue-callout"
        transform={`translate(${calloutX} ${calloutY})`}
      >
        <rect width={CALLOUT_WIDTH} height={calloutHeight} rx="14" />
        <text x="18" y="27" className="cue-step-label">
          {t('teaching.overlayStep', {
            current: stepIndex + 1,
            total: stepTotal,
            gesture: t(`gesture.action.${cue.gesture}`),
          })}
        </text>
        <text x="18" y="54" className="cue-caption" lang={cue.locale}>
          {captionLines.map((line, index) => (
            <tspan
              key={`${index}-${line}`}
              x="18"
              dy={index === 0 ? 0 : LINE_HEIGHT}
            >
              {line}
            </tspan>
          ))}
        </text>
      </g>
    </svg>
  );
}
