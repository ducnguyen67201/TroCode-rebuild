import React from 'react';
import { afterEach, expect, it } from 'vitest';
import {
  cleanup,
  fireEvent,
  render,
  screen,
  waitFor,
} from '@testing-library/react';
import { RuntimeStatus } from './RuntimeStatus';
import { createPreviewClient } from '../../platform/preview-client';
afterEach(cleanup);
it('starts and stops an explicit preview session', async () => {
  render(<RuntimeStatus client={createPreviewClient()} />);
  fireEvent.click(screen.getByText('Start session'));
  await waitFor(() =>
    expect(screen.getByRole('status').textContent).toContain('running'),
  );
  fireEvent.click(screen.getByText('Stop'));
  await waitFor(() =>
    expect(screen.getByRole('status').textContent).toContain('stopped'),
  );
});
it('displays sanitized failures', async () => {
  const client = createPreviewClient();
  client.start = async () => {
    throw { message: 'secret', code: 'TIMEOUT' };
  };
  render(<RuntimeStatus client={client} />);
  fireEvent.click(screen.getByText('Start session'));
  await waitFor(() =>
    expect(screen.getByRole('alert').textContent).toContain('timed out'),
  );
});
