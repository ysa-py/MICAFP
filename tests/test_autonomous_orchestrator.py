from autonomous import AgentRole, NetworkHealth, ResilientOrchestrator, TaskStatus


def test_endpoint_selection_prefers_healthiest(tmp_path):
    orch = ResilientOrchestrator(tmp_path / "state.json")
    orch.register_endpoint("slow", "https://slow.example", NetworkHealth(latency_ms=1800, packet_loss=0.4, bandwidth_kbps=64, online=True))
    orch.register_endpoint("fast", "https://fast.example", NetworkHealth(latency_ms=50, packet_loss=0.0, bandwidth_kbps=8192, online=True))
    assert orch.choose_endpoint().name == "fast"


def test_task_deduplication_and_persistence(tmp_path):
    state = tmp_path / "state.json"
    orch = ResilientOrchestrator(state)
    first = orch.schedule("validate", {"target": "artifacts"}, role=AgentRole.VALIDATOR)
    second = orch.schedule("validate", {"target": "artifacts"}, role=AgentRole.VALIDATOR)
    assert first.idempotency_key == second.idempotency_key
    loaded = ResilientOrchestrator(state)
    assert len(loaded._queue) == 1


def test_circuit_breaker_defers_failed_endpoint(tmp_path):
    orch = ResilientOrchestrator(tmp_path / "state.json")
    endpoint = orch.register_endpoint("primary", "https://primary.example", NetworkHealth(online=True))
    for _ in range(3):
        orch.mark_failure(endpoint)
    assert not endpoint.available()


def test_run_ready_records_checkpoint(tmp_path):
    orch = ResilientOrchestrator(tmp_path / "state.json")
    orch.register_endpoint("local", "file://cache", NetworkHealth(latency_ms=1, online=True))
    orch.register_handler("validate", lambda task, endpoint: {"ok": True, "endpoint": endpoint.name})
    orch.schedule("validate", {"target": "docs"}, priority=1)
    completed = orch.run_ready(budget=1)
    assert completed[0].status == TaskStatus.SUCCEEDED
    assert completed[0].checkpoint == {"ok": True, "endpoint": "local"}
