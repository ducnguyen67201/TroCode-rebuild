"""Account-isolated evidence facts. Rendering never becomes a learner attempt."""

import json
import sqlite3
import sys
import time
from pathlib import Path
from typing import Any, Literal
from uuid import UUID, uuid4

EvidenceKind = Literal[
    "step_observed",
    "step_reported",
    "presented",
    "acknowledged",
    "reported",
    "observed",
    "check_started",
    "check_confirmed",
    "check_mismatch",
    "check_unknown",
    "check_interrupted",
]
KINDS = frozenset(
    {
        "step_observed",
        "step_reported",
        "presented",
        "acknowledged",
        "reported",
        "observed",
        "check_started",
        "check_confirmed",
        "check_mismatch",
        "check_unknown",
        "check_interrupted",
    }
)


class SessionStore:
    def __init__(self, root: Path, account_id: str) -> None:
        account = str(UUID(account_id))
        directory = root / "accounts" / account
        directory.mkdir(parents=True, exist_ok=True, mode=0o700)
        directory.chmod(0o700)
        self._lock = (directory / "session.lock").open("a+b")
        try:
            if sys.platform == "win32":
                import msvcrt

                self._lock.seek(0)
                self._lock.write(b"\0")
                self._lock.flush()
                self._lock.seek(0)
                msvcrt.locking(self._lock.fileno(), msvcrt.LK_NBLCK, 1)
            else:
                import fcntl

                fcntl.flock(self._lock.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
        except OSError:
            self._lock.close()
            raise ValueError("This account already has an active teaching runtime.") from None
        try:
            self._db = sqlite3.connect(directory / "sessions.sqlite3", timeout=2)
            version = self._db.execute("PRAGMA user_version").fetchone()[0]
            if version not in (0, 1, 2):
                raise ValueError("Unsupported session store version.")
            self._db.execute("PRAGMA journal_mode=WAL")
            self._db.execute("PRAGMA synchronous=FULL")
            self._db.execute("""CREATE TABLE IF NOT EXISTS evidence (
                id TEXT PRIMARY KEY, session_id TEXT NOT NULL, kind TEXT NOT NULL,
                recorded_at REAL NOT NULL, metadata TEXT NOT NULL)""")
            self._db.execute("PRAGMA user_version=2")
            self._db.commit()
            (directory / "sessions.sqlite3").chmod(0o600)
            unfinished = self._db.execute(
                "SELECT session_id, metadata FROM evidence WHERE kind='check_started'"
            ).fetchall()
            finished = {
                json.loads(row[0]).get("check_id")
                for row in self._db.execute(
                    "SELECT metadata FROM evidence WHERE kind IN "
                    "('check_confirmed','check_mismatch','check_unknown','check_interrupted')"
                )
            }
            for session_id, metadata in unfinished:
                check_id = json.loads(metadata).get("check_id")
                if check_id and check_id not in finished:
                    self.append(
                        str(uuid4()), session_id, "check_interrupted", {"check_id": check_id}
                    )
        except Exception:
            if hasattr(self, "_db"):
                self._db.close()
            self._lock.close()
            raise

    def append(
        self, event_id: str, session_id: str, kind: EvidenceKind, metadata: dict[str, Any]
    ) -> bool:
        event_id, session_id = str(UUID(event_id)), str(UUID(session_id))
        if kind not in KINDS:
            raise ValueError("Unknown evidence source.")
        # Persist identifiers/outcomes only; never screenshots, raw AX trees, or credentials.
        if not set(metadata).issubset(
            {
                "cue_id",
                "observation_id",
                "check_id",
                "outcome",
                "plan_id",
                "step_index",
            }
        ):
            raise ValueError("Evidence metadata contains unsupported fields.")
        body = json.dumps(metadata, allow_nan=False, sort_keys=True)
        if len(body.encode()) > 2048:
            raise ValueError("Evidence metadata exceeds its limit.")
        with self._db:
            existing = self._db.execute(
                "SELECT session_id, kind, metadata FROM evidence WHERE id=?", (event_id,)
            ).fetchone()
            if existing:
                if existing != (session_id, kind, body):
                    raise ValueError("Evidence identity cannot be reused for another fact.")
                return False
            self._db.execute(
                "INSERT INTO evidence VALUES (?, ?, ?, ?, ?)",
                (event_id, session_id, kind, time.time(), body),
            )
        return True

    def read(self, session_id: str) -> list[dict[str, Any]]:
        rows = self._db.execute(
            "SELECT id, kind, recorded_at, metadata FROM evidence "
            "WHERE session_id=? ORDER BY recorded_at, rowid LIMIT 1000",
            (str(UUID(session_id)),),
        ).fetchall()
        return [
            {"id": row[0], "kind": row[1], "recorded_at": row[2], "metadata": json.loads(row[3])}
            for row in rows
        ]

    def close(self) -> None:
        self._db.close()
        self._lock.close()
