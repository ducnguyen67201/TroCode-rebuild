"""Agents SDK computer adapter for one pinned window and no other native authority."""

from __future__ import annotations

import asyncio
from collections.abc import Awaitable, Callable
from typing import Any

from agents import AsyncComputer, Button, Environment
from cua_driver import (
    ActionTarget,
    ClickButton,
    ClickInput,
    ClickPosition,
    DragInput,
    InputDeliveryMode,
    MoveCursorInput,
    PressKeyInput,
    ScrollBy,
    ScrollDirection,
    ScrollInput,
    TypeTextInput,
)

from tro_runtime.observation_source import CuaObservationSource
from tro_runtime.observations import Observation, Target

MAX_ACTIONS = 12
MAX_TYPED_CHARACTERS = 1_000

Confirm = Callable[[str], Awaitable[None]]


class SelectedWindowComputer(AsyncComputer):
    def __init__(
        self,
        source: CuaObservationSource,
        target: Target,
        confirm: Confirm,
        cancelled: asyncio.Event,
    ) -> None:
        self.source = source
        self.target = target
        self.confirm = confirm
        self.cancelled = cancelled
        self.actions_used = 0
        self._last: Observation | None = None
        self._focused_editable = False

    @property
    def environment(self) -> Environment:
        return "mac" if __import__("sys").platform == "darwin" else "windows"

    @property
    def dimensions(self) -> tuple[int, int]:
        return (round(self.target.bounds.width), round(self.target.bounds.height))

    async def screenshot(self) -> str:
        observation = await self._fresh()
        if observation.image is None:
            raise ValueError("Selected-window screenshot is unavailable.")
        return observation.image.split(",", 1)[1]

    async def click(self, x: int, y: int, button: Button) -> None:
        observation = await self._before_mutation()
        element = self._element_at(observation, x, y)
        risky = element is None or self._risky(element.label, element.role)
        if risky:
            await self.confirm("Click an unknown or consequential control")
        await self._driver_call(
            "click",
            ClickInput(
                target=self._target(),
                position=ClickPosition.COORDINATES(*self._point(x, y)),
                delivery_mode=InputDeliveryMode.FOREGROUND,
                session=None,
                button={
                    "left": ClickButton.LEFT,
                    "right": ClickButton.RIGHT,
                    "wheel": ClickButton.MIDDLE,
                }.get(button, ClickButton.LEFT),
                count=1,
            ),
        )
        self._focused_editable = element is not None and not risky and self._editable(element.role)

    async def double_click(self, x: int, y: int) -> None:
        await self.click(x, y, "left")
        await self.click(x, y, "left")

    async def scroll(self, x: int, y: int, scroll_x: int, scroll_y: int) -> None:
        await self._before_mutation()
        if scroll_y:
            direction = ScrollDirection.DOWN if scroll_y > 0 else ScrollDirection.UP
            amount = abs(scroll_y)
        else:
            direction = ScrollDirection.RIGHT if scroll_x > 0 else ScrollDirection.LEFT
            amount = abs(scroll_x)
        px, py = self._point(x, y)
        await self._driver_call(
            "scroll",
            ScrollInput(
                x=px,
                y=py,
                direction=direction,
                target=self._target(),
                scope=None,
                session=None,
                by=ScrollBy.LINE,
                amount=min(amount, 100),
            ),
        )

    async def type(self, text: str) -> None:
        await self._before_mutation()
        if not text or len(text) > MAX_TYPED_CHARACTERS:
            raise ValueError("Typed text is outside the allowed bound.")
        # CUA does not expose a focused flag, so only a prior verified click on a
        # known non-sensitive editable element can establish safe local focus.
        if not self._focused_editable:
            await self.confirm("Type text into the currently focused field")
        await self._driver_call(
            "type_text",
            TypeTextInput(text=text, target=self._target(), scope=None, session=None),
        )

    async def wait(self) -> None:
        self._check_cancelled()
        await asyncio.sleep(0.25)

    async def move(self, x: int, y: int) -> None:
        await self._before_mutation()
        px, py = self._point(x, y)
        await self._driver_call(
            "move_cursor",
            MoveCursorInput(x=px, y=py, target=self._target(), scope=None, session=None),
        )

    async def keypress(self, keys: list[str]) -> None:
        await self._before_mutation()
        normalized = [key.lower() for key in keys]
        if len(keys) > 1 or any(
            key in normalized for key in ("meta", "cmd", "win", "alt", "option", "ctrl")
        ):
            raise ValueError("Keyboard shortcuts are not permitted.")
        if "enter" in normalized or "return" in normalized:
            await self.confirm("Use a keyboard shortcut or submit a form")
        await self._driver_call(
            "press_key",
            PressKeyInput(
                key=keys[0], target=self._target(), scope=None, session=None, modifiers=None
            ),
        )
        self._focused_editable = False

    async def drag(self, path: list[tuple[int, int]]) -> None:
        await self._before_mutation()
        if len(path) < 2 or len(path) > 100:
            raise ValueError("Invalid drag path.")
        start = self._point(*path[0])
        end = self._point(*path[-1])
        await self._driver_call(
            "drag",
            DragInput(
                from_x=start[0],
                from_y=start[1],
                to_x=end[0],
                to_y=end[1],
                target=self._target(),
                scope=None,
                session=None,
                duration_ms=300,
                steps=min(len(path), 100),
                button=ClickButton.LEFT,
                modifier=None,
            ),
        )

    async def _before_mutation(self) -> Observation:
        self._check_cancelled()
        if self.actions_used >= MAX_ACTIONS:
            raise ValueError("Action limit reached.")
        observation = await self._fresh()
        self.actions_used += 1
        return observation

    async def _fresh(self) -> Observation:
        self._check_cancelled()
        observation = await self.source.observe(self.target, include_image=True)
        if (
            observation.target.pid != self.target.pid
            or observation.target.window_id != self.target.window_id
            or observation.target.bounds != self.target.bounds
        ):
            raise ValueError("Selected window changed.")
        self._last = observation
        return observation

    async def _driver_call(self, method: str, value: object) -> None:
        self._check_cancelled()
        driver = await self.source.action_driver(self.target)
        operation = getattr(driver, method)
        await asyncio.wait_for(operation(value), 5)
        self._check_cancelled()

    def _target(self) -> ActionTarget:
        return ActionTarget.WINDOW(self.target.pid, self.target.window_id)

    def _point(self, x: int, y: int) -> tuple[float, float]:
        width, height = self.dimensions
        if x < 0 or y < 0 or x >= width or y >= height:
            raise ValueError("Action coordinate is outside the selected window.")
        return (self.target.bounds.x + x, self.target.bounds.y + y)

    def _element_at(self, observation: Observation, x: int, y: int) -> Any | None:
        px, py = self._point(x, y)
        for element in observation.elements:
            bounds = element.bounds
            if (
                bounds.x <= px <= bounds.x + bounds.width
                and bounds.y <= py <= bounds.y + bounds.height
            ):
                return element
        return None

    @staticmethod
    def _risky(label: str, role: str) -> bool:
        value = f"{label} {role}".casefold()
        return any(
            word in value
            for word in (
                "send",
                "post",
                "upload",
                "buy",
                "purchase",
                "delete",
                "password",
                "secure",
                "permission",
                "setting",
            )
        )

    @staticmethod
    def _editable(role: str) -> bool:
        value = role.casefold().replace("_", "").replace("-", "")
        return any(word in value for word in ("textfield", "textarea", "textbox", "edit"))

    def _check_cancelled(self) -> None:
        if self.cancelled.is_set():
            raise asyncio.CancelledError
