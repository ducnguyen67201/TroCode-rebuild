"""Closed recovery messages; never expose upstream bodies or screen content."""

from typing import Literal

Reason = Literal[
    "model_timeout",
    "model_unavailable",
    "invalid_plan",
    "missing_cursor_tool",
    "invalid_cursor_tool",
    "ungrounded_cursor_target",
]
MESSAGES: dict[Reason, str] = {
    "model_timeout": "Planning timed out. Your progress is preserved. Retry explicitly.",
    "model_unavailable": "Model access is unavailable. Reconnect your proof account, then replan.",
    "invalid_plan": "The proposed guidance could not be grounded. Observe again, then replan.",
    "missing_cursor_tool": "The model did not return a cursor step. Try again.",
    "invalid_cursor_tool": "The model returned an invalid cursor tool call. Try again.",
    "ungrounded_cursor_target": "The model chose a target that is not uniquely visible. Try again.",
}


class GuidanceError(Exception):
    def __init__(self, reason: Reason) -> None:
        self.reason = reason
        super().__init__(MESSAGES[reason])
