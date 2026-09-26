import asyncio
import time
from dataclasses import replace
from uuid import uuid4

import pytest
from test_guidance import observation

from tro_runtime.errors import GuidanceError
from tro_runtime.planning import PlannedStep, PlanProgress, TeachingPlan
from tro_runtime.teaching import TeachingSession


class Source:
    def __init__(self):
        self.value = "0"
        self.closed = False

    async def list_targets(self):
        return (observation().target,)

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


def test_prepared_text_or_voice_instruction_starts_the_same_visual_journey(monkeypatch):
    async def plan(self, observation, question, locale, completed=()):
        assert question == "Show me how to use the counter"
        assert locale == "en"
        return TeachingPlan(
            steps=[
                PlannedStep.model_validate(
                    {
                        "target": {"role": "button", "label": "Counter"},
                        "gesture": "click",
                        "caption": "Click the counter yourself.",
                        "destination": None,
                        "direction": None,
                        "expected": None,
                    }
                )
            ]
        )

    monkeypatch.setattr("tro_runtime.agent.GuidanceAgent.plan", plan)

    async def scenario():
        source = Source()
        session = TeachingSession(source)
        utterance_id = str(uuid4())
        prepared = await session.prepare_instruction(
            {"kind": "runtime.prepareInstruction", "utteranceId": utterance_id}
        )
        await session.start_guidance(
            {
                "kind": "runtime.startGuidance",
                "utteranceId": utterance_id,
                "preparationId": prepared["preparationId"],
                "instruction": "Show me how to use the counter",
                "locale": "en",
                "modelConfig": {
                    "origin": "https://api.example.com",
                    "grant": "a" * 64,
                    "model": "guidance-model",
                },
            }
        )
        assert session.progress is not None
        assert session.progress.plan.steps[0].gesture == "click"
        assert session.cue is not None
        assert source.value == "0"
        assert session.prepared is None
        await session.close()

    asyncio.run(scenario())


def test_instruction_preparation_is_single_use_even_when_stale():
    async def scenario():
        session = TeachingSession(Source())
        utterance_id = str(uuid4())
        prepared = await session.prepare_instruction(
            {"kind": "runtime.prepareInstruction", "utteranceId": utterance_id}
        )
        assert session.prepared is not None
        session.prepared["expires"] = time.monotonic() - 1
        with pytest.raises(ValueError, match="no longer current"):
            await session.start_guidance(
                {
                    "utteranceId": utterance_id,
                    "preparationId": prepared["preparationId"],
                    "instruction": "Show me",
                    "locale": "en",
                    "modelConfig": {},
                }
            )
        assert session.prepared is None
        await session.close()

    asyncio.run(scenario())


def test_failed_prepared_guidance_keeps_prior_journey_and_target(monkeypatch):
    async def fail(*args, **kwargs):
        raise RuntimeError("private screen detail")

    monkeypatch.setattr("tro_runtime.agent.GuidanceAgent.plan", fail)

    async def scenario():
        session = TeachingSession(Source())
        old_target = replace(observation().target, pid=99)
        session.target = old_target
        session.progress = prior = PlanProgress(
            TeachingPlan(
                steps=[
                    PlannedStep.model_validate(
                        {
                            "target": {"role": "button", "label": "Counter"},
                            "gesture": "click",
                            "caption": "Click it yourself.",
                            "destination": None,
                            "direction": None,
                            "expected": None,
                        }
                    )
                ]
            ),
            "en",
            replace(observation(), target=old_target),
        )
        utterance_id = str(uuid4())
        prepared = await session.prepare_instruction({"utteranceId": utterance_id})
        with pytest.raises(GuidanceError, match="could not be grounded"):
            await session.start_guidance(
                {
                    "utteranceId": utterance_id,
                    "preparationId": prepared["preparationId"],
                    "instruction": "Show me",
                    "locale": "en",
                    "modelConfig": {
                        "origin": "https://api.example.com",
                        "grant": "a" * 64,
                        "model": "guidance-model",
                    },
                }
            )
        assert session.progress is prior and session.target == old_target
        assert session.prepared is None
        await session.close()

    asyncio.run(scenario())
