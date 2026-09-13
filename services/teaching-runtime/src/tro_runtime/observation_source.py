"""The only CUA boundary: list windows, read one window, close.

The immutable bounded manifest intersects native user/managed policies. No policy
file is replaced and no unrestricted mode or generic tool dispatcher is exposed.
"""

from __future__ import annotations

import asyncio
import base64
import json
import os
import tempfile
import time
from importlib.resources import files
from pathlib import Path
from typing import TYPE_CHECKING, Protocol
from uuid import uuid4

if TYPE_CHECKING:
    from cua_driver import CuaDriver

from tro_runtime.observations import Element, Observation, Rect, Target


def _text(value: str | None, limit: int = 256) -> str:
    return (value or "").encode("utf-8")[:limit].decode("utf-8", errors="ignore")


class ObservationSource(Protocol):
    async def list_targets(self) -> tuple[Target, ...]: ...
    async def observe(self, target: Target, include_image: bool = True) -> Observation: ...
    async def close(self) -> None: ...


class CuaObservationSource:
    def __init__(self) -> None:
        self._driver: CuaDriver | None = None
        self._scope: tuple[int, int] | None = None
        self._directory = tempfile.TemporaryDirectory(prefix="tro-observation-")
        os.environ["CUA_DRIVER_RS_TELEMETRY_ENABLED"] = "false"
        os.environ["CUA_DRIVER_RS_UPDATE_CHECK"] = "false"

    async def _connect(self, target: Target | None) -> CuaDriver:
        from cua_driver import (
            ConfiguredDriverOptions,
            CuaDriver,
            RuntimeAuthorizationOptions,
            SessionPermissionMode,
        )

        scope = (target.pid, target.window_id) if target else None
        if self._driver is not None and self._scope == scope:
            return self._driver
        if self._driver is not None:
            await self._driver.shutdown()
            self._driver = None
        manifest = json.loads(files("tro_runtime").joinpath("resources/read-only.json").read_text())
        if target:
            manifest["resources"] = {
                "desktop": {
                    "windows": [{"pid": target.pid, "window_id": target.window_id}],
                    "display": False,
                }
            }
        path = Path(self._directory.name) / "read-only.json"
        path.write_text(json.dumps(manifest), encoding="utf-8")
        path.chmod(0o600)
        options = ConfiguredDriverOptions(
            claude_code_compatibility=False,
            authorization=RuntimeAuthorizationOptions(
                allowed_modes=[SessionPermissionMode.BOUNDED],
                compatibility_mode=SessionPermissionMode.BOUNDED,
                compatibility_bounded_manifest_path=str(path),
                unrestricted_acknowledged=False,
                max_session_ttl_seconds=3600,
                max_idle_ttl_seconds=900,
            ),
        )
        # The worker process is the cancellation boundary for native initialization.
        self._driver = CuaDriver.create_configured(options)
        self._scope = scope
        return self._driver

    async def list_targets(self) -> tuple[Target, ...]:
        from cua_driver import ListWindowsInput

        driver = await self._connect(None)
        result = await asyncio.wait_for(
            driver.list_windows(ListWindowsInput(pid=None, on_screen_only=True)), 10
        )
        targets = []
        for window in result.windows:
            if (
                window.pid is None
                or window.pid in (os.getpid(), os.getppid())
                or window.minimized
                or not window.is_on_screen
                or window.on_current_space is False
            ):
                continue
            bounds = window.bounds
            try:
                targets.append(
                    Target(
                        window.pid,
                        window.window_id,
                        _text(window.title),
                        Rect(bounds.x, bounds.y, bounds.width, bounds.height),
                    )
                )
            except ValueError:
                continue
            if len(targets) == 100:
                break
        return tuple(targets)

    async def observe(self, target: Target, include_image: bool = True) -> Observation:
        from cua_driver import GetWindowStateInput

        driver = await self._connect(target)
        result = await asyncio.wait_for(
            driver.get_window_state(
                GetWindowStateInput(
                    pid=target.pid,
                    window_id=target.window_id,
                    session=None,
                    query=None,
                    include_accessibility_tree=True,
                    include_screenshot=include_image,
                    screenshot_out_file=None,
                    max_elements=200,
                    max_depth=20,
                    max_dimension=1200,
                )
            ),
            10,
        )
        bounds = result.window_bounds
        if (
            result.pid != target.pid
            or result.window_id != target.window_id
            or bounds is None
            or result.degraded
        ):
            raise ValueError("Selected window is unavailable or observation is degraded.")
        current = Target(
            target.pid,
            target.window_id,
            _text(result.window_title or target.title),
            Rect(bounds.x, bounds.y, bounds.width, bounds.height),
        )
        elements = []
        for item in (result.elements or [])[:200]:
            frame = item.frame
            if frame is None:
                continue
            try:
                rect = Rect(frame.x, frame.y, frame.w, frame.h)
            except ValueError:
                continue
            if not current.bounds.contains(rect):
                continue
            elements.append(
                Element(
                    str(item.element_index),
                    _text(item.label),
                    _text(item.role, 80),
                    rect,
                    _text(item.value),
                )
            )
        image = None
        if include_image and result.screenshot_frame_valid:
            for snapshot in result.images[:1]:
                if (
                    snapshot.mime_type not in ("image/png", "image/jpeg")
                    or len(snapshot.data_base64) > 5_592_408
                ):
                    raise ValueError("Invalid selected-window image.")
                if len(base64.b64decode(snapshot.data_base64, validate=True)) > 4 * 1024 * 1024:
                    raise ValueError("Selected-window image exceeds limit.")
                image = f"data:{snapshot.mime_type};base64,{snapshot.data_base64}"
        return Observation(
            str(uuid4()),
            current,
            time.time(),
            tuple(elements),
            result.elements_complete is True and not result.truncated,
            image,
        )

    async def close(self) -> None:
        try:
            if self._driver is not None:
                await self._driver.shutdown()
                self._driver = None
        finally:
            self._directory.cleanup()
