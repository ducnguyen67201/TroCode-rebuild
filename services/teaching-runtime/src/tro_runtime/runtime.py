"""Pure lifecycle transitions. Teaching I/O is owned by the asynchronous controller."""

from typing import Any

from tro_runtime.protocol import DIGEST


class Runtime:
    def __init__(self) -> None:
        self.generation: str | None = None
        self.account_id: str | None = None
        self.session_id: str | None = None
        self.state = "ready"
        self.closed = False

    def handle(self, request: dict[str, Any]) -> dict[str, Any]:
        reply = {
            key: request[key]
            for key in ("protocolVersion", "requestId", "correlationId", "generationId")
        }

        def error(code: str, message: str) -> dict[str, Any]:
            return {
                **reply,
                "kind": "runtime.error",
                "code": code,
                "message": message,
                "retryable": False,
            }

        kind = request["kind"]
        if self.closed:
            return error("NOT_READY", "Runtime is closed.")
        if kind == "runtime.initialize":
            if self.generation is not None:
                return error("BUSY", "Runtime is already initialized.")
            if request["schemaDigest"] != DIGEST:
                return error("PROTOCOL_MISMATCH", "Runtime protocol is incompatible.")
            self.generation = request["generationId"]
            self.account_id = request["accountId"]
            return {
                **reply,
                "kind": "runtime.ready",
                "schemaDigest": DIGEST,
                "capabilities": ["diagnostic", "selected_window_actions"],
            }
        if self.generation is None or self.generation != request["generationId"]:
            return error("NOT_READY", "Initialize the current runtime first.")
        if kind == "runtime.health":
            return {**reply, "kind": "runtime.healthResult", "state": self.state}
        if kind == "runtime.start":
            if self.session_id is not None and self.session_id != request["sessionId"]:
                return error("BUSY", "A teaching session is already running.")
            self.session_id = request["sessionId"]
            self.state = "running"
            return {**reply, "kind": "runtime.started", "sessionId": self.session_id}
        if kind == "runtime.stop":
            self.session_id = None
            self.state = "stopped"
            return {**reply, "kind": "runtime.stopped"}
        if kind == "runtime.shutdown":
            self.session_id = None
            self.closed = True
            return {**reply, "kind": "runtime.shutdownComplete"}
        return error("INVALID_MESSAGE", "Unexpected runtime message direction.")
