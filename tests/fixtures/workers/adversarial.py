"""Test-only subprocess. Never used by the application executable resolver."""
import json
import os
import sys
import time
from tro_runtime.runtime import Runtime
from tro_runtime.protocol import encode_message
runtime = Runtime()
mode = sys.argv[1]
for line in sys.stdin.buffer:
    request = json.loads(line)
    kind = request['kind']
    if kind == 'runtime.initialize' and mode == 'boot-hang':
        time.sleep(30)
        continue
    if kind == 'runtime.health' and mode == 'crash':
        sys.exit(5)
    if kind == 'runtime.health' and mode == 'oversize':
        sys.stdout.buffer.write(b' ' * (256 * 1024 + 2))
        sys.stdout.buffer.flush()
        continue
    if kind == 'runtime.shutdown' and mode == 'hang':
        time.sleep(30)
        continue
    reply = runtime.handle(request)
    if mode == 'environment' and kind == 'runtime.health' and 'CARGO_MANIFEST_DIR' in os.environ:
        reply = {**request, 'kind': 'runtime.error', 'code': 'INTERNAL', 'message': 'Unfiltered environment', 'retryable': False}

    if kind == 'runtime.initialize' and mode == 'digest':
        reply['schemaDigest'] = 'a' * 64
    if kind == 'runtime.health' and mode == 'stale':
        old = {**reply, 'generationId': '00000000-0000-4000-8000-000000000000'}
        sys.stdout.buffer.write(encode_message(old))
    sys.stdout.buffer.write(encode_message(reply))
    sys.stdout.buffer.flush()
    if runtime.closed:
        break
