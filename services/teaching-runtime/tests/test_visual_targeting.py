import asyncio
import json
import time
from dataclasses import replace
from types import SimpleNamespace
from uuid import uuid4

import pytest
from test_guidance import observation

from tro_runtime.agent import GuidanceAgent
from tro_runtime.errors import GuidanceError
from tro_runtime.planning import PlannedStep, PlanProgress, TeachingPlan, VisualTarget
from tro_runtime.teaching import TeachingSession


def screenshot(image="same-window-image"):
    return replace(
        observation(captured=time.time(), identifier=str(uuid4()), complete=False),
        elements=(),
        image=image,
    )


def visual_step(gesture="click", destination=None):
    return PlannedStep.model_validate(
        {
            "target": {
                "description": "Canvas shape",
                "x": 0.2,
                "y": 0.25,
                "width": 0.1,
                "height": 0.1,
            },
            "gesture": gesture,
            "caption": "Select this shape yourself.",
            "destination": destination,
            "direction": None,
            "expected": None,
        }
    )


def test_visual_region_maps_without_accessibility_and_preserves_frame_identity():
    original = screenshot()
    progress = PlanProgress(TeachingPlan(steps=[visual_step()]), "en", original)
    current = screenshot()
    cue = progress.observe(current)
    assert cue.source.x == 0  # Window origin -100 + 20% of 500.
    assert cue.source.y == 100
    assert cue.source.width == 50
    assert cue.grounding == "visual" and cue.screenshot_id == current.id
    assert current.elements == ()  # Never masquerade as an accessibility element.
    assert progress.status == "awaiting_confirmation"
    assert progress.observe(screenshot()).id == cue.id


@pytest.mark.parametrize("change", ["image", "bounds"])
def test_visual_coordinates_invalidate_on_scroll_resize_or_window_motion(change):
    original = screenshot()
    progress = PlanProgress(TeachingPlan(steps=[visual_step()]), "en", original)
    assert progress.observe(screenshot()) is not None
    current = screenshot("different-screen" if change == "image" else "same-window-image")
    if change == "bounds":
        current = replace(
            current, target=replace(current.target, bounds=replace(current.target.bounds, x=50))
        )
    assert progress.observe(current) is None
    assert progress.status == "paused" and progress.needs_replan


def test_missing_image_does_not_trigger_model_recovery():
    progress = PlanProgress(TeachingPlan(steps=[visual_step()]), "en", screenshot())
    assert progress.observe(screenshot(None)) is None
    assert progress.status == "paused" and not progress.needs_replan


@pytest.mark.parametrize(
    "field,value",
    [("x", -0.1), ("x", float("nan")), ("width", 0), ("width", 1.1), ("height", float("inf"))],
)
def test_visual_regions_reject_invalid_geometry(field, value):
    data = visual_step().target.model_dump()
    data[field] = value
    with pytest.raises(ValueError):
        VisualTarget.model_validate(data)


def test_visual_drag_maps_both_regions_and_projection_omits_image():
    async def scenario():
        frame = screenshot()
        destination = {"description": "Drop zone", "x": 0.6, "y": 0.5, "width": 0.1, "height": 0.1}
        progress = PlanProgress(TeachingPlan(steps=[visual_step("drag", destination)]), "en", frame)
        cue = progress.observe(frame)
        assert cue.destination.x == 200
        session = TeachingSession()
        session.observation, session.cue = frame, cue
        projected = session.projection(str(uuid4()))
        assert "image" not in projected["observation"]
        assert projected["observation"]["screenshot_id"] == frame.id

    asyncio.run(scenario())


def test_sdk_can_propose_visual_guidance_for_an_unlabelled_canvas(monkeypatch):
    async def run(agent, *args, **kwargs):
        assert [tool.name for tool in agent.tools] == [
            "show_student_where",
            "show_student_click",
            "show_student_drag",
            "show_student_type",
            "show_student_scroll",
        ]
        if isinstance(args[0], list):
            assert args[0][0]["content"][1]["type"] == "input_image"
        tool = agent.tools[1]
        await tool.on_invoke_tool(
            SimpleNamespace(tool_name=tool.name),
            json.dumps(
                {
                    "target": visual_step().target.model_dump(),
                    "caption": "Click the shape yourself.",
                    "expected": None,
                }
            ),
        )
        return SimpleNamespace(final_output="staged")

    monkeypatch.setattr("tro_runtime.agent.Runner.run", run)
    result = asyncio.run(GuidanceAgent("unused").plan(screenshot(), "Select a shape", "en"))
    assert isinstance(result.steps[0].target, VisualTarget)
    with pytest.raises(GuidanceError, match="could not be grounded"):
        asyncio.run(GuidanceAgent("unused").plan(screenshot(None), "Select a shape", "en"))


