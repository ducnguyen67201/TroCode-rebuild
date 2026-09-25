import asyncio

import pytest

from tro_runtime.action_run import ActionGuard


def test_confirmation_is_single_use_and_resumes_same_waiter() -> None:
    async def scenario() -> None:
        events: list[dict[str, object]] = []

        async def sink(event: dict[str, object]) -> None:
            events.append(event)

        guard = ActionGuard("run", sink)
        waiting = asyncio.create_task(guard.confirm("Send a message"))
        await asyncio.sleep(0)
        confirmation = str(events[0]["confirmationId"])
        assert guard.decide(confirmation, True)
        assert not guard.decide(confirmation, True)
        await waiting

    asyncio.run(scenario())


def test_rejection_cancels_confirmation() -> None:
    async def scenario() -> None:
        events: list[dict[str, object]] = []

        async def sink(event: dict[str, object]) -> None:
            events.append(event)

        guard = ActionGuard("run", sink)
        waiting = asyncio.create_task(guard.confirm("Delete an item"))
        await asyncio.sleep(0)
        assert guard.decide(str(events[0]["confirmationId"]), False)
        with pytest.raises(asyncio.CancelledError):
            await waiting

    asyncio.run(scenario())
