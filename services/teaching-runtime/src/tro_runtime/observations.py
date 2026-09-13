"""Bounded observation values. Screen content is untrusted evidence, never instructions."""

import math
from dataclasses import dataclass, field


@dataclass(frozen=True)
class Rect:
    x: float
    y: float
    width: float
    height: float

    def __post_init__(self) -> None:
        if (
            not all(
                math.isfinite(v) and abs(v) <= 100_000
                for v in (self.x, self.y, self.width, self.height)
            )
            or self.width <= 0
            or self.height <= 0
        ):
            raise ValueError("Invalid observation geometry.")

    def contains(self, other: "Rect") -> bool:
        return (
            self.x <= other.x
            and self.y <= other.y
            and other.x + other.width <= self.x + self.width
            and other.y + other.height <= self.y + self.height
        )


@dataclass(frozen=True)
class Target:
    pid: int
    window_id: int
    title: str
    bounds: Rect

    def __post_init__(self) -> None:
        if self.pid <= 0 or self.window_id < 0 or len(self.title) > 256:
            raise ValueError("Invalid window target.")


@dataclass(frozen=True)
class Element:
    id: str
    label: str
    role: str
    bounds: Rect
    value: str


@dataclass(frozen=True)
class Observation:
    id: str
    target: Target
    captured_at: float
    elements: tuple[Element, ...]
    complete: bool
    image: str | None = field(default=None, repr=False, compare=False)

    def element(self, element_id: str) -> Element:
        matches = [item for item in self.elements if item.id == element_id]
        if len(matches) != 1:
            raise ValueError("Target is not uniquely grounded in this observation.")
        return matches[0]
