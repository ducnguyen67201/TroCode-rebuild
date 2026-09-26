import React from 'react';
import { afterEach, expect, it } from 'vitest';
import { cleanup, render } from '@testing-library/react';
import type { TeachingCue } from '@tro/contracts';
import { TeachingOverlay } from './TeachingOverlay';
afterEach(cleanup);
const cue: TeachingCue = {
  id: 'cue',
  observation_id: 'observation',
  element_id: '1',
  gesture: 'drag',
  caption: 'Drag the block yourself.',
  locale: 'en',
  source: { x: -50, y: 20, width: 80, height: 40 },
  destination: { x: 200, y: 200, width: 80, height: 80 },
  direction: null,
  expires_at: 101,
};
it('renders a drag path in overlay coordinates without actionable controls', () => {
  const { container } = render(
    <TeachingOverlay cue={cue} origin={{ x: -100, y: 0 }} />,
  );
  expect(container.querySelector('.cue-path')?.getAttribute('d')).toBe(
    'M 90 40 L 340 240',
  );
  expect(container.querySelectorAll('button,input,a')).toHaveLength(0);
  expect(
    container.querySelectorAll('.cue-target,.cue-destination'),
  ).toHaveLength(2);
  expect(container.textContent).toContain('Step 1 of 1 · Drag');
  expect(container.textContent).toContain('Drag the block yourself.');
});
it('renders a ghost click without changing external input', () => {
  const { container } = render(
    <TeachingOverlay cue={{ ...cue, gesture: 'click', destination: null }} />,
  );
  expect(container.querySelector('.cue-pulse')).not.toBeNull();
  expect(container.querySelector('.cue-path')).toBeNull();
});

it.each([
  ['point', 'Look here'],
  ['click', 'Click'],
  ['type', 'Type'],
  ['scroll', 'Scroll'],
] as const)('renders the %s teaching demonstration', (gesture, verb) => {
  const { container } = render(
    <TeachingOverlay
      cue={{
        ...cue,
        gesture,
        destination: null,
        direction: gesture === 'scroll' ? 'right' : null,
      }}
    />,
  );
  expect(container.textContent).toContain(`Step 1 of 1 · ${verb}`);
  expect(container.querySelectorAll('button,input,a,[tabindex]')).toHaveLength(
    0,
  );
  if (gesture === 'type') expect(container.textContent).toContain('Aa');
  if (gesture === 'scroll') expect(container.textContent).toContain('→');
});

it('clamps a readable callout near the viewport edge and reports progress', () => {
  const { container } = render(
    <TeachingOverlay
      cue={{ ...cue, gesture: 'point', destination: null }}
      origin={{ x: -100, y: 0 }}
      stepIndex={1}
      stepTotal={3}
      viewport={{ width: 420, height: 220 }}
    />,
  );
  expect(container.textContent).toContain('Step 2 of 3 · Look here');
  expect(
    container.querySelector('.cue-callout')?.getAttribute('transform'),
  ).toBe('translate(12 84)');
  expect(container.querySelector('svg')?.getAttribute('focusable')).toBe(
    'false',
  );
  expect(container.querySelectorAll('button,input,a')).toHaveLength(0);
});