def test_capture_can_fall_back_to_screen_only_and_checks_image_geometry(monkeypatch):
    import base64
    import sys

    from tro_runtime.observation_source import CuaObservationSource

    monkeypatch.setitem(
        sys.modules,
        "cua_driver",
        SimpleNamespace(GetWindowStateInput=lambda **kwargs: SimpleNamespace(**kwargs)),
    )

    async def scenario():
        calls = []
        output = SimpleNamespace(
            pid=42,
            window_id=7,
            window_bounds=SimpleNamespace(x=-100, y=0, width=500, height=400),
            window_title="Canvas",
            degraded=True,
            elements=[],
            elements_complete=False,
            truncated=False,
            screenshot_frame_valid=True,
            screenshot_width=1000,
            screenshot_height=800,
            images=[
                SimpleNamespace(
                    mime_type="image/png", data_base64=base64.b64encode(b"test-image").decode()
                )
            ],
        )

        async def capture(request):
            calls.append(request.include_accessibility_tree)
            if request.include_accessibility_tree:
                raise ValueError("AX unavailable")
            return output

        async def connect(target):
            return SimpleNamespace(get_window_state=capture)

        source = CuaObservationSource()
        monkeypatch.setattr(source, "_connect", connect)
        result = await source.observe(observation().target)
        assert calls == [True, False]
        assert result.image is not None and not result.complete and not result.elements
        output.screenshot_width = 800
        with pytest.raises(ValueError, match="geometry"):
            await source.observe(observation().target)

    asyncio.run(scenario())


def test_capture_retries_at_a_smaller_dimension_to_fit_model_gateway(monkeypatch):
    import base64
    import sys

    from tro_runtime.observation_source import CuaObservationSource

    monkeypatch.setitem(
        sys.modules,
        "cua_driver",
        SimpleNamespace(GetWindowStateInput=lambda **kwargs: SimpleNamespace(**kwargs)),
    )

    async def scenario():
        calls = []

        async def capture(request):
            calls.append(request.max_dimension)
            raw_size = 600_000 if request.max_dimension == 768 else 400_000
            width = request.max_dimension
            height = round(width * 4 / 5)
            return SimpleNamespace(
                pid=42,
                window_id=7,
                window_bounds=SimpleNamespace(x=-100, y=0, width=500, height=400),
                window_title="Canvas",
                degraded=False,
                elements=[],
                elements_complete=True,
                truncated=False,
                screenshot_frame_valid=True,
                screenshot_width=width,
                screenshot_height=height,
                images=[
                    SimpleNamespace(
                        mime_type="image/png",
                        data_base64=base64.b64encode(b"x" * raw_size).decode(),
                    )
                ],
            )

        async def connect(target):
            return SimpleNamespace(get_window_state=capture)

        source = CuaObservationSource()
        monkeypatch.setattr(source, "_connect", connect)
        result = await source.observe(observation().target)
        assert calls == [768, 512]
        assert result.image is not None and len(result.image) < 750_000

    asyncio.run(scenario())


def test_visual_action_can_complete_from_ax_without_replanning_on_first_changed_frame():
    from tro_runtime.planning import Postcondition

    step = visual_step()
    step.expected = Postcondition.model_validate(
        {"target": {"role": "button", "label": "Counter"}, "value": "1"}
    )

    def frame(value, image):
        return replace(observation(value, time.time(), str(uuid4())), image=image)

    progress = PlanProgress(TeachingPlan(steps=[step]), "en", frame("0", "before"))
    assert progress.observe(frame("0", "before")) is not None
    assert progress.observe(frame("1", "after")) is None
    assert progress.index == 0 and not progress.needs_replan
    assert progress.observe(frame("1", "after")) is None
    assert progress.status == "completed" and not progress.needs_replan
