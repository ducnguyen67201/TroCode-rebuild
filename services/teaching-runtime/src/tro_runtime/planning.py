"""Semantic plans and a deterministic, observation-only step controller."""

import time
from dataclasses import replace
from typing import Literal
from uuid import uuid4

from pydantic import BaseModel, ConfigDict, Field, model_validator

from tro_runtime.guidance import Cue, make_cue
from tro_runtime.observations import Element, Observation


class Selector(BaseModel):
    model_config = ConfigDict(extra="forbid")
    role: str = Field(min_length=1, max_length=80)
    label: str = Field(min_length=1, max_length=160)

    def resolve(self, observation: Observation) -> Element | None:
        matches = [e for e in observation.elements if (e.role, e.label) == (self.role, self.label)]
        return matches[0] if len(matches) == 1 else None


class Postcondition(BaseModel):
    model_config = ConfigDict(extra="forbid")
    target: Selector
    value: str = Field(max_length=160)

    def matches(self, observation: Observation) -> bool | None:
        element = self.target.resolve(observation)
        return element.value == self.value if element is not None else None


class PlannedStep(BaseModel):
    model_config = ConfigDict(extra="forbid")
    target: Selector
    gesture: Literal["point", "click", "drag", "type", "scroll"]
    caption: str = Field(min_length=1, max_length=400)
    destination: Selector | None
    direction: Literal["up", "down", "left", "right"] | None
    expected: Postcondition | None

    @model_validator(mode="after")
    def validate_gesture(self) -> "PlannedStep":
        if (self.gesture == "drag") != (self.destination is not None):
            raise ValueError("Drag needs a destination.")
        if (self.gesture == "scroll") != (self.direction is not None):
            raise ValueError("Scroll needs a direction.")
        return self


class TeachingPlan(BaseModel):
    model_config = ConfigDict(extra="forbid")
    steps: list[PlannedStep] = Field(min_length=1, max_length=3)


class PlanProgress:
    """One bounded plan. No model calls, native actions, or attribution of learning."""

    def __init__(self, plan: TeachingPlan, locale: Literal["en", "vi"], observation: Observation):
        self.id = str(uuid4())
        self.cue_id = str(uuid4())
        self.needs_replan = False
        self.plan = plan
        self.locale = locale
        self.window = (observation.target.pid, observation.target.window_id)
        self.index = 0
        self.status = "running"
        self.message = "Follow the guidance. Tro checks visible progress automatically."
        self.armed = False
        self.matches = 0
        self.last_time = 0.0
        self.last_id = ""
        self.started = time.monotonic()
        self.shown = False

    def projection(self) -> dict[str, object]:
        return {
            "id": self.id,
            "index": self.index,
            "status": self.status,
            "message": self.message,
            "steps": [s.caption for s in self.plan.steps],
        }

    def pause(self, message: str = "Paused. Resume when ready or request a revised plan.") -> None:
        self.status = "paused"
        self.message = message
        self.matches = 0

    def control(self, action: str) -> None:
        if action == "pause" and self.status != "completed":
            self.pause()
        elif action == "resume" and self.status == "paused":
            self.status = "running"
            self.armed = False
            self.shown = False
            self.started = time.monotonic()
            self.needs_replan = False
        elif action == "confirm" and self.status == "awaiting_confirmation" and self.shown:
            self.advance()
        else:
            raise ValueError("This plan control is unavailable.")

    def advance(self) -> None:
        self.index += 1
        self.cue_id = str(uuid4())
        self.armed = False
        self.matches = 0
        self.shown = False
        self.started = time.monotonic()
        self.status = "completed" if self.index == len(self.plan.steps) else "running"
        self.message = (
            "Prepared steps finished. This does not establish mastery."
            if self.status == "completed"
            else "Continue with the next instruction."
        )

    def observe(self, observation: Observation) -> Cue | None:
        if self.status in ("paused", "completed"):
            return None
        if (observation.target.pid, observation.target.window_id) != self.window:
            self.pause("The selected window changed. Request a revised plan.")
            return None
        if (
            not observation.complete
            or observation.id == self.last_id
            or observation.captured_at <= self.last_time
            or not 0 <= time.time() - observation.captured_at <= 1
        ):
            self.matches = 0
            return None
        self.last_id, self.last_time = observation.id, observation.captured_at
        step = self.plan.steps[self.index]
        if self.shown and step.expected is not None:
            matches = step.expected.matches(observation)
            if matches is False:
                self.armed = True
            self.matches = self.matches + 1 if matches is True and self.armed else 0
            if self.matches >= 2:
                self.advance()
                if self.status == "completed":
                    return None
                step = self.plan.steps[self.index]
        source = step.target.resolve(observation)
        destination = step.destination.resolve(observation) if step.destination else None
        if source is None or (step.destination is not None and destination is None):
            self.message = "Waiting for the expected control to become visible."
            if time.monotonic() - self.started > 10:
                self.pause("The expected control is unavailable. Request a revised plan.")
                self.needs_replan = True
            return None
        if not self.shown:
            self.armed = step.expected is not None and step.expected.matches(observation) is False
            self.status = "running" if self.armed else "awaiting_confirmation"
            self.message = (
                "Watching for the expected visible change."
                if self.armed
                else "Automatic confirmation is uncertain. Continue when you are ready."
            )
        cue = make_cue(
            observation,
            source.id,
            step.gesture,
            step.caption,
            self.locale,
            time.time(),
            destination.id if destination else None,
            step.direction,
        )
        self.shown = True
        return replace(cue, id=self.cue_id)
