import json
from pathlib import Path

import pytest

from tro_runtime.protocol import MAX_FRAME_BYTES, parse_message

CORPUS = json.loads(
    (Path(__file__).parents[3] / "tests/fixtures/contracts/corpus.json").read_text()
)


@pytest.mark.parametrize("case", CORPUS, ids=lambda case: case["name"])
def test_corpus(case):
    frame = json.dumps(case["value"]).encode()
    if case["valid"]:
        assert parse_message(frame) == case["value"]
    else:
        with pytest.raises(ValueError, match="Invalid runtime message"):
            parse_message(frame)


@pytest.mark.parametrize("frame", [b"", b"\xff", b" " * (MAX_FRAME_BYTES + 1)])
def test_bad_frame(frame):
    with pytest.raises(ValueError):
        parse_message(frame)
