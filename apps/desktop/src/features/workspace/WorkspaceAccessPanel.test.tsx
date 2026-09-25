import { afterEach, expect, it, vi } from 'vitest';
import {
  cleanup,
  fireEvent,
  render,
  screen,
  waitFor,
} from '@testing-library/react';
import type {
  WorkspaceMember,
  WorkspaceMemberList,
  WorkspaceSummary,
} from '@tro/contracts';
import type { WorkspaceClient } from '../../platform/workspace-client';
import { WorkspaceAccessPanel } from './WorkspaceAccessPanel';

afterEach(cleanup);

const workspace: WorkspaceSummary = {
  workspaceId: '00000000-0000-0000-0000-000000000002',
  name: 'Northstar Robotics',
  role: 'owner',
};

const owner: WorkspaceMember = {
  membershipId: '00000000-0000-0000-0000-000000000003',
  email: 'owner@example.com',
  displayName: 'Northstar Owner',
  role: 'owner',
  state: 'active',
  joinedAt: '2026-09-25T12:00:00Z',
};

it('adds an exact-email pending member and refreshes the owner list', async () => {
  const pending: WorkspaceMember = {
    membershipId: '00000000-0000-0000-0000-000000000004',
    email: 'student+robotics@example.com',
    displayName: null,
    role: 'student',
    state: 'pending',
    joinedAt: null,
  };
  const members = vi
    .fn<WorkspaceClient['members']>()
    .mockResolvedValueOnce(memberList([owner]))
    .mockResolvedValueOnce(memberList([owner, pending]));
  const addMember = vi
    .fn<WorkspaceClient['addMember']>()
    .mockResolvedValue(pending);
  const client = clientWith({ members, addMember });

  render(<WorkspaceAccessPanel client={client} workspace={workspace} />);
  await screen.findByText('Northstar Owner');
  fireEvent.change(screen.getByLabelText('Google email'), {
    target: { value: ' student+robotics@example.com ' },
  });
  fireEvent.click(screen.getByRole('button', { name: 'Add access' }));

  await screen.findByText('student+robotics@example.com');
  expect(addMember).toHaveBeenCalledWith(
    workspace.workspaceId,
    'student+robotics@example.com',
    'student',
  );
  expect(screen.getByRole('status').textContent).toContain('Access added');
});

it('requires a second click before removing a non-owner member', async () => {
  const student: WorkspaceMember = {
    membershipId: '00000000-0000-0000-0000-000000000004',
    email: 'student@example.com',
    displayName: null,
    role: 'student',
    state: 'pending',
    joinedAt: null,
  };
  const members = vi
    .fn<WorkspaceClient['members']>()
    .mockResolvedValueOnce(memberList([owner, student]))
    .mockResolvedValueOnce(memberList([owner]));
  const removeMember = vi
    .fn<WorkspaceClient['removeMember']>()
    .mockResolvedValue();
  const client = clientWith({ members, removeMember });

  render(<WorkspaceAccessPanel client={client} workspace={workspace} />);
  const remove = await screen.findByRole('button', { name: 'Remove access' });
  fireEvent.click(remove);
  expect(removeMember).not.toHaveBeenCalled();
  fireEvent.click(screen.getByRole('button', { name: 'Confirm removal' }));

  await waitFor(() =>
    expect(screen.queryByText('student@example.com')).toBeNull(),
  );
  expect(removeMember).toHaveBeenCalledWith(
    workspace.workspaceId,
    student.membershipId,
  );
  expect(screen.queryByRole('button', { name: 'Remove access' })).toBeNull();
});

function memberList(members: WorkspaceMember[]): WorkspaceMemberList {
  return { workspace: { ...workspace, role: 'owner' }, members };
}

function clientWith(overrides: Partial<WorkspaceClient>): WorkspaceClient {
  return {
    members: async () => memberList([owner]),
    addMember: async () => {
      throw new Error('not implemented');
    },
    removeMember: async () => undefined,
    ...overrides,
  };
}
