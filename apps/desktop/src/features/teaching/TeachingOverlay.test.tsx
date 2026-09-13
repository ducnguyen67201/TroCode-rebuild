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
  expect(container.querySelector('path')?.getAttribute('d')).toBe(
    'M 90 40 L 340 240',
  );
  expect(container.querySelectorAll('button,input,a')).toHaveLength(0);
  expect(container.querySelectorAll('rect')).toHaveLength(2);
});
it('renders a ghost click without changing external input', () => {
  const { container } = render(
    <TeachingOverlay cue={{ ...cue, gesture: 'click', destination: null }} />,
  );
  expect(container.querySelector('.cue-pulse')).not.toBeNull();
  expect(container.querySelector('path')).toBeNull();
});
