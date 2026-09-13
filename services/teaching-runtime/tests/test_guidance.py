from dataclasses import replace

import pytest

from tro_runtime.guidance import check_expected_value, make_cue
from tro_runtime.observations import Element, Observation, Rect, Target


def observation(value="0", captured=100, identifier="first", complete=True):
    target = Target(42, 7, "Practice", Rect(-100, 0, 500, 400))
    return Observation(
        identifier,
        target,
        captured,
        (
            Element("1", "Counter", "button", Rect(-50, 20, 80, 40), value),
            Element("2", "Destination", "group", Rect(200, 200, 80, 80), ""),
        ),
        complete,
    )


@pytest.mark.parametrize(
    "gesture,destination,direction",
    [
        ("point", None, None),
        ("click", None, None),
        ("drag", "2", None),
        ("type", None, None),
        ("scroll", None, "down"),
    ],
)
def test_visual_gestures_are_grounded_values(gesture, destination, direction):
    evidence = observation()
    cue = make_cue(evidence, "1", gesture, "Try it yourself", "en", 100.2, destination, direction)
    assert cue.source == evidence.elements[0].bounds
    assert cue.expires_at == 101
    assert evidence.elements[0].value == "0"  # No effects, including repeated cues.


def test_stale_and_invented_targets_fail_closed():
    for element, now, destination in [("invented", 100, None), ("1", 102, None), ("1", 100, "2")]:
        with pytest.raises(ValueError):
            make_cue(observation(), element, "click", "Try it", "en", now, destination)
    with pytest.raises(ValueError):
        make_cue(observation(), "1", "drag", "Drag", "en", 100)
    with pytest.raises(ValueError):
        Rect(float("nan"), 0, 10, 10)


def test_checks_distinguish_facts_and_uncertainty():
    before = observation()
    assert (
        check_expected_value(before, observation("1", 101, "next"), "Counter", "1").outcome
        == "confirmed"
    )
    assert (
        check_expected_value(before, observation("0", 101, "next"), "Counter", "1").outcome
        == "mismatch"
    )
    for after in [
        before,
        observation("1", 101, "next", False),
        replace(observation("1", 101, "next"), target=replace(before.target, pid=43)),
    ]:
        assert check_expected_value(before, after, "Counter", "1").outcome == "unknown"
