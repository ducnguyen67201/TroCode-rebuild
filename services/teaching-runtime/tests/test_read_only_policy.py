import asyncio
import json
import sys
from importlib.resources import files
from pathlib import Path

import pytest

from tro_runtime.observation_source import CuaObservationSource


def test_product_ceiling_has_only_observation():
    manifest = json.loads(files("tro_runtime").joinpath("resources/read-only.json").read_text())
    assert manifest["mode"] == "bounded"
    assert set(manifest["allow"]["tools"]) == {"list_apps", "list_windows", "get_window_state"}
    assert manifest["resources"] == {}
    assert not any(
        hasattr(CuaObservationSource, name)
        for name in [
            "click",
            "drag",
            "type",
            "scroll",
            "call_tool",
            "launch_app",
            "focus_window",
            "action_driver",
        ]
    )


def test_runtime_source_and_resources_contain_no_native_mutation_authority():
    root = Path(__file__).parents[1] / "src" / "tro_runtime"
    forbidden = (
        "ComputerTool",
        "AsyncComputer",
        "ClickInput",
        "DragInput",
        "TypeTextInput",
        "ScrollInput",
        "PressKeyInput",
        "MoveCursorInput",
        "action_driver",
        "window-control.json",
    )
    for path in root.rglob("*"):
        if path.is_file() and path.suffix in {".py", ".json"}:
            content = path.read_text(encoding="utf-8")
            assert not any(token in content for token in forbidden), path
    assert not (root / "resources" / "window-control.json").exists()


@pytest.mark.skipif(
    sys.platform not in ("darwin", "win32"), reason="Native CUA supports target desktop platforms"
)
def test_native_manifest_denies_mutation_dispatch_without_live_targets():
    async def scenario():
        source = CuaObservationSource()
        try:
            driver = await source._connect(None)
            # The SDK inventory is canonical, not an authorization projection. Exercise
            # dispatch with an impossible process target; no live application is addressed.
            for name in ("click", "drag", "type_text", "scroll", "launch_app", "bring_to_front"):
                result = await driver.call_tool(
                    name,
                    json.dumps(
                        {
                            "pid": 2147483647,
                            "window_id": 2147483647,
                            "x": -10000000,
                            "y": -10000000,
                            "app": "/nonexistent/tro-denied-policy-probe",
                        }
                    ),
                )
                assert result.is_error
                assert "permission" in result.text.lower() or "manifest" in result.text.lower()

        finally:
            await source.close()

    asyncio.run(scenario())


def test_native_text_is_bounded_by_wire_bytes_without_broken_unicode():
    from tro_runtime.observation_source import _text

    assert len(_text("😀" * 256).encode("utf-8")) <= 256
    assert _text(None) == ""
