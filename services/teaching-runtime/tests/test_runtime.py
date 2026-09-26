import io
import json
import subprocess
import sys
from uuid import uuid4

import pytest

from tro_runtime.__main__ import safe_failure_details, serve
from tro_runtime.protocol import DIGEST, MAX_FRAME_BYTES, encode_message
from tro_runtime.runtime import Runtime


def request(kind, **extra):
    return {
        "protocolVersion": 3,
        "kind": "runtime." + kind,
        "requestId": str(uuid4()),
        "correlationId": str(uuid4()),
        "generationId": "33333333-3333-4333-8333-333333333333",
        **extra,
    }


def initialize(runtime):
    return runtime.handle(request("initialize", schemaDigest=DIGEST, accountId=None))


def test_lifecycle():
    runtime = Runtime()
    assert runtime.handle(request("health"))["code"] == "NOT_READY"
    ready = initialize(runtime)
    assert ready["kind"] == "runtime.ready"
    assert ready["capabilities"] == ["diagnostic", "instructor_cursor"]
    assert initialize(runtime)["code"] == "BUSY"
    session = str(uuid4())
    assert runtime.handle(request("start", sessionId=session))["kind"] == "runtime.started"
    assert runtime.handle(request("start", sessionId=str(uuid4())))["code"] == "BUSY"
    assert runtime.handle(request("health"))["state"] == "running"
    assert runtime.handle(request("stop"))["kind"] == "runtime.stopped"
    assert runtime.handle(request("stop"))["kind"] == "runtime.stopped"
    assert runtime.handle(request("shutdown"))["kind"] == "runtime.shutdownComplete"
    assert runtime.handle(request("health"))["code"] == "NOT_READY"


def test_mismatch_and_generation():
    runtime = Runtime()
    assert (
        runtime.handle(request("initialize", schemaDigest="a" * 64, accountId=None))["code"]
        == "PROTOCOL_MISMATCH"
    )
    initialize(runtime)
    assert runtime.handle(request("health", generationId=str(uuid4())))["code"] == "NOT_READY"
    assert runtime.handle(request("stopped"))["code"] == "INVALID_MESSAGE"


def test_failure_diagnostics_use_protocol_code_and_identify_safe_runtime_stage():
    assert safe_failure_details("runtime.prepareInstruction") == (
        "NOT_READY",
        "Selected-window observation is unavailable. Check observation permissions.",
    )
    assert safe_failure_details("runtime.executeInstruction") == (
        "NOT_READY",
        "The selected-window action could not start.",
    )
    assert safe_failure_details("runtime.ask")[0] == "NOT_READY"


def test_real_process_and_eof():
    frames = [request("initialize", schemaDigest=DIGEST, accountId=None), request("health")]
    result = subprocess.run(
        [sys.executable, "-m", "tro_runtime"],
        input=b"".join(map(encode_message, frames)),
        capture_output=True,
        timeout=5,
    )
    assert result.returncode == 0
    assert json.loads(result.stdout.splitlines()[1])["state"] == "ready"
    assert result.stderr == b""


def test_invalid_input_is_bounded_and_private():
    for frame in [b"secret", b"secret\n", b" " * (MAX_FRAME_BYTES + 2)]:
        output = io.BytesIO()
        assert serve(io.BytesIO(frame), output) == 2
        assert output.getvalue() == b""


def test_shutdown_exits_cleanly_while_parent_keeps_stdin_open():
    child = subprocess.Popen(
        [sys.executable, "-m", "tro_runtime"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    try:
        child.stdin.write(
            encode_message(request("initialize", schemaDigest=DIGEST, accountId=None))
        )
        child.stdin.flush()
        assert json.loads(child.stdout.readline())["kind"] == "runtime.ready"
        child.stdin.write(encode_message(request("shutdown")))
        child.stdin.flush()
        assert json.loads(child.stdout.readline())["kind"] == "runtime.shutdownComplete"
        assert child.wait(timeout=5) == 0
        assert child.stderr.read() == b""
    finally:
        child.kill()
        child.wait()
        child.stdin.close()
        child.stdout.close()
        child.stderr.close()


@pytest.mark.parametrize(
    "kind,payload", [("observe", {}), ("ask", {"question": "Help", "locale": "en"})]
)
def test_stop_cancels_pending_teaching_and_never_publishes_a_late_result(
    monkeypatch, kind, payload
):
    import asyncio
    import os

    from tro_runtime.__main__ import serve_async
    from tro_runtime.teaching import TeachingSession

    async def scenario():
        entered = asyncio.Event()
        cancelled = asyncio.Event()

        class HungTeaching(TeachingSession):
            async def handle(self, request):
                entered.set()
                try:
                    await asyncio.Event().wait()
                finally:
                    cancelled.set()

        monkeypatch.setattr("tro_runtime.__main__.TeachingSession", HungTeaching)
        reader, writer = os.pipe()
        source = os.fdopen(reader, "rb", buffering=0)
        output = io.BytesIO()
        server = asyncio.create_task(serve_async(source, output))
        try:
            frames = [
                request("initialize", schemaDigest=DIGEST, accountId=None),
                request("start", sessionId=str(uuid4())),
                request(kind, **payload),
            ]
            os.write(writer, b"".join(map(encode_message, frames)))
            await asyncio.wait_for(entered.wait(), 1)
            os.write(writer, encode_message(request("stop")) + encode_message(request("shutdown")))
            assert await asyncio.wait_for(server, 1) == 0
            assert cancelled.is_set()
            replies = [json.loads(line)["kind"] for line in output.getvalue().splitlines()]
            assert replies == [
                "runtime.ready",
                "runtime.started",
                "runtime.stopped",
                "runtime.shutdownComplete",
            ]
        finally:
            os.close(writer)
            source.close()
            if not server.done():
                server.cancel()
                await asyncio.gather(server, return_exceptions=True)

    asyncio.run(scenario())


def test_worker_bootstrap_does_not_import_model_sdk():
    import subprocess
    import sys

    result = subprocess.run(
        [
            sys.executable,
            "-c",
            "import sys; import tro_runtime.__main__; "
            "assert 'agents' not in sys.modules; assert 'openai' not in sys.modules",
        ],
        capture_output=True,
        timeout=5,
    )
    assert result.returncode == 0, result.stderr.decode()
