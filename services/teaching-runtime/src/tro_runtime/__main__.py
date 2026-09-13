"""Portable bounded stdio entry point. Stdout is exclusively protocol frames."""

import sys
from typing import BinaryIO

from tro_runtime.protocol import MAX_FRAME_BYTES, encode_message, parse_message
from tro_runtime.runtime import Runtime


def serve(source: BinaryIO, destination: BinaryIO) -> int:
    runtime = Runtime()
    while not runtime.closed:
        frame = source.readline(MAX_FRAME_BYTES + 2)
        if not frame:
            return 0  # Parent EOF is shutdown, never a reconnect trigger.
        if not frame.endswith(b"\n") or len(frame) - 1 > MAX_FRAME_BYTES:
            return 2
        try:
            reply = runtime.handle(parse_message(frame[:-1]))
            destination.write(encode_message(reply))
            destination.flush()
        except (ValueError, BrokenPipeError, OSError):
            return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(serve(sys.stdin.buffer, sys.stdout.buffer))
