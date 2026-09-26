import asyncio
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
    from test_planning import step

    async def run(agent, *args, **kwargs):
        assert agent.tools == []
        assert agent.model_settings.max_tokens == 1024
        assert kwargs["run_config"].tracing_disabled
        return SimpleNamespace(final_output={"steps": [step().model_dump()]})

    monkeypatch.setattr("tro_runtime.agent.Runner.run", run)
    result = asyncio.run(GuidanceAgent("unused").plan(observation(), "Help me", "en"))
    assert len(result.steps) == 1
