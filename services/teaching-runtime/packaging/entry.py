"""Frozen worker entry and content-free native asset self-check."""

import asyncio
import json
import sys

from tro_runtime.__main__ import serve
from tro_runtime.observation_source import CuaObservationSource
from tro_runtime.protocol import DIGEST


async def self_check() -> int:
    source = CuaObservationSource()
    try:
        await source._connect(None)
        print(json.dumps({"schemaDigest": DIGEST, "nativePolicyLoaded": True}))
        return 0
    except Exception:
        print("Runtime native self-check failed.", file=sys.stderr)
        return 2
    finally:
        await source.close()


if __name__ == "__main__":
    raise SystemExit(
        asyncio.run(self_check())
        if sys.argv[1:] == ["--self-check"]
        else serve(sys.stdin.buffer, sys.stdout.buffer)
    )
