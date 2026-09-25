"""One live action run, cancellation and single-use confirmation authority."""

from __future__ import annotations

import asyncio
from collections.abc import Awaitable, Callable
from dataclasses import dataclass
from uuid import uuid4

EventSink = Callable[[dict[str, object]], Awaitable[None]]


@dataclass(frozen=True)
class Confirmation:
    id: str
    summary: str


class ActionGuard:
    def __init__(self, run_id: str, event_sink: EventSink) -> None:
        self.run_id = run_id
        self._event_sink = event_sink
        self._pending: tuple[Confirmation, asyncio.Future[bool]] | None = None

    async def confirm(self, summary: str) -> None:
        if self._pending is not None:
            raise ValueError("A confirmation is already pending.")
        confirmation = Confirmation(str(uuid4()), summary[:256])
        decision: asyncio.Future[bool] = asyncio.get_running_loop().create_future()
        self._pending = (confirmation, decision)
        await self._event_sink(
            {
                "phase": "confirmation",
                "summary": "Approval is required before this action.",
                "confirmationId": confirmation.id,
                "confirmationReason": confirmation.summary,
            }
        )
        try:
            approved = await asyncio.wait_for(decision, 30)
        finally:
            self._pending = None
        if not approved:
            raise asyncio.CancelledError

    def decide(self, confirmation_id: str, approved: bool) -> bool:
        if self._pending is None or self._pending[0].id != confirmation_id:
            return False
        future = self._pending[1]
        if future.done():
            return False
        future.set_result(approved)
        return True

    def cancel(self) -> None:
        if self._pending is not None and not self._pending[1].done():
            self._pending[1].cancel()
