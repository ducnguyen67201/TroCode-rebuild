import asyncio
import time
from dataclasses import replace
from uuid import uuid4

import pytest
from test_guidance import observation

from tro_runtime.teaching import TeachingSession


class Source:
    def __init__(self):
        self.value = "0"
        self.closed = False

    async def list_targets(self):
        return (observation().target,)

    async def frontmost_target(self):
        return observation().target

    async def observe(self, target, include_image=True):
        return replace(observation(self.value, time.time(), str(uuid4())), target=target)

    async def close(self):
        self.closed = True


def test_report_is_not_success_and_close_clears_guidance():
    async def scenario():
        source = Source()
        session = TeachingSession(source)
        await session.handle({"kind": "runtime.listTargets"})
        await session.handle({"kind": "runtime.selectTarget", "pid": 42, "windowId": 7})
        await session.handle({"kind": "runtime.observe"})
        await session.handle(
            {
                "kind": "runtime.explain",
                "elementId": "1",
                "gesture": "click",
                "caption": "Click yourself",
                "locale": "en",
                "destinationId": None,
                "direction": None,
            }
        )
        cue = session.cue
        assert cue is not None and source.value == "0"
        await session.handle({"kind": "runtime.presentationAck", "cueId": cue.id})
        assert session.check is None
        await session.handle(
            {
                "kind": "runtime.check",
                "cueId": cue.id,
                "label": "Counter",
                "expected": "1",
                "requestId": str(uuid4()),
            }
        )
        assert session.check["outcome"] == "mismatch"
        await session.close()
        assert source.closed and session.cue is None

    asyncio.run(scenario())


def test_unknown_target_and_automation_requests_are_rejected():
    async def scenario():
        session = TeachingSession(Source())
        with pytest.raises(ValueError):
            await session.handle({"kind": "runtime.selectTarget", "pid": 99, "windowId": 1})
        with pytest.raises(ValueError):
            await session.handle({"kind": "runtime.execute"})

    asyncio.run(scenario())


def test_refresh_hides_changed_control_but_preserves_the_learner_check():
    async def scenario():
        source = Source()
        session = TeachingSession(source)
        session.target = observation().target
        await session.handle({"kind": "runtime.observe"})
        await session.handle(
            {
                "kind": "runtime.explain",
                "elementId": "1",
                "gesture": "click",
                "caption": "Click yourself",
                "locale": "en",
                "destinationId": None,
                "direction": None,
            }
        )
        cue_id = session.cue.id
        await session.handle({"kind": "runtime.refreshCue"})
        assert session.cue.id == cue_id
        source.value = "1"
        await session.handle({"kind": "runtime.refreshCue"})
        assert session.cue is None
        await session.handle(
            {
                "kind": "runtime.check",
                "cueId": cue_id,
                "label": "Counter",
                "expected": "1",
                "requestId": str(uuid4()),
            }
        )
        assert session.check["outcome"] == "confirmed"
        session.observation = replace(session.observation, image="private-image")
        assert "image" not in session.projection(str(uuid4()))["observation"]
        await session.close()

    asyncio.run(scenario())


def test_terminal_action_event_is_published_after_next_preparation_is_allowed(monkeypatch):
    async def scenario():
        class CompletedAgent:
            def __init__(self, *_args):
                pass

            async def run(self, _instruction):
                return None

        source = Source()
        session = TeachingSession(source)
        monkeypatch.setattr("tro_runtime.teaching.CuaObservationSource", Source)
        monkeypatch.setattr("tro_runtime.action_agent.ComputerActionAgent", CompletedAgent)
        monkeypatch.setattr("tro_runtime.model_client.create_model", lambda *_args: object())
        first_utterance = str(uuid4())
        prepared = await session.prepare_instruction({"utteranceId": first_utterance})
        next_utterance = str(uuid4())
        next_preparation = None

        async def event_sink(event):
            nonlocal next_preparation
            if event["phase"] == "completed":
                next_preparation = await session.prepare_instruction(
                    {"utteranceId": next_utterance}
                )

        session.event_sink = event_sink
        await session.execute_instruction(
            {
                "protocolVersion": 3,
                "generationId": str(uuid4()),
                "utteranceId": first_utterance,
                "preparationId": prepared["preparationId"],
                "instruction": "Do one safe thing",
                "modelConfig": {"origin": "local", "grant": "grant", "model": "model"},
            }
        )
        task = session.action_task
        assert task is not None
        await task

        assert next_preparation is not None
        assert next_preparation["utteranceId"] == next_utterance

    asyncio.run(scenario())
