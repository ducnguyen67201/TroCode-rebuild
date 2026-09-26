"""Application-owned tools that stage visual teaching steps, never native input."""

from __future__ import annotations

from typing import Annotated, Literal

from agents import FunctionTool, function_tool
from pydantic import Field

from tro_runtime.observations import Observation
from tro_runtime.planning import (
    PlannedStep,
    Postcondition,
    Selector,
    TeachingPlan,
    VisualTarget,
)

CursorTarget = Selector | VisualTarget
Caption = Annotated[str, Field(min_length=1, max_length=400)]
ToolResult = dict[str, bool | int]


class InstructorCursor:
    """Collect a bounded, atomic plan from presentation-only function tools."""

    MAX_STEPS = 3

    def __init__(self, observation: Observation, locale: Literal["en", "vi"]) -> None:
        if locale not in ("en", "vi"):
            raise ValueError("Unsupported guidance locale.")
        self._observation = observation
        self._locale = locale
        self._steps: list[PlannedStep] = []

    @property
    def locale(self) -> Literal["en", "vi"]:
        return self._locale

    def _stage(self, step: PlannedStep) -> ToolResult:
        if len(self._steps) >= self.MAX_STEPS:
            raise ValueError("A walkthrough can contain at most three steps.")
        self._steps.append(step)
        return {"staged": True, "step": len(self._steps)}

    def show_student_where(
        self,
        target: CursorTarget,
        caption: Caption,
        expected: Postcondition | None = None,
    ) -> ToolResult:
        """Point to and circle a target the learner should look at."""
        return self._stage(
            PlannedStep(
                target=target,
                gesture="point",
                caption=caption,
                destination=None,
                direction=None,
                expected=expected,
            )
        )

    def show_student_click(
        self,
        target: CursorTarget,
        caption: Caption,
        expected: Postcondition | None = None,
    ) -> ToolResult:
        """Show a ghost click; the learner performs the real click."""
        return self._stage(
            PlannedStep(
                target=target,
                gesture="click",
                caption=caption,
                destination=None,
                direction=None,
                expected=expected,
            )
        )

    def show_student_drag(
        self,
        source: CursorTarget,
        destination: CursorTarget,
        caption: Caption,
        expected: Postcondition | None = None,
    ) -> ToolResult:
        """Show a ghost trajectory; the learner performs the real drag."""
        return self._stage(
            PlannedStep(
                target=source,
                gesture="drag",
                caption=caption,
                destination=destination,
                direction=None,
                expected=expected,
            )
        )

    def show_student_type(
        self,
        target: CursorTarget,
        caption: Caption,
        expected: Postcondition | None = None,
    ) -> ToolResult:
        """Point to a field without receiving or inserting the learner's value."""
        return self._stage(
            PlannedStep(
                target=target,
                gesture="type",
                caption=caption,
                destination=None,
                direction=None,
                expected=expected,
            )
        )

    def show_student_scroll(
        self,
        target: CursorTarget,
        direction: Literal["up", "down", "left", "right"],
        caption: Caption,
        expected: Postcondition | None = None,
    ) -> ToolResult:
        """Show a scroll direction; the learner performs the real scroll."""
        return self._stage(
            PlannedStep(
                target=target,
                gesture="scroll",
                caption=caption,
                destination=None,
                direction=direction,
                expected=expected,
            )
        )

    def tools(self) -> tuple[FunctionTool, ...]:
        """Return the closed model-visible catalog in stable teaching order."""

        @function_tool(name_override="show_student_where", failure_error_function=None)
        def show_student_where(
            target: CursorTarget,
            caption: Caption,
            expected: Postcondition | None = None,
        ) -> ToolResult:
            """Point to and circle a target the learner should look at."""
            return self.show_student_where(target, caption, expected)

        @function_tool(name_override="show_student_click", failure_error_function=None)
        def show_student_click(
            target: CursorTarget,
            caption: Caption,
            expected: Postcondition | None = None,
        ) -> ToolResult:
            """Show a ghost click; the learner performs the real click."""
            return self.show_student_click(target, caption, expected)

        @function_tool(name_override="show_student_drag", failure_error_function=None)
        def show_student_drag(
            source: CursorTarget,
            destination: CursorTarget,
            caption: Caption,
            expected: Postcondition | None = None,
        ) -> ToolResult:
            """Show a ghost drag path; the learner performs the real drag."""
            return self.show_student_drag(source, destination, caption, expected)

        @function_tool(name_override="show_student_type", failure_error_function=None)
        def show_student_type(
            target: CursorTarget,
            caption: Caption,
            expected: Postcondition | None = None,
        ) -> ToolResult:
            """Point to a field without receiving or inserting a typed value."""
            return self.show_student_type(target, caption, expected)

        @function_tool(name_override="show_student_scroll", failure_error_function=None)
        def show_student_scroll(
            target: CursorTarget,
            direction: Literal["up", "down", "left", "right"],
            caption: Caption,
            expected: Postcondition | None = None,
        ) -> ToolResult:
            """Show a scroll direction; the learner performs the real scroll."""
            return self.show_student_scroll(target, direction, caption, expected)

        return (
            show_student_where,
            show_student_click,
            show_student_drag,
            show_student_type,
            show_student_scroll,
        )

    def finish(self) -> TeachingPlan:
        if not self._steps:
            raise ValueError("The walkthrough contains no teaching steps.")
        plan = TeachingPlan(steps=list(self._steps))
        if plan.steps[0].target.resolve(self._observation) is None:
            raise ValueError("The first plan target is not uniquely observed.")
        return plan
