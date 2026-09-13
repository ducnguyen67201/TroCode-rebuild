"""One teaching session; observation and presentation remain separate from learner input."""

from __future__ import annotations

import time
from dataclasses import asdict, replace
from pathlib import Path
from typing import TYPE_CHECKING, Any
from uuid import uuid4

from tro_runtime.guidance import Cue, check_expected_value, make_cue
from tro_runtime.observation_source import CuaObservationSource, ObservationSource
from tro_runtime.observations import Observation, Target
from tro_runtime.planning import PlanProgress
from tro_runtime.session_store import SessionStore

if TYPE_CHECKING:
    from tro_runtime.agent import GuidanceAgent


class TeachingSession:
    def __init__(self, source: ObservationSource | None = None) -> None:
        self.source = source
        self.agent: GuidanceAgent | None = None
        self.model_config: dict[str, str] | None = None
        self.store: SessionStore | None = None
        self.session_id: str | None = None
        self.targets: tuple[Target, ...] = ()
        self.target: Target | None = None
        self.observation: Observation | None = None
        self.cue: Cue | None = None
        self.last_cue: Cue | None = None
        self.check: dict[str, Any] | None = None
        self.revision = 0
        self.progress: PlanProgress | None = None
        self.objective = ""
        self.replans_remaining = 0

    def configure(self, account: str | None, session: str | None, request: dict[str, Any]) -> None:
        if account is None or session is None or self.store is not None:
            raise ValueError("A fresh authenticated session is required.")
        self.store = SessionStore(Path(request["storageRoot"]), account)
        self.session_id = session
        self.model_config = request["modelConfig"]

    def record(self, kind: Any, **metadata: str) -> None:
        if self.store is not None and self.session_id is not None:
            self.store.append(str(uuid4()), self.session_id, kind, metadata)

    def projection(self, session_id: str | None) -> dict[str, Any]:
        self.revision += 1
        observation = asdict(self.observation) if self.observation else None
        if observation is not None:
            observation.pop("image", None)  # Image bytes remain inside the Python/model boundary.
        return {
            "revision": self.revision,
            "journey": self.progress.projection() if self.progress else None,
            "session_id": session_id,
            "targets": [asdict(target) for target in self.targets],
            "target": asdict(self.target) if self.target else None,
            "observation": observation,
            "cue": asdict(self.cue) if self.cue else None,
            "check": self.check,
        }

    async def handle(self, request: dict[str, Any]) -> str:
        if self.source is None:
            self.source = CuaObservationSource()
        kind = request["kind"]
        if kind in (
            "runtime.listTargets",
            "runtime.selectTarget",
            "runtime.explain",
            "runtime.ask",
        ):
            self.progress = None
        if kind == "runtime.listTargets":
            self.cue = None
            self.targets = await self.source.list_targets()
            return "targetsResult"
        if kind == "runtime.selectTarget":
            matches = [
                target
                for target in self.targets
                if (target.pid, target.window_id) == (request["pid"], request["windowId"])
            ]
            if len(matches) != 1:
                raise ValueError("Select an available window.")
            self.last_cue = None
            self.target = matches[0]
            self.observation = None
            self.cue = None
            self.check = None
            return "targetSelected"
        if self.target is None:
            raise ValueError("Select a window first.")
        if kind == "runtime.refreshCue":
            if self.progress is not None:
                await self.refresh_plan()
                return "cueRefreshResult"
            if self.cue is None or self.observation is None:
                return "cueRefreshResult"
            fresh = await self.source.observe(self.target, include_image=False)
            try:
                old = self.observation.element(self.cue.element_id)
                new = fresh.element(self.cue.element_id)
                unchanged = old == new and fresh.target == self.observation.target
                if self.cue.destination is not None:
                    old_destination = [
                        element
                        for element in self.observation.elements
                        if element.bounds == self.cue.destination
                    ]
                    new_destination = [
                        element
                        for element in fresh.elements
                        if element.bounds == self.cue.destination
                    ]
                    unchanged = (
                        unchanged
                        and len(old_destination) == 1
                        and old_destination == new_destination
                    )
            except ValueError:
                unchanged = False
            if unchanged:
                self.cue = replace(
                    self.cue, observation_id=fresh.id, expires_at=fresh.captured_at + 1
                )
                self.observation = fresh
            else:
                self.cue = None
            return "cueRefreshResult"
        if kind == "runtime.planControl":
            if self.progress is None:
                raise ValueError("No active plan.")
            before = self.progress.index
            self.progress.control(request["action"])
            if self.progress.index != before:
                self.record("step_reported", plan_id=self.progress.id, step_index=str(before))
            self.cue = None
            await self.refresh_plan()
            return "planControlResult"
        if kind == "runtime.ask":
            if self.agent is None and self.model_config is not None:
                from tro_runtime.agent import GuidanceAgent
                from tro_runtime.model_client import create_model

                config = self.model_config
                self.agent = GuidanceAgent(
                    create_model(config["origin"], config["grant"], config["model"])
                )
            if self.agent is None:
                raise ValueError("Connect a proof account with model access first.")
            self.cue = None
            observation = await self.source.observe(self.target)
            plan = await self.agent.plan(observation, request["question"], request["locale"])
            self.objective = request["question"]
            self.replans_remaining = 1
            self.progress = PlanProgress(plan, request["locale"], observation)
            await self.refresh_plan()
            return "askResult"
        if kind in ("runtime.observe", "runtime.explain", "runtime.check"):
            previous = self.observation
            previous_cue = self.last_cue
            self.cue = None
            self.check = None
            if kind == "runtime.check":
                if previous is None or previous_cue is None or previous_cue.id != request["cueId"]:
                    raise ValueError("This guidance is no longer current.")
                self.record("reported", cue_id=previous_cue.id)
                self.record("check_started", check_id=request["requestId"], cue_id=previous_cue.id)
            # Every guidance/check request obtains fresh selected-window evidence.
            observation = await self.source.observe(self.target)
            self.record("observed", observation_id=observation.id)
            if kind == "runtime.explain":
                if previous is None:
                    raise ValueError("Observe before selecting a control.")
                for element_id in (request["elementId"], request["destinationId"]):
                    if element_id is not None:
                        old, new = previous.element(element_id), observation.element(element_id)
                        if (old.label, old.role) != (new.label, new.role):
                            raise ValueError("The selected control changed. Observe again.")
                self.cue = make_cue(
                    observation,
                    request["elementId"],
                    request["gesture"],
                    request["caption"],
                    request["locale"],
                    time.time(),
                    request["destinationId"],
                    request["direction"],
                )
                self.last_cue = self.cue
                self.record("presented", cue_id=self.cue.id, observation_id=observation.id)
            if kind == "runtime.check":
                if previous is None or previous_cue is None or previous_cue.id != request["cueId"]:
                    raise ValueError("This guidance is no longer current.")
                self.check = asdict(
                    check_expected_value(
                        previous, observation, request["label"], request["expected"]
                    )
                )
                self.record(
                    "check_" + self.check["outcome"],
                    check_id=request["requestId"],
                    observation_id=observation.id,
                )
            self.observation = observation
            self.target = observation.target
            return {
                "runtime.observe": "observationResult",
                "runtime.explain": "explanationResult",
                "runtime.check": "checkResult",
            }[kind]
        if kind == "runtime.presentationAck":
            if self.cue is None or self.cue.id != request["cueId"]:
                raise ValueError("This guidance is no longer current.")
            self.record("acknowledged", cue_id=self.cue.id)
            # Rendering acknowledges pixels only; never creates a check or learner attempt.
            return "presentationAckResult"
        raise ValueError("Unsupported teaching request.")

    async def refresh_plan(self) -> None:
        if self.progress is None or self.source is None or self.target is None:
            return
        self.cue = None
        if self.progress.status in ("paused", "completed"):
            return
        before = self.progress.index
        was_shown = self.progress.shown
        try:
            fresh = await self.source.observe(self.target, include_image=False)
            cue = self.progress.observe(fresh)
        except Exception:
            self.progress.pause("Observation is unavailable. Check access and resume explicitly.")
            return
        if self.progress.needs_replan and self.replans_remaining and self.agent is not None:
            self.replans_remaining -= 1
            try:
                context = await self.source.observe(self.target)
                plan = await self.agent.plan(
                    context,
                    self.objective,
                    self.progress.locale,
                    tuple(step.caption for step in self.progress.plan.steps[: self.progress.index]),
                )
                self.progress = PlanProgress(plan, self.progress.locale, context)
                await self.refresh_plan()
            except Exception:
                self.progress.pause("Replanning is unavailable. Retry explicitly when ready.")
            return
        self.observation = fresh
        self.target = fresh.target
        self.cue = cue
        if cue is not None:
            self.last_cue = cue
            if not was_shown or self.progress.index != before:
                self.record("presented", cue_id=cue.id, observation_id=fresh.id)
        if self.progress.index != before:
            self.record(
                "step_observed",
                plan_id=self.progress.id,
                step_index=str(before),
                observation_id=fresh.id,
            )

    async def close(self) -> None:
        self.progress = None
        self.objective = ""
        self.replans_remaining = 0
        self.last_cue = None
        self.cue = None
        self.observation = None
        self.target = None
        self.targets = ()
        self.check = None
        self.agent = None
        self.model_config = None
        if self.store is not None:
            self.store.close()
            self.store = None
        if self.source is not None:
            await self.source.close()
            self.source = None
