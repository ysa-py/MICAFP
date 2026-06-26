#!/usr/bin/env python3
from __future__ import annotations

"""Offline-first autonomous resilience orchestrator.

This module intentionally contains no hard-coded bridge provisioning or traffic
obfuscation side effects.  It provides enterprise reliability primitives that
callers can compose with approved transports: persistent idempotent task queue,
adaptive endpoint health scoring, circuit-breaker failover, checkpoint recovery,
request de-duplication, and reflection events for autonomous validation.
"""

import hashlib
import heapq
import json
import math
import os
import time
import uuid
from collections.abc import Callable, Iterable
from dataclasses import asdict, dataclass, field
from enum import Enum
from pathlib import Path
from typing import Any


class TaskStatus(str, Enum):
    PENDING = "pending"
    RUNNING = "running"
    SUCCEEDED = "succeeded"
    FAILED = "failed"
    DEFERRED = "deferred"


class AgentRole(str, Enum):
    PLANNER = "planner"
    NETWORK_HEALTH = "network_health"
    VALIDATOR = "validator"
    RECOVERY = "recovery"
    OPTIMIZER = "optimizer"
    DOCUMENTATION = "documentation"


@dataclass(slots=True)
class NetworkHealth:
    latency_ms: float = 250.0
    packet_loss: float = 0.0
    bandwidth_kbps: float = 1024.0
    online: bool = True
    last_checked: float = field(default_factory=time.time)

    @property
    def score(self) -> float:
        latency_penalty = min(self.latency_ms / 2000.0, 1.0)
        loss_penalty = min(max(self.packet_loss, 0.0), 1.0)
        bandwidth_bonus = min(math.log2(max(self.bandwidth_kbps, 1.0)) / 16.0, 1.0)
        online_bonus = 1.0 if self.online else 0.0
        return max(0.0, min(1.0, (online_bonus * 0.50) + (1 - latency_penalty) * 0.20 + (1 - loss_penalty) * 0.20 + bandwidth_bonus * 0.10))


@dataclass(slots=True)
class EndpointState:
    name: str
    url: str
    health: NetworkHealth = field(default_factory=NetworkHealth)
    failures: int = 0
    successes: int = 0
    circuit_open_until: float = 0.0

    def available(self, now: float | None = None) -> bool:
        return (now or time.time()) >= self.circuit_open_until and self.health.online

    @property
    def selection_score(self) -> float:
        reliability = (self.successes + 1) / (self.successes + self.failures + 2)
        return self.health.score * reliability


@dataclass(order=True, slots=True)
class AutonomousTask:
    run_at: float
    priority: int
    action: str = field(compare=False)
    payload: dict[str, Any] = field(default_factory=dict, compare=False)
    role: AgentRole = field(default=AgentRole.PLANNER, compare=False)
    task_id: str = field(default_factory=lambda: uuid.uuid4().hex, compare=False)
    idempotency_key: str = field(default="", compare=False)
    attempts: int = field(default=0, compare=False)
    status: TaskStatus = field(default=TaskStatus.PENDING, compare=False)
    checkpoint: dict[str, Any] = field(default_factory=dict, compare=False)

    def ensure_idempotency_key(self) -> None:
        if not self.idempotency_key:
            canonical = json.dumps({"action": self.action, "payload": self.payload}, sort_keys=True, default=str)
            self.idempotency_key = hashlib.sha256(canonical.encode("utf-8")).hexdigest()


