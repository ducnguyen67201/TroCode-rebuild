import asyncio

import pytest

from tro_runtime.computer import SelectedWindowComputer
from tro_runtime.observations import Rect, Target


async def _confirm(_: str) -> None:
    return None


def computer() -> SelectedWindowComputer:
    return SelectedWindowComputer(
        object(),  # type: ignore[arg-type]
        Target(4, 8, "Practice", Rect(-100, 20, 800, 600)),
        _confirm,
        asyncio.Event(),
    )


def test_coordinates_stay_inside_pinned_window() -> None:
    value = computer()
    assert value.dimensions == (800, 600)
    assert value._point(0, 0) == (-100, 20)
    assert value._point(799, 599) == (699, 619)
    with pytest.raises(ValueError):
        value._point(800, 1)


def test_risk_terms_fail_closed() -> None:
    assert SelectedWindowComputer._risky("Send", "button")
    assert SelectedWindowComputer._risky("", "secure text field")
    assert not SelectedWindowComputer._risky("Next tab", "button")
    assert SelectedWindowComputer._editable("AXTextField")
    assert SelectedWindowComputer._editable("text-box")
    assert not SelectedWindowComputer._editable("button")
