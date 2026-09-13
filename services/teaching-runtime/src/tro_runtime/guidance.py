"""Pure visual guidance and evidence rules; these values cannot execute input."""

from dataclasses import dataclass
from typing import Literal
from uuid import uuid4

from tro_runtime.observations import Observation, Rect

Gesture = Literal["point", "click", "drag", "type", "scroll"]


@dataclass(frozen=True)
class Cue:
    id: str
    observation_id: str
    element_id: str
    gesture: Gesture
    caption: str
    locale: Literal["en", "vi"]
    source: Rect
    destination: Rect | None
    direction: Literal["up", "down", "left", "right"] | None
    expires_at: float
    grounding: str = "accessibility"
    screenshot_id: str | None = None


def make_cue(
    observation: Observation,
    element_id: str,
    gesture: Gesture,
    caption: str,
    locale: Literal["en", "vi"],
    now: float,
    destination_id: str | None = None,
    direction: Literal["up", "down", "left", "right"] | None = None,
    visual_bounds: tuple[Rect, Rect | None] | None = None,
) -> Cue:
    if now < observation.captured_at or now - observation.captured_at > 1:
        raise ValueError("Observe the window again before showing guidance.")
    if not caption.strip() or len(caption) > 400 or locale not in ("en", "vi"):
        raise ValueError("Invalid guidance caption.")
    if gesture not in ("point", "click", "drag", "type", "scroll"):
        raise ValueError("Only visual gestures are supported.")
    if (gesture == "drag") != (destination_id is not None):
        raise ValueError("A visual drag requires a grounded destination.")
    if (gesture == "scroll") != (direction is not None):
        raise ValueError("A scroll cue requires a direction.")
    if direction is not None and direction not in ("up", "down", "left", "right"):
        raise ValueError("Invalid scroll direction.")
    if visual_bounds is not None:
        if observation.image is None:
            raise ValueError("Visual guidance requires a fresh screenshot.")
        source, destination = visual_bounds
        if (gesture == "drag") != (destination is not None):
            raise ValueError("Visual drag requires a destination rectangle.")
    else:
        source = observation.element(element_id).bounds
        destination = observation.element(destination_id).bounds if destination_id else None
    if not observation.target.bounds.contains(source) or (
        destination is not None and not observation.target.bounds.contains(destination)
    ):
        raise ValueError("Guidance must remain within the observed window.")
    return Cue(
        str(uuid4()),
        observation.id,
        element_id,
        gesture,
        caption.strip(),
        locale,
        source,
        destination,
        direction,
        observation.captured_at + 1,
        "visual" if visual_bounds is not None else "accessibility",
        observation.screenshot_id if visual_bounds is not None else None,
    )


@dataclass(frozen=True)
class CheckResult:
    outcome: Literal["confirmed", "mismatch", "unknown"]
    source: Literal["fresh_observation"]
    observation_id: str
    checked_at: float
    message: str


def check_expected_value(
    before: Observation, after: Observation, label: str, expected: str
) -> CheckResult:
    """Confirm a named postcondition, never mastery or who caused the change."""
    outcome: Literal["confirmed", "mismatch", "unknown"] = "unknown"
    message = "There is insufficient fresh evidence to confirm this change."
    same_window = (before.target.pid, before.target.window_id) == (
        after.target.pid,
        after.target.window_id,
    )
    matches = [element for element in after.elements if element.label == label]
    if (
        same_window
        and after.id != before.id
        and after.captured_at > before.captured_at
        and after.complete
        and len(matches) == 1
    ):
        outcome = "confirmed" if matches[0].value == expected else "mismatch"
        message = (
            "The expected state is visible. This does not establish mastery."
            if outcome == "confirmed"
            else "The expected state is not visible yet."
        )
    return CheckResult(outcome, "fresh_observation", after.id, after.captured_at, message)
