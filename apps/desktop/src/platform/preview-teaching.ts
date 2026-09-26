import type { TeachingState, TeachingCue } from '@tro/contracts';
import type { TeachingClient } from './teaching-client';
export function createPreviewTeaching(): TeachingClient {
  const bounds = { x: 0, y: 0, width: 600, height: 300 };
  const target = {
    pid: 1,
    window_id: 1,
    title: 'Simulated practice window',
    bounds,
  };
  let state: TeachingState = {
    revision: 0,
    session_id: crypto.randomUUID(),
    targets: [],
    target: null,
    observation: null,
    cue: null,
    check: null,
  };
  const publish = () => {
    state = { ...state, revision: state.revision + 1 };
    return structuredClone(state);
  };
  function observe() {
    state.observation = {
      id: crypto.randomUUID(),
      target,
      captured_at: Date.now() / 1000,
      complete: true,
      elements: [
        {
          id: '1',
          label: 'Practice control',
          role: 'button',
          value: '0',
          bounds: { x: 40, y: 40, width: 120, height: 50 },
        },
        {
          id: '2',
          label: 'Destination',
          role: 'group',
          value: '',
          bounds: { x: 350, y: 180, width: 120, height: 80 },
        },
      ],
    };
    state.cue = null;
    state.check = null;
  }
  function cue(
    gesture: TeachingCue['gesture'],
    caption: string,
    locale: TeachingCue['locale'],
    destinationId: string | null,
    direction: TeachingCue['direction'],
  ) {
    observe();
    state.cue = {
      id: crypto.randomUUID(),
      observation_id: state.observation!.id,
      element_id: '1',
      gesture,
      caption,
      locale,
      source: state.observation!.elements[0]!.bounds,
      destination: destinationId
        ? state.observation!.elements[1]!.bounds
        : null,
      direction,
      expires_at: Date.now() / 1000 + 1,
    };
  }
  return {
    planControl: async (action) => {
      const journey = state.journey;
      if (!journey) throw new Error('No preview plan');
      if (action === 'pause') {
        journey.status = 'paused';
        state.cue = null;
      } else if (action === 'resume') journey.status = 'awaiting_confirmation';
      else if (
        action === 'confirm' &&
        journey.status === 'awaiting_confirmation'
      ) {
        journey.index++;
        journey.status =
          journey.index === journey.steps.length
            ? 'completed'
            : 'awaiting_confirmation';
        state.cue = null;
      } else throw new Error('Unavailable preview control');
      journey.message =
        'Simulation only: preview cannot observe external actions.';
      return publish();
    },
    connectProof: async () => {
      state = {
        ...state,
        target: null,
        observation: null,
        cue: null,
        check: null,
      };
    },
    listTargets: async () => {
      state.journey = null;
      state.targets = [target];
      return publish();
    },
    selectTarget: async () => {
      state.journey = null;
      state.target = target;
      state.observation = null;
      return publish();
    },
    observe: async () => {
      observe();
      return publish();
    },
    ask: async (_question, locale) => {
      state.journey = {
        id: crypto.randomUUID(),
        index: 0,
        status: 'awaiting_confirmation',
        message: 'Simulation only: preview cannot observe external actions.',
        steps: ['Locate the practice control.', 'Try the control yourself.'],
      };
      cue(
        'point',
        'Use your own mouse on this practice control.',
        locale,
        null,
        null,
      );
      return publish();
    },
    explain: async (request) => {
      state.journey = null;
      cue(
        request.gesture,
        request.caption,
        request.locale,
        request.destinationId,
        request.direction,
      );
      return publish();
    },
    check: async () => {
      observe();
      state.check = {
        outcome: 'unknown',
        source: 'fresh_observation',
        observation_id: state.observation!.id,
        checked_at: Date.now() / 1000,
        message: 'Preview cannot observe your actions in another application.',
      };
      return publish();
    },
  };
}
