#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
STATE_PATH="${AUTONOMOUS_STATE_PATH:-$ROOT_DIR/data/autonomous_orchestrator_state.json}"

cd "$ROOT_DIR"
python - <<'PY'
import os
from autonomous import AgentRole, ModelCandidate, NetworkHealth, ResilientOrchestrator

state_path = os.environ.get("AUTONOMOUS_STATE_PATH", "data/autonomous_orchestrator_state.json")
orch = ResilientOrchestrator(state_path)
orch.register_endpoint("github-actions", "https://api.github.com", NetworkHealth(latency_ms=120, packet_loss=0.0, bandwidth_kbps=4096, online=True))
orch.register_endpoint("local-cache", "file://data/cache", NetworkHealth(latency_ms=5, packet_loss=0.0, bandwidth_kbps=100000, online=True))
orch.register_model(ModelCandidate("local-fast", quality=0.74, latency_ms=80, cost_per_1k=0.0, available=True, context_tokens=8192))
orch.register_model(ModelCandidate("github-hosted-deep", quality=0.92, latency_ms=450, cost_per_1k=0.0, available=True, context_tokens=128000))
orch.plan_validation_cycle([
    "python scripts/validate_artifacts.py",
    "python scripts/security_scan.py --fail-on-severity critical",
    "python scripts/validate_dependencies.py",
    "python scripts/generate_architecture_docs.py",
])
orch.record_resource_snapshot()
print(f"Autonomous orchestrator bootstrapped at {state_path}")
selected = orch.choose_endpoint()
model = orch.route_model(required_context_tokens=32000)
print(f"endpoint={selected.name if selected else 'none'} timeout={orch.adaptive_timeout_seconds(selected):.1f}s")
print(f"model={model.name if model else 'none'} queued_validations={len(orch._queue)}")
PY
