/* Generated. Do not edit. */

export type WorkspaceMember = {
  membershipId: string;
  email: string;
  displayName: string | null;
  role: 'owner' | 'teacher' | 'student';
  state: 'pending' | 'active';
  joinedAt: string | null;
};

export interface WorkspaceMemberList {
  workspace: ManagedWorkspaceSummary;
  /**
   * @maxItems 500
   */
  members: WorkspaceMember[];
}
export interface ManagedWorkspaceSummary {
  workspaceId: string;
  name: string;
  role: 'owner';
}
