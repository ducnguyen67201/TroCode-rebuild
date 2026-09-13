import Ajv from 'ajv';
import protocolSchema from '../schema/protocol.schema.json';
import statusSchema from '../schema/status.schema.json';
import type { TeachingState, RuntimeMessage } from './generated';
import type { RuntimeStatus } from './generated-status';
const ajv = new Ajv({
  strict: true,
  allowUnionTypes: true,
  coerceTypes: false,
});
const messageValidator = ajv.compile<RuntimeMessage>(protocolSchema);
const statusValidator = ajv.compile<RuntimeStatus>(statusSchema);
export function parseMessage(value: unknown): RuntimeMessage {
  if (!messageValidator(value)) throw new Error('Invalid runtime message');
  return value;
}
export function parseStatus(value: unknown): RuntimeStatus {
  if (!statusValidator(value)) throw new Error('Invalid runtime status');
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
