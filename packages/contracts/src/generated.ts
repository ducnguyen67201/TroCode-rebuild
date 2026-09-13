/* Generated. Do not edit. */

export type RuntimeMessage =
  | RuntimeInitialize
  | RuntimeReady
  | RuntimeHealth
  | RuntimeHealthResult
  | RuntimeStart
  | RuntimeStarted
  | RuntimeStop
  | RuntimeStopped
  | RuntimeShutdown
  | RuntimeShutdownComplete
  | RuntimeError;

export interface RuntimeInitialize {
  protocolVersion: 1;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.initialize';
  schemaDigest: string;
  accountId: string | null;
}
export interface RuntimeReady {
  protocolVersion: 1;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.ready';
  schemaDigest: string;
  /**
   * @maxItems 1
   */
  capabilities: [] | ['diagnostic'];
}
export interface RuntimeHealth {
  protocolVersion: 1;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.health';
}
export interface RuntimeHealthResult {
  protocolVersion: 1;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.healthResult';
  state: 'ready' | 'running' | 'stopped';
}
export interface RuntimeStart {
  protocolVersion: 1;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.start';
  sessionId: string;
}
export interface RuntimeStarted {
  protocolVersion: 1;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.started';
  sessionId: string;
}
export interface RuntimeStop {
  protocolVersion: 1;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.stop';
}
export interface RuntimeStopped {
  protocolVersion: 1;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.stopped';
}
export interface RuntimeShutdown {
  protocolVersion: 1;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.shutdown';
}
export interface RuntimeShutdownComplete {
  protocolVersion: 1;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.shutdownComplete';
}
export interface RuntimeError {
  protocolVersion: 1;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.error';
  code:
    | 'INVALID_MESSAGE'
    | 'PROTOCOL_MISMATCH'
    | 'NOT_READY'
    | 'BUSY'
    | 'TIMEOUT'
    | 'WORKER_EXITED'
    | 'UNAUTHORIZED'
    | 'FORBIDDEN'
    | 'INTERNAL';
  message: string;
  retryable: boolean;
}
