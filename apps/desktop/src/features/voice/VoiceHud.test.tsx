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

it('projects planning and guiding without exposing controls', () => {
  const { container, rerender } = render(
    <VoiceHudPresentation
      status={{
        phase: 'planning',
        revision: 2,
        message: 'Preparing a simple walkthrough…',
      }}
    />,
  );
  expect(container.querySelector('.voice-hud-planning')).not.toBeNull();
  rerender(
    <VoiceHudPresentation
      status={{
        phase: 'guiding',
        revision: 3,
        message: 'Follow the cursor in the selected window.',
      }}
    />,
  );
  expect(container.querySelector('.voice-hud-guiding')).not.toBeNull();
  expect(container.querySelectorAll('button,input,a')).toHaveLength(0);
});
