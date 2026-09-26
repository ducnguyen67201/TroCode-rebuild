import React from 'react';
import { afterEach, expect, it } from 'vitest';
import { cleanup, render, screen } from '@testing-library/react';
import { VoiceHudPresentation } from './VoiceHud';

afterEach(cleanup);

it('renders a compact non-interactive waveform pill with an accessible status', () => {
  const { container } = render(
    <VoiceHudPresentation
      status={{ phase: 'listening', revision: 1, message: 'Listening…' }}
    />,
  );

  expect(screen.getByRole('status', { name: 'Listening…' })).toBeTruthy();
  expect(container.querySelectorAll('.voice-wave i')).toHaveLength(13);
  expect(container.querySelectorAll('button,input,a')).toHaveLength(0);
  expect(container.textContent).toBe('');
});