class ResilientOrchestrator:
    """Autonomous task scheduler with adaptive network resilience controls."""

    def __init__(self, state_path: str | os.PathLike[str] = "data/autonomous_orchestrator_state.json") -> None:
        self.state_path = Path(state_path)
        self.endpoints: dict[str, EndpointState] = {}
        self._queue: list[AutonomousTask] = []
        self._dedupe: set[str] = set()
        self.reflection_log: list[dict[str, Any]] = []
        self.handlers: dict[str, Callable[[AutonomousTask, EndpointState | None], dict[str, Any]]] = {}
        self.load()

    def register_endpoint(self, name: str, url: str, health: NetworkHealth | None = None) -> EndpointState:
        endpoint = self.endpoints.get(name) or EndpointState(name=name, url=url, health=health or NetworkHealth())
        endpoint.url = url
        if health is not None:
            endpoint.health = health
        self.endpoints[name] = endpoint
        self.save()
        return endpoint

    def register_handler(self, action: str, handler: Callable[[AutonomousTask, EndpointState | None], dict[str, Any]]) -> None:
        self.handlers[action] = handler

    def schedule(self, action: str, payload: dict[str, Any] | None = None, *, role: AgentRole = AgentRole.PLANNER, priority: int = 50, delay_seconds: float = 0.0, idempotency_key: str = "") -> AutonomousTask:
        task = AutonomousTask(run_at=time.time() + max(delay_seconds, 0.0), priority=priority, action=action, payload=payload or {}, role=role, idempotency_key=idempotency_key)
        task.ensure_idempotency_key()
        if task.idempotency_key in self._dedupe:
            self.reflect("deduplicated", {"action": action, "idempotency_key": task.idempotency_key})
            return task
        self._dedupe.add(task.idempotency_key)
        heapq.heappush(self._queue, task)
        self.save()
        return task

    def choose_endpoint(self) -> EndpointState | None:
        candidates = [endpoint for endpoint in self.endpoints.values() if endpoint.available()]
        if not candidates:
            return None
        return max(candidates, key=lambda endpoint: endpoint.selection_score)

    def adaptive_timeout_seconds(self, endpoint: EndpointState | None = None) -> float:
        health = endpoint.health if endpoint else NetworkHealth(online=False)
        base = max(2.0, health.latency_ms / 1000.0 * 4.0)
        loss_multiplier = 1.0 + min(max(health.packet_loss, 0.0), 1.0) * 3.0
        bandwidth_multiplier = 2.0 if health.bandwidth_kbps < 128 else 1.0
        return min(120.0, base * loss_multiplier * bandwidth_multiplier)

    def backoff_seconds(self, attempts: int, endpoint: EndpointState | None = None) -> float:
        health_factor = 1.0 + (1.0 - (endpoint.health.score if endpoint else 0.0))
        return min(300.0, (2 ** min(attempts, 8)) * health_factor)

    def mark_success(self, endpoint: EndpointState | None) -> None:
        if endpoint:
            endpoint.successes += 1
            endpoint.failures = max(0, endpoint.failures - 1)
            endpoint.circuit_open_until = 0.0
        self.save()

    def mark_failure(self, endpoint: EndpointState | None) -> None:
        if endpoint:
            endpoint.failures += 1
            if endpoint.failures >= 3:
                endpoint.circuit_open_until = time.time() + self.backoff_seconds(endpoint.failures, endpoint)
        self.save()

    def run_ready(self, *, budget: int = 10) -> list[AutonomousTask]:
        completed: list[AutonomousTask] = []
        now = time.time()
        while self._queue and budget > 0 and self._queue[0].run_at <= now:
            task = heapq.heappop(self._queue)
            task.status = TaskStatus.RUNNING
            task.attempts += 1
            endpoint = self.choose_endpoint()
            handler = self.handlers.get(task.action, self._default_handler)
            try:
                task.checkpoint = handler(task, endpoint)
                task.status = TaskStatus.SUCCEEDED
                self.mark_success(endpoint)
                completed.append(task)
                self.reflect("task_succeeded", {"task_id": task.task_id, "action": task.action, "endpoint": endpoint.name if endpoint else None})
            except Exception as exc:
                task.status = TaskStatus.DEFERRED
                self.mark_failure(endpoint)
                task.run_at = time.time() + self.backoff_seconds(task.attempts, endpoint)
                heapq.heappush(self._queue, task)
                self.reflect("task_deferred", {"task_id": task.task_id, "action": task.action, "error": str(exc), "attempts": task.attempts})
            budget -= 1
        self.save()
        return completed

    def heartbeat(self, probes: Iterable[tuple[str, NetworkHealth]]) -> None:
        for name, health in probes:
            if name in self.endpoints:
                self.endpoints[name].health = health
        self.reflect("heartbeat", {"endpoint_count": len(self.endpoints)})
        self.save()

    def reflect(self, event: str, detail: dict[str, Any]) -> None:
        self.reflection_log.append({"ts": time.time(), "event": event, "detail": detail})
        self.reflection_log = self.reflection_log[-200:]

    def _default_handler(self, task: AutonomousTask, endpoint: EndpointState | None) -> dict[str, Any]:
        return {"handled": False, "action": task.action, "endpoint": endpoint.name if endpoint else None}

    def save(self) -> None:
        self.state_path.parent.mkdir(parents=True, exist_ok=True)
        payload = {
            "endpoints": {name: asdict(endpoint) for name, endpoint in self.endpoints.items()},
            "queue": [asdict(task) for task in sorted(self._queue)],
            "reflection_log": self.reflection_log,
        }
        self.state_path.write_text(json.dumps(payload, indent=2, sort_keys=True), encoding="utf-8")

    def load(self) -> None:
        if not self.state_path.exists() or self.state_path.stat().st_size == 0:
            return
        try:
            payload = json.loads(self.state_path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as exc:
            self.reflect("state_recovery", {"path": str(self.state_path), "error": str(exc)})
            return
        for name, raw in payload.get("endpoints", {}).items():
            health = NetworkHealth(**raw.get("health", {}))
            self.endpoints[name] = EndpointState(name=raw.get("name", name), url=raw.get("url", ""), health=health, failures=raw.get("failures", 0), successes=raw.get("successes", 0), circuit_open_until=raw.get("circuit_open_until", 0.0))
        for raw in payload.get("queue", []):
            task = AutonomousTask(run_at=raw["run_at"], priority=raw["priority"], action=raw["action"], payload=raw.get("payload", {}), role=AgentRole(raw.get("role", AgentRole.PLANNER)), task_id=raw.get("task_id", uuid.uuid4().hex), idempotency_key=raw.get("idempotency_key", ""), attempts=raw.get("attempts", 0), status=TaskStatus(raw.get("status", TaskStatus.PENDING)), checkpoint=raw.get("checkpoint", {}))
            task.ensure_idempotency_key()
            self._dedupe.add(task.idempotency_key)
            heapq.heappush(self._queue, task)
        self.reflection_log = payload.get("reflection_log", [])[-200:]
