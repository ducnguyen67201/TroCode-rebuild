"""Strict wire validation; no domain state or I/O dispatch."""

import json
from importlib.resources import files
from typing import Any

from jsonschema import Draft7Validator
from jsonschema.exceptions import ValidationError

MAX_FRAME_BYTES = 256 * 1024
SCHEMA = json.loads(files("tro_runtime").joinpath("schema.json").read_text())
DIGEST = files("tro_runtime").joinpath("digest.txt").read_text()
VALIDATOR = Draft7Validator(SCHEMA)


def parse_message(frame: bytes) -> dict[str, Any]:
    if not frame or len(frame) > MAX_FRAME_BYTES:
        raise ValueError("Invalid runtime message")
    try:
        value = json.loads(frame)
        VALIDATOR.validate(value)
    except (ValueError, UnicodeError, ValidationError) as error:
        # Validation details can contain private input; never propagate them.
        raise ValueError("Invalid runtime message") from error
    if not isinstance(value, dict):
        raise ValueError("Invalid runtime message")
    return value


def encode_message(value: dict[str, Any]) -> bytes:
    frame = json.dumps(value, ensure_ascii=False, separators=(",", ":")).encode()
    parse_message(frame)
    return frame + b"\n"
