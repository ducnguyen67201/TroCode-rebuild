from uuid import uuid4

import pytest

from tro_runtime.session_store import SessionStore


def test_isolation_locking_and_idempotent_facts(tmp_path):
    account, session, event = str(uuid4()), str(uuid4()), str(uuid4())
    store = SessionStore(tmp_path, account)
    try:
        assert store.append(event, session, "presented", {"cue_id": "cue"})
        assert not store.append(event, session, "presented", {"cue_id": "cue"})
        with pytest.raises(ValueError):
            store.append(event, session, "reported", {"cue_id": "cue"})
        with pytest.raises(ValueError):
            SessionStore(tmp_path, account)
        other = SessionStore(tmp_path, str(uuid4()))
        assert other.read(session) == []
        other.close()
        assert [fact["kind"] for fact in store.read(session)] == ["presented"]
    finally:
        store.close()
    reopened = SessionStore(tmp_path, account)
    assert len(reopened.read(session)) == 1
    reopened.close()


def test_rejects_paths_and_private_payloads(tmp_path):
    with pytest.raises(ValueError):
        SessionStore(tmp_path, "../escape")
    store = SessionStore(tmp_path, str(uuid4()))
    try:
        with pytest.raises(ValueError):
            store.append(str(uuid4()), str(uuid4()), "observed", {"screenshot": "private"})
    finally:
        store.close()


def test_reopen_labels_unfinished_check_without_inventing_success(tmp_path):
    account, session, check = str(uuid4()), str(uuid4()), str(uuid4())
    store = SessionStore(tmp_path, account)
    store.append(str(uuid4()), session, "check_started", {"check_id": check})
    store.close()
    store = SessionStore(tmp_path, account)
    assert [row["kind"] for row in store.read(session)] == ["check_started", "check_interrupted"]
    store.close()


def test_action_evidence_is_metadata_only(tmp_path):
    account, session = str(uuid4()), str(uuid4())
    store = SessionStore(tmp_path, account)
    try:
        assert store.append(
            str(uuid4()),
            session,
            "action_outcome",
            {"run_id": str(uuid4()), "outcome": "completed"},
        )
        with pytest.raises(ValueError):
            store.append(
                str(uuid4()),
                session,
                "action_outcome",
                {"transcript": "private"},
            )
    finally:
        store.close()
