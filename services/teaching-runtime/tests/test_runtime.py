import io
import json
import subprocess
import sys
from uuid import uuid4

from tro_runtime.__main__ import serve
from tro_runtime.protocol import DIGEST, MAX_FRAME_BYTES, encode_message
from tro_runtime.runtime import Runtime


def request(kind, **extra):
    return {
        "protocolVersion": 1,
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
    assert initialize(runtime)["kind"] == "runtime.ready"
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
