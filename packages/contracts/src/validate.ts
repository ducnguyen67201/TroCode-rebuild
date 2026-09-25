import Ajv from 'ajv';
import protocolSchema from '../schema/protocol.schema.json';
import statusSchema from '../schema/status.schema.json';
import authSchema from '../schema/auth.schema.json';
import workspaceSchema from '../schema/workspace.schema.json';
import type { TeachingState, RuntimeMessage } from './generated';
import type { RuntimeStatus } from './generated-status';
import type { AuthStatus } from './generated-auth';
import type {
  WorkspaceMember,
  WorkspaceMemberList,
} from './generated-workspace';
const ajv = new Ajv({
  strict: true,
  allowUnionTypes: true,
  coerceTypes: false,
});
const messageValidator = ajv.compile<RuntimeMessage>(protocolSchema);
const statusValidator = ajv.compile<RuntimeStatus>(statusSchema);
const authStatusValidator = ajv.compile<AuthStatus>(authSchema);
const workspaceMemberListValidator =
  ajv.compile<WorkspaceMemberList>(workspaceSchema);
const workspaceMemberValidator = ajv.compile<WorkspaceMember>({
  $ref: '#/definitions/WorkspaceMember',
  definitions: workspaceSchema.definitions,
});
export function parseMessage(value: unknown): RuntimeMessage {
  if (!messageValidator(value)) throw new Error('Invalid runtime message');
  return value;
}
export function parseStatus(value: unknown): RuntimeStatus {
  if (!statusValidator(value)) throw new Error('Invalid runtime status');
  return value;
}

export function parseAuthStatus(value: unknown): AuthStatus {
  if (!authStatusValidator(value)) throw new Error('Invalid auth status');
  return value;
}

export function parseWorkspaceMemberList(value: unknown): WorkspaceMemberList {
  if (!workspaceMemberListValidator(value))
    throw new Error('Invalid workspace member list');
  return value;
}

export function parseWorkspaceMember(value: unknown): WorkspaceMember {
  if (!workspaceMemberValidator(value))
    throw new Error('Invalid workspace member');
  return value;
}

const teachingValidator = ajv.compile<TeachingState>({
  $ref: '#/definitions/TeachingState',
  definitions: protocolSchema.definitions,
});
export function parseTeaching(value: unknown): TeachingState {
  if (!teachingValidator(value)) throw new Error('Invalid teaching state');
  return value;
}
