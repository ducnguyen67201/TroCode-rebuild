import asyncio
from types import SimpleNamespace

import pytest
from test_guidance import observation

from tro_runtime.agent import GuidanceAgent, GuidanceProposal
from tro_runtime.model_client import create_model


def test_sdk_has_no_executable_tools_and_validates_grounding(monkeypatch):
    async def run(agent, *args, **kwargs):
        assert agent.tools == []
        assert kwargs["max_turns"] == 4
        assert kwargs["run_config"].tracing_disabled
        return SimpleNamespace(
            final_output={
                "element_id": "1",
                "gesture": "click",
                "caption": "Click it yourself.",
                "destination_id": None,
                "direction": None,
            }
        )

    monkeypatch.setattr("tro_runtime.agent.Runner.run", run)
    result = asyncio.run(GuidanceAgent("unused").explain(observation(), "Where do I click?", "en"))
    assert result.element_id == "1"
    with pytest.raises(ValueError):
        GuidanceProposal.model_validate(
            {
                "element_id": "1",
                "gesture": "execute",
                "caption": "Act",
                "destination_id": None,
                "direction": None,
            }
        )


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
