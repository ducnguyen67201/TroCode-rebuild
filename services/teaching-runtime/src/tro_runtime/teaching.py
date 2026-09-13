"""One teaching session; observation and presentation remain separate from learner input."""

import time
from dataclasses import asdict, replace
from pathlib import Path
from typing import Any
from uuid import uuid4

from tro_runtime.agent import GuidanceAgent
from tro_runtime.guidance import Cue, check_expected_value, make_cue
from tro_runtime.model_client import create_model
from tro_runtime.observation_source import CuaObservationSource, ObservationSource
from tro_runtime.observations import Observation, Target
from tro_runtime.session_store import SessionStore


class TeachingSession:
    def __init__(self, source: ObservationSource | None = None) -> None:
        self.source = source
        self.agent: GuidanceAgent | None = None
        self.store: SessionStore | None = None
        self.session_id: str | None = None
        self.targets: tuple[Target, ...] = ()
        self.target: Target | None = None
        self.observation: Observation | None = None
        self.cue: Cue | None = None
        self.last_cue: Cue | None = None
        self.check: dict[str, Any] | None = None
        self.revision = 0

    def configure(self, account: str | None, session: str | None, request: dict[str, Any]) -> None:
        if account is None or session is None or self.store is not None:
            raise ValueError("A fresh authenticated session is required.")
        self.store = SessionStore(Path(request["storageRoot"]), account)
        self.session_id = session
        config = request["modelConfig"]
        if config is not None:
            self.agent = GuidanceAgent(
                create_model(config["origin"], config["grant"], config["model"])
            )

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
        if kind == "runtime.ask":
            if self.agent is None:
                raise ValueError("Connect a proof account with model access first.")
            self.cue = None
            observation = await self.source.observe(self.target)
            proposal = await self.agent.explain(observation, request["question"], request["locale"])
            fresh = await self.source.observe(self.target)
            # AX indices may be reused. Require the same semantic target before presenting.
            original = observation.element(proposal.element_id)
            current = fresh.element(proposal.element_id)
            if (original.label, original.role) != (current.label, current.role):
                raise ValueError("The observed control changed during explanation.")
            if proposal.destination_id is not None:
                original_destination = observation.element(proposal.destination_id)
                current_destination = fresh.element(proposal.destination_id)
                if (original_destination.label, original_destination.role) != (
                    current_destination.label,
                    current_destination.role,
                ):
                    raise ValueError("The destination changed during explanation.")
            self.cue = make_cue(
                fresh,
                proposal.element_id,
                proposal.gesture,
                proposal.caption,
                request["locale"],
                time.time(),
                proposal.destination_id,
                proposal.direction,
            )
            self.observation = fresh
            self.target = fresh.target
            self.last_cue = self.cue
            self.record("presented", cue_id=self.cue.id, observation_id=fresh.id)
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

    async def close(self) -> None:
        self.last_cue = None
        self.cue = None
        self.observation = None
        self.target = None
        self.targets = ()
        self.check = None
        self.agent = None
        if self.store is not None:
            self.store.close()
            self.store = None
        if self.source is not None:
            await self.source.close()
            self.source = None
