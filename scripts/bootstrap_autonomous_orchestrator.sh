#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
STATE_PATH="${AUTONOMOUS_STATE_PATH:-$ROOT_DIR/data/autonomous_orchestrator_state.json}"

cd "$ROOT_DIR"
python - <<'PY'
import os
from autonomous import AgentRole, NetworkHealth, ResilientOrchestrator

state_path = os.environ.get("AUTONOMOUS_STATE_PATH", "data/autonomous_orchestrator_state.json")
orch = ResilientOrchestrator(state_path)
orch.register_endpoint("github-actions", "https://api.github.com", NetworkHealth(latency_ms=120, packet_loss=0.0, bandwidth_kbps=4096, online=True))
orch.register_endpoint("local-cache", "file://data/cache", NetworkHealth(latency_ms=5, packet_loss=0.0, bandwidth_kbps=100000, online=True))
orch.schedule("validate_artifacts", {"command": "python scripts/validate_artifacts.py"}, role=AgentRole.VALIDATOR, priority=10)
orch.schedule("security_scan", {"command": "python scripts/security_scan.py"}, role=AgentRole.VALIDATOR, priority=20)
orch.schedule("dependency_audit", {"command": "python scripts/validate_dependencies.py"}, role=AgentRole.RECOVERY, priority=30)
orch.schedule("documentation_refresh", {"command": "python scripts/generate_architecture_docs.py"}, role=AgentRole.DOCUMENTATION, priority=40)
print(f"Autonomous orchestrator bootstrapped at {state_path}")
print(f"endpoint={orch.choose_endpoint().name if orch.choose_endpoint() else 'none'} timeout={orch.adaptive_timeout_seconds(orch.choose_endpoint()):.1f}s")
PY
