"""Bounded private stdio, with control admission independent of observation work."""

import asyncio
import os
import sys
import threading
from typing import Any, BinaryIO

from tro_runtime.protocol import MAX_FRAME_BYTES, encode_message, parse_message
from tro_runtime.runtime import Runtime
from tro_runtime.teaching import TeachingSession

TEACHING_REQUESTS = frozenset(
    {
        "runtime.listTargets",
        "runtime.selectTarget",
        "runtime.observe",
        "runtime.explain",
        "runtime.check",
        "runtime.presentationAck",
        "runtime.ask",
        "runtime.configure",
        "runtime.refreshCue",
        "runtime.planControl",
        "runtime.prepareInstruction",
        "runtime.startGuidance",
    }
)


def safe_failure_details(kind: str) -> tuple[str, str]:
    """Return a diagnostic category that never includes native/provider details."""
    if kind == "runtime.prepareInstruction":
        return (
            "NOT_READY",
            "Selected-window observation is unavailable. Check observation permissions.",
        )
    if kind == "runtime.executeInstruction":
        return (
            "NOT_READY",
            "The selected-window action could not start.",
        )
    return (
        "NOT_READY",
        "Observation or guidance is unavailable. Retry explicitly.",
    )


async def serve_async(source: BinaryIO, destination: BinaryIO) -> int:
    runtime = Runtime()
    teaching: TeachingSession
    loop = asyncio.get_running_loop()
    ordinary: asyncio.Queue[dict[str, Any]] = asyncio.Queue(32)
    controls: asyncio.Queue[dict[str, Any] | None] = asyncio.Queue(4)
    failed = False
    active: asyncio.Task[None] | None = None

    def write(value: dict[str, Any]) -> None:
        destination.write(encode_message(value))
        destination.flush()

    teaching = TeachingSession()

    def failure(request: dict[str, Any], code: str, message: str) -> dict[str, Any]:
        return {
            **{
                key: request[key]
                for key in ("protocolVersion", "requestId", "correlationId", "generationId")
            },
            "kind": "runtime.error",
            "code": code,
            "message": message,
            "retryable": False,
        }

    def admit(request: dict[str, Any] | None, invalid: bool = False) -> None:
        nonlocal failed
        failed |= invalid
        if (
            request is not None
            and request["kind"] not in TEACHING_REQUESTS
            and request["kind"] not in ("runtime.stop", "runtime.shutdown")
        ):
            write(runtime.handle(request))
            return
        queue = (
            controls
            if request is None or request["kind"] in ("runtime.stop", "runtime.shutdown")
            else ordinary
        )
        try:
            queue.put_nowait(request)  # type: ignore[arg-type]
        except asyncio.QueueFull:
            if request is not None:
                write(failure(request, "BUSY", "Runtime queue is full."))
            else:
                failed = True
                # EOF must not be dropped behind ordinary work.
                controls.get_nowait()
                controls.put_nowait(None)

    def read() -> None:
        # Read the OS pipe directly. A daemon blocked on sys.stdin.buffer can hold
        # its buffered-reader lock during interpreter shutdown and abort Python.
        try:
            descriptor = source.fileno()
        except (AttributeError, OSError):
            descriptor = None
        pending = bytearray()

        def read_frame() -> bytes:
            if descriptor is None:
                return source.readline(MAX_FRAME_BYTES + 2)
            while True:
                newline = pending.find(b"\n")
                if newline >= 0:
                    frame = bytes(pending[: newline + 1])
                    del pending[: newline + 1]
                    return frame
                if len(pending) > MAX_FRAME_BYTES:
                    raise ValueError("Invalid frame")
                chunk = os.read(descriptor, min(4096, MAX_FRAME_BYTES + 1 - len(pending)))
                if not chunk:
                    return bytes(pending)
                pending.extend(chunk)

        try:
            while True:
                frame = read_frame()
                if not frame:
                    loop.call_soon_threadsafe(admit, None)
                    return
                if not frame.endswith(b"\n") or len(frame) - 1 > MAX_FRAME_BYTES:
                    raise ValueError("Invalid frame")
                request = parse_message(frame[:-1])
                loop.call_soon_threadsafe(admit, request)
                # Bound scheduled callbacks as well as the asyncio queues.
                fence = threading.Event()
                loop.call_soon_threadsafe(fence.set)
                fence.wait()
        except (ValueError, OSError):
            loop.call_soon_threadsafe(admit, None, True)
        except RuntimeError:
            pass  # Event loop already closed after shutdown.

    async def handle(request: dict[str, Any]) -> None:
        if request["kind"] not in TEACHING_REQUESTS:
            write(runtime.handle(request))
            return
        if runtime.generation != request["generationId"] or runtime.session_id is None:
            write(failure(request, "NOT_READY", "Start the current teaching session first."))
            return
        try:
            if request["kind"] == "runtime.configure":
                teaching.configure(runtime.account_id, runtime.session_id, request)
                write(
                    {
                        **{
                            key: request[key]
                            for key in (
                                "protocolVersion",
                                "requestId",
                                "correlationId",
                                "generationId",
                            )
                        },
                        "kind": "runtime.configured",
                    }
                )
                return
            if request["kind"] == "runtime.prepareInstruction":
                prepared = await teaching.prepare_instruction(request)
                write(
                    {
                        **{
                            key: request[key]
                            for key in (
                                "protocolVersion",
                                "requestId",
                                "correlationId",
                                "generationId",
                            )
                        },
                        "kind": "runtime.instructionPrepared",
                        "prepared": prepared,
                    }
                )
                return
            if request["kind"] == "runtime.startGuidance":
                await teaching.start_guidance(request)
                write(
                    {
                        **{
                            key: request[key]
                            for key in (
                                "protocolVersion",
                                "requestId",
                                "correlationId",
                                "generationId",
                            )
                        },
                        "kind": "runtime.guidanceStarted",
                        "state": teaching.projection(runtime.session_id),
                    }
                )
                return
            kind = await teaching.handle(request)
            write(
                {
                    **{
                        key: request[key]
                        for key in ("protocolVersion", "requestId", "correlationId", "generationId")
                    },
                    "kind": "runtime." + kind,
                    "state": teaching.projection(runtime.session_id),
                }
            )
        except asyncio.CancelledError:
            raise
        except Exception:
            # Native errors can contain screen text, paths, or application titles.
            code, message = safe_failure_details(request["kind"])
            write(failure(request, code, message))

    threading.Thread(target=read, daemon=True, name="tro-stdin").start()
    control_wait = asyncio.create_task(controls.get())
    ordinary_wait: asyncio.Task[dict[str, Any]] | None = None
    try:
        while not runtime.closed:
            if active is None and ordinary_wait is None:
                ordinary_wait = asyncio.create_task(ordinary.get())
            waiters = {control_wait}
            if active is not None:
                waiters.add(active)
            if ordinary_wait is not None:
                waiters.add(ordinary_wait)
            done, _ = await asyncio.wait(waiters, return_when=asyncio.FIRST_COMPLETED)
            if control_wait in done:
                request = control_wait.result()
                if active:
                    active.cancel()
                    await asyncio.gather(active, return_exceptions=True)
                    active = None
                if ordinary_wait:
                    ordinary_wait.cancel()
                    await asyncio.gather(ordinary_wait, return_exceptions=True)
                    ordinary_wait = None
                while not ordinary.empty():
                    queued = ordinary.get_nowait()
                    write(failure(queued, "NOT_READY", "Request cancelled by stop."))
                if request is None:
                    break
                write(runtime.handle(request))
                await asyncio.wait_for(teaching.close(), 2)
                control_wait = asyncio.create_task(controls.get())
            elif active is not None and active in done:
                active.result()
                active = None
            elif ordinary_wait is not None and ordinary_wait in done:
                active = asyncio.create_task(handle(ordinary_wait.result()))
                ordinary_wait = None
    except (ValueError, OSError, TimeoutError):
        failed = True
    finally:
        tasks = [task for task in (active, control_wait, ordinary_wait) if task is not None]
        for task in tasks:
            task.cancel()
        await asyncio.gather(*tasks, return_exceptions=True)
        try:
            await asyncio.wait_for(teaching.close(), 2)
        except Exception:
            failed = True
    return 2 if failed else 0


def serve(source: BinaryIO, destination: BinaryIO) -> int:
    return asyncio.run(serve_async(source, destination))


if __name__ == "__main__":
    raise SystemExit(serve(sys.stdin.buffer, sys.stdout.buffer))
