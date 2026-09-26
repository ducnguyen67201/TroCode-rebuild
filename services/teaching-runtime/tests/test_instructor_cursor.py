import asyncio
import json
from types import SimpleNamespace

import pytest
from test_guidance import observation

from tro_runtime.instructor_cursor import InstructorCursor
from tro_runtime.planning import Selector, VisualTarget


def selector(label="Counter"):
    return Selector(role="button", label=label)


def test_catalog_is_fixed_strict_and_contains_no_typed_value_argument():
    cursor = InstructorCursor(observation(), "en")
    tools = cursor.tools()
    assert [tool.name for tool in tools] == [
        "show_student_where",
        "show_student_click",
        "show_student_drag",
        "show_student_type",
        "show_student_scroll",
    ]
    assert all(tool.strict_json_schema for tool in tools)
    type_schema = next(
        tool.params_json_schema for tool in tools if tool.name == "show_student_type"
    )
    assert "value" not in type_schema["properties"]
    assert type_schema["additionalProperties"] is False


def test_methods_stage_all_visual_gestures_without_mutating_observation():
    evidence = observation()
    before = repr(evidence)
    cursor = InstructorCursor(evidence, "en")
    cursor.show_student_where(selector(), "Look here")
    cursor.show_student_drag(selector(), selector("Destination"), "Drag this yourself")
    cursor.show_student_scroll(selector(), "down", "Scroll down yourself")
    plan = cursor.finish()
    assert [step.gesture for step in plan.steps] == ["point", "drag", "scroll"]
    assert repr(evidence) == before


@pytest.mark.parametrize(
    "name,arguments,gesture",
    [
        (
            "show_student_where",
            {"target": {"role": "button", "label": "Counter"}},
            "point",
        ),
        (
            "show_student_click",
            {"target": {"role": "button", "label": "Counter"}},
            "click",
        ),
        (
            "show_student_drag",
            {
                "source": {"role": "button", "label": "Counter"},
                "destination": {"role": "group", "label": "Destination"},
            },
            "drag",
        ),
        (
            "show_student_type",
            {"target": {"role": "button", "label": "Counter"}},
            "type",
        ),
        (
            "show_student_scroll",
            {
                "target": {"role": "button", "label": "Counter"},
                "direction": "down",
            },
            "scroll",
        ),
    ],
)
def test_wrapped_handlers_stage_steps_without_changing_observation(name, arguments, gesture):
    async def scenario():
        evidence = observation()
        before = repr(evidence)
        cursor = InstructorCursor(evidence, "en")
        tool = next(tool for tool in cursor.tools() if tool.name == name)
        result = await tool.on_invoke_tool(
            SimpleNamespace(tool_name=tool.name),
            json.dumps({**arguments, "caption": "Try this yourself", "expected": None}),
        )
        assert result == {"staged": True, "step": 1}
        assert cursor.finish().steps[0].gesture == gesture
        assert repr(evidence) == before

    asyncio.run(scenario())


def test_visual_target_requires_image_for_the_first_step():
    cursor = InstructorCursor(observation(), "en")
    cursor.show_student_where(
        VisualTarget(description="Canvas shape", x=0.1, y=0.1, width=0.2, height=0.2),
        "Look at the shape",
    )
    with pytest.raises(ValueError, match="first plan target"):
        cursor.finish()


def test_empty_fourth_and_invalid_inputs_fail_closed():
    cursor = InstructorCursor(observation(), "en")
    with pytest.raises(ValueError, match="no teaching steps"):
        cursor.finish()
    for _ in range(3):
        cursor.show_student_click(selector(), "Click here yourself")
    with pytest.raises(ValueError, match="at most three"):
        cursor.show_student_click(selector(), "Another step")
    assert len(cursor.finish().steps) == 3
    with pytest.raises(ValueError):
        InstructorCursor(observation(), "fr")
