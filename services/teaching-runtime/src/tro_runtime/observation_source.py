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
    from cua_driver import CuaDriver, WindowStateOutput

from tro_runtime.observations import Element, Observation, Rect, Target

MAX_MODEL_IMAGE_URL_CHARS = 750_000
SCREENSHOT_DIMENSIONS = (768, 512, 360)


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

    async def frontmost_target(self) -> Target:
        targets = await self.list_targets()
        if not targets:
            raise ValueError("No eligible external window is available.")
        return targets[0]

    async def list_targets(self) -> tuple[Target, ...]:
        from cua_driver import ListWindowsInput

        driver = await self._connect(None)
        result = await asyncio.wait_for(
            driver.list_windows(ListWindowsInput(pid=None, on_screen_only=True)), 10
        )
        targets: list[tuple[int, Target]] = []
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
                    (
                        window.z_index if window.z_index is not None else 2**31 - 1,
                        Target(
                            window.pid,
                            window.window_id,
                            _text(window.title),
                            Rect(bounds.x, bounds.y, bounds.width, bounds.height),
                        ),
                    )
                )
            except ValueError:
                continue
            if len(targets) == 100:
                break
        targets.sort(key=lambda item: item[0])
        return tuple(target for _, target in targets)

    async def observe(self, target: Target, include_image: bool = True) -> Observation:
        from cua_driver import GetWindowStateInput

        driver = await self._connect(target)

        async def capture(accessibility: bool, max_dimension: int) -> WindowStateOutput:
            return await driver.get_window_state(
                GetWindowStateInput(
                    pid=target.pid,
                    window_id=target.window_id,
                    session=None,
                    query=None,
                    include_accessibility_tree=accessibility,
                    include_screenshot=include_image,
                    screenshot_out_file=None,
                    max_elements=200,
                    max_depth=20,
                    max_dimension=max_dimension,
                )
            )

        accessibility_available = True

        async def capture_available(max_dimension: int) -> WindowStateOutput:
            nonlocal accessibility_available
            try:
                return await capture(accessibility_available, max_dimension)
            except Exception:
                if not include_image or not accessibility_available:
                    raise
                # Screen-only guidance remains possible when AX access is unavailable.
                accessibility_available = False
                return await capture(False, max_dimension)

        async def capture_model_input() -> tuple[WindowStateOutput, str | None]:
            dimensions = SCREENSHOT_DIMENSIONS if include_image else SCREENSHOT_DIMENSIONS[:1]
            for max_dimension in dimensions:
                candidate = await capture_available(max_dimension)
                candidate_image = self._image_url(candidate) if include_image else None
                if candidate_image is None or len(candidate_image) <= MAX_MODEL_IMAGE_URL_CHARS:
                    return candidate, candidate_image
            raise ValueError("Selected-window image exceeds model limit.")

        result, image = await asyncio.wait_for(capture_model_input(), 10)
        bounds = result.window_bounds
        if result.pid != target.pid or result.window_id != target.window_id or bounds is None:
            raise ValueError("Selected window is unavailable.")
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
        if (
            include_image
            and image is not None
            and result.screenshot_frame_valid
            and result.screenshot_width
            and result.screenshot_height
        ):
            # Reject mismatched/cropped aspect ratios before normalized image mapping.
            width, height = result.screenshot_width, result.screenshot_height
            if abs(width - height * current.bounds.width / current.bounds.height) > 2:
                raise ValueError("Screenshot geometry does not match the selected window.")
        return Observation(
            str(uuid4()),
            current,
            time.time(),
            tuple(elements),
            result.elements_complete is True and not result.truncated and not result.degraded,
            image,
        )

    @staticmethod
    def _image_url(result: WindowStateOutput) -> str | None:
        if (
            not result.screenshot_frame_valid
            or not result.screenshot_width
            or not result.screenshot_height
            or not result.images
        ):
            return None
        snapshot = result.images[0]
        if snapshot.mime_type not in ("image/png", "image/jpeg"):
            raise ValueError("Invalid selected-window image.")
        try:
            base64.b64decode(snapshot.data_base64, validate=True)
        except (ValueError, TypeError):
            raise ValueError("Invalid selected-window image.") from None
        return f"data:{snapshot.mime_type};base64,{snapshot.data_base64}"

    async def close(self) -> None:
        try:
            if self._driver is not None:
                await self._driver.shutdown()
                self._driver = None
        finally:
            self._directory.cleanup()
