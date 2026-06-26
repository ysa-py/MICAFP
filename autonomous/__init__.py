"""Autonomous resilience primitives for TorShield-IR.

The package exposes offline-first orchestration building blocks used by the
bootstrap scripts and CI tests. Implementations are deterministic and avoid
network side effects unless a caller injects concrete transport functions.
"""

from .advanced_orchestrator import (
    AdvancedResilientOrchestrator,
    ModelCandidate,
    ResourceSnapshot,
    ResilientOrchestrator,
    ValidationResult,
)
from .resilient_orchestrator import (
    AgentRole,
    AutonomousTask,
    EndpointState,
    NetworkHealth,
    TaskStatus,
)

__all__ = [
    "AdvancedResilientOrchestrator",
    "AgentRole",
    "AutonomousTask",
    "EndpointState",
    "ModelCandidate",
    "NetworkHealth",
    "ResourceSnapshot",
    "ResilientOrchestrator",
    "TaskStatus",
    "ValidationResult",
]
