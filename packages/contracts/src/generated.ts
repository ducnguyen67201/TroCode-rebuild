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
  | RuntimeError
  | RuntimeListTargets
  | RuntimeListTargetsResult
  | RuntimeSelectTarget
  | RuntimeSelectTargetResult
  | RuntimeObserve
  | RuntimeObserveResult
  | RuntimeExplain
  | RuntimeExplainResult
  | RuntimeCheck
  | RuntimeCheckResult
  | RuntimePresentationAck
  | RuntimePresentationAckResult
  | RuntimeConfigure
  | RuntimeConfigured
  | RuntimeAsk
  | RuntimeAskResult
  | RuntimeRefreshCue
  | RuntimeRefreshCueResult
  | RuntimePlanControl
  | RuntimePlanControlResult;

export interface RuntimeInitialize {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.initialize';
  schemaDigest: string;
  accountId: string | null;
}
export interface RuntimeReady {
  protocolVersion: 2;
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
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.health';
}
export interface RuntimeHealthResult {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.healthResult';
  state: 'ready' | 'running' | 'stopped';
}
export interface RuntimeStart {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.start';
  sessionId: string;
}
export interface RuntimeStarted {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.started';
  sessionId: string;
}
export interface RuntimeStop {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.stop';
}
export interface RuntimeStopped {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.stopped';
}
export interface RuntimeShutdown {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.shutdown';
}
export interface RuntimeShutdownComplete {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.shutdownComplete';
}
export interface RuntimeError {
  protocolVersion: 2;
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
export interface RuntimeListTargets {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.listTargets';
}
export interface RuntimeListTargetsResult {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.targetsResult';
  state: TeachingState;
}
export interface TeachingState {
  revision: number;
  session_id: string | null;
  /**
   * @maxItems 100
   */
  targets: Target[];
  target: Target | null;
  observation: Observation | null;
  cue: TeachingCue | null;
  check: CheckResult | null;
  journey?: Journey | null;
  readiness?: {
    observation: 'unknown' | 'available' | 'unavailable';
    model: 'unconfigured' | 'ready' | 'unavailable';
    reason: 'none' | 'connect_model' | 'observe_again' | 'retry_plan';
    screen: 'unknown' | 'available' | 'unavailable';
    accessibility: 'unknown' | 'available' | 'unavailable';
  };
  /**
   * @maxItems 200
   */
  timings?: {
    phase: 'observation' | 'model' | 'grounding';
    elapsed_ms: number;
  }[];
}
export interface Target {
  pid: number;
  window_id: number;
  title: string;
  bounds: Rect;
}
export interface Rect {
  x: number;
  y: number;
  width: number;
  height: number;
}
export interface Observation {
  id: string;
  target: Target;
  captured_at: number;
  /**
   * @maxItems 200
   */
  elements: Element[];
  complete: boolean;
  screenshot_id?: string | null;
}
export interface Element {
  id: string;
  label: string;
  role: string;
  bounds: Rect;
  value: string;
}
export interface TeachingCue {
  id: string;
  observation_id: string;
  element_id: string;
  gesture: 'point' | 'click' | 'drag' | 'type' | 'scroll';
  caption: string;
  locale: 'en' | 'vi';
  source: Rect;
  destination: Rect | null;
  direction: ('up' | 'down' | 'left' | 'right') | null;
  expires_at: number;
  screenshot_id?: string | null;
  grounding?: 'accessibility' | 'visual';
}
export interface CheckResult {
  outcome: 'confirmed' | 'mismatch' | 'unknown';
  source: 'fresh_observation';
  observation_id: string;
  checked_at: number;
  message: string;
}
export interface Journey {
  id: string;
  index: number;
  status: 'running' | 'awaiting_confirmation' | 'paused' | 'completed';
  message: string;
  /**
   * @minItems 1
   * @maxItems 3
   */
  steps: [string] | [string, string] | [string, string, string];
}
export interface RuntimeSelectTarget {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.selectTarget';
  pid: number;
  windowId: number;
}
export interface RuntimeSelectTargetResult {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.targetSelected';
  state: TeachingState;
}
export interface RuntimeObserve {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.observe';
}
export interface RuntimeObserveResult {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.observationResult';
  state: TeachingState;
}
export interface RuntimeExplain {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.explain';
  elementId: string;
  gesture: 'point' | 'click' | 'drag' | 'type' | 'scroll';
  caption: string;
  locale: 'en' | 'vi';
  destinationId: string | null;
  direction: ('up' | 'down' | 'left' | 'right') | null;
}
export interface RuntimeExplainResult {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.explanationResult';
  state: TeachingState;
}
export interface RuntimeCheck {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.check';
  cueId: string;
  label: string;
  expected: string;
}
export interface RuntimeCheckResult {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.checkResult';
  state: TeachingState;
}
export interface RuntimePresentationAck {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.presentationAck';
  cueId: string;
}
export interface RuntimePresentationAckResult {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.presentationAckResult';
  state: TeachingState;
}
export interface RuntimeConfigure {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.configure';
  storageRoot: string;
  modelConfig: null | {
    origin: string;
    grant: string;
    model: string;
  };
}
export interface RuntimeConfigured {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.configured';
}
export interface RuntimeAsk {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.ask';
  question: string;
  locale: 'en' | 'vi';
}
export interface RuntimeAskResult {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.askResult';
  state: TeachingState;
}
export interface RuntimeRefreshCue {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.refreshCue';
}
export interface RuntimeRefreshCueResult {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.cueRefreshResult';
  state: TeachingState;
}
export interface RuntimePlanControl {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.planControl';
  action: 'pause' | 'resume' | 'confirm';
}
export interface RuntimePlanControlResult {
  protocolVersion: 2;
  requestId: string;
  correlationId: string;
  generationId: string;
  kind: 'runtime.planControlResult';
  state: TeachingState;
}
