import type { WorkspaceMember, WorkspaceMemberList } from '@tro/contracts';

export type AddWorkspaceMemberRole = 'teacher' | 'student';

export interface WorkspaceClient {
  members(workspaceId: string): Promise<WorkspaceMemberList>;
  addMember(
    workspaceId: string,
    email: string,
    role: AddWorkspaceMemberRole,
  ): Promise<WorkspaceMember>;
  removeMember(workspaceId: string, membershipId: string): Promise<void>;
}
