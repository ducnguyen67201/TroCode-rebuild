import asyncio
import json
from types import SimpleNamespace

import pytest
from test_guidance import observation

from tro_runtime.agent import GuidanceAgent
from tro_runtime.model_client import create_model


@pytest.mark.parametrize(
    "origin",
    [
        "http://example.com",
        "https://user:pass@example.com",
        "https://example.com/path",
        "https://example.com?redirect=evil",
    ],
)
def test_private_model_origin_is_bounded(origin):
    with pytest.raises(ValueError):
        create_model(origin, "grant", "model")


def test_private_model_allows_exact_loopback_http_only_when_native_debug_enables_it(
    monkeypatch,
):
    origin = "http://127.0.0.1:4319"
    monkeypatch.delenv("TRO_RUNTIME_ALLOW_LOOPBACK_HTTP", raising=False)
    with pytest.raises(ValueError):
        create_model(origin, "grant", "model")

    monkeypatch.setenv("TRO_RUNTIME_ALLOW_LOOPBACK_HTTP", "1")
    assert create_model(origin, "grant", "model").model == "model"
    with pytest.raises(ValueError):
        create_model("http://localhost:4319", "grant", "model")


def test_planner_is_bounded_structured_and_observation_only(monkeypatch):
    async def run(agent, *args, **kwargs):
        assert [tool.name for tool in agent.tools] == [
            "show_student_where",
            "show_student_click",
            "show_student_drag",
            "show_student_type",
            "show_student_scroll",
        ]
        assert agent.model_settings.max_tokens == 1024
        assert agent.model_settings.parallel_tool_calls is False
        assert kwargs["max_turns"] == 4
        assert kwargs["run_config"].tracing_disabled
        tool = agent.tools[1]
        await tool.on_invoke_tool(
            SimpleNamespace(tool_name=tool.name),
            json.dumps(
                {
                    "target": {"role": "button", "label": "Counter"},
                    "caption": "Click the counter yourself.",
                    "expected": None,
                }
            ),
        )
        return SimpleNamespace(final_output="Done")

    monkeypatch.setattr("tro_runtime.agent.Runner.run", run)
    result = asyncio.run(GuidanceAgent("unused").plan(observation(), "Help me", "en"))
    assert len(result.steps) == 1


def test_planner_rejects_a_model_run_without_cursor_calls(monkeypatch):
    from tro_runtime.errors import GuidanceError

    async def run(*args, **kwargs):
        return SimpleNamespace(final_output="No tool calls")

    monkeypatch.setattr("tro_runtime.agent.Runner.run", run)
    with pytest.raises(GuidanceError, match="could not be grounded"):
        asyncio.run(GuidanceAgent("unused").plan(observation(), "Help me", "en"))


def test_planner_closes_sdk_behavior_errors(monkeypatch):
    from tro_runtime.errors import GuidanceError

    async def run(*args, **kwargs):
        raise RuntimeError("provider details and screen text")

    monkeypatch.setattr("tro_runtime.agent.Runner.run", run)
    with pytest.raises(GuidanceError, match="could not be grounded") as failure:
        asyncio.run(GuidanceAgent("unused").plan(observation(), "Help me", "en"))
    assert "provider details" not in str(failure.value)
