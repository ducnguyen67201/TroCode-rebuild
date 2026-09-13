import asyncio
import time
from dataclasses import replace
from uuid import uuid4

import pytest
from test_guidance import observation
from test_teaching import Source

from tro_runtime.planning import PlannedStep, PlanProgress, TeachingPlan
from tro_runtime.teaching import TeachingSession


def step(expected="1"):
    return PlannedStep.model_validate(
        {
            "target": {"role": "button", "label": "Counter"},
            "gesture": "click",
            "caption": "Click the counter yourself.",
            "destination": None,
            "direction": None,
            "expected": None
            if expected is None
            else {"target": {"role": "button", "label": "Counter"}, "value": expected},
        }
    )


def fresh(value="0", **kwargs):
    return observation(value, time.time(), str(uuid4()), **kwargs)


def test_plan_advances_without_model_and_requires_stable_transition():
    progress = PlanProgress(TeachingPlan(steps=[step(), step("2")]), "en", fresh())
    assert progress.observe(fresh()) is not None
    progress.observe(fresh("1"))
    assert progress.index == 0
    progress.observe(fresh("1"))
    assert progress.index == 1
    progress.observe(fresh("2"))
    assert progress.index == 1
    progress.observe(fresh("2"))
    assert progress.status == "completed"


def test_already_satisfied_condition_never_auto_completes():
    progress = PlanProgress(TeachingPlan(steps=[step()]), "en", fresh("1"))
    for _ in range(3):
        progress.observe(fresh("1"))
    assert progress.index == 0
    assert progress.status == "awaiting_confirmation"
    progress.control("confirm")
    assert progress.status == "completed"


def test_stale_partial_and_duplicate_evidence_cannot_advance():
    progress = PlanProgress(TeachingPlan(steps=[step()]), "en", fresh())
    progress.observe(fresh())
    item = fresh("1")
    progress.observe(item)
    progress.observe(item)
    progress.observe(fresh("1", complete=False))
    progress.observe(replace(fresh("1"), captured_at=time.time() - 2))
    assert progress.index == 0
    progress.observe(fresh("1"))
    assert progress.index == 0
    progress.observe(fresh("1"))
    assert progress.status == "completed"


def test_rebinds_semantic_target_and_rejects_ambiguity_or_wrong_window():
    progress = PlanProgress(TeachingPlan(steps=[step()]), "en", fresh())
    item = fresh()
    renamed = replace(item, elements=(replace(item.elements[0], id="new-id"),))
    assert progress.observe(renamed).element_id == "new-id"
    duplicate = fresh()
    assert progress.observe(replace(duplicate, elements=duplicate.elements * 2)) is None
    wrong = fresh()
    assert progress.observe(replace(wrong, target=replace(wrong.target, pid=99))) is None
    assert progress.status == "paused"


def test_pause_resume_and_missing_target_do_not_skip_steps():
    progress = PlanProgress(TeachingPlan(steps=[step()]), "en", fresh())
    progress.observe(fresh())
    progress.control("pause")
    assert progress.observe(fresh("1")) is None
    progress.control("resume")
    progress.observe(fresh("1"))
    assert progress.index == 0 and progress.status == "awaiting_confirmation"
    progress.started -= 11
    progress.observe(replace(fresh(), elements=()))
    assert progress.status == "paused"


@pytest.mark.parametrize(
    "change", [{"gesture": "execute"}, {"gesture": "drag"}, {"direction": "up"}]
)
def test_plan_rejects_actions_and_invalid_gestures(change):
    with pytest.raises(ValueError):
        PlannedStep.model_validate({**step().model_dump(), **change})


def test_session_calls_planner_once_then_local_observation_and_stops():
    class Planner:
        calls = 0

        async def plan(self, *args):
            self.calls += 1
            return TeachingPlan(steps=[step(), step("2")])

    async def scenario():
        source = Source()
        planner = Planner()
        session = TeachingSession(source)
        session.agent = planner
        session.target = fresh().target
        await session.handle({"kind": "runtime.ask", "question": "Help", "locale": "en"})
        source.value = "1"
        for _ in range(2):
            await session.handle({"kind": "runtime.refreshCue"})
        assert planner.calls == 1 and session.progress.index == 1
        await session.handle({"kind": "runtime.planControl", "action": "pause"})
        assert session.cue is None
        await session.close()
        assert session.progress is None and source.closed

    asyncio.run(scenario())


def test_missing_target_replans_once_then_pauses_without_model_storm():
    class Planner:
        calls = 0

        async def plan(self, *args):
            self.calls += 1
            return TeachingPlan(steps=[step()])

    class MissingSource(Source):
        missing = False

        async def observe(self, target, include_image=True):
            result = await super().observe(target, include_image)
            return replace(result, elements=()) if self.missing else result

    async def scenario():
        source, planner = MissingSource(), Planner()
        session = TeachingSession(source)
        session.agent = planner
        session.target = fresh().target
        await session.handle({"kind": "runtime.ask", "question": "Help", "locale": "en"})
        source.missing = True
        session.progress.started -= 11
        await session.handle({"kind": "runtime.refreshCue"})
        assert planner.calls == 2
        session.progress.started -= 11
        for _ in range(3):
            await session.handle({"kind": "runtime.refreshCue"})
        assert planner.calls == 2 and session.progress.status == "paused"
        assert session.cue is None
        await session.close()

    asyncio.run(scenario())
