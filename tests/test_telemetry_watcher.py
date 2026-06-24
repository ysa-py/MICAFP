from datetime import UTC, datetime, timedelta, timezone

from telemetry_watcher import DPIEvent, SelfHealEvent, SlotEvent, TelemetryWatcher


def test_24h_summary_accepts_naive_and_aware_event_timestamps():
    watcher = TelemetryWatcher()
    watcher._dpi_events.clear()
    watcher._slot_events.clear()
    watcher._self_heal_events.clear()
    watcher._save_daily_report = lambda aggregation: None

    now = datetime.now(UTC)
    naive_recent = (now - timedelta(hours=1)).replace(tzinfo=None).isoformat()
    aware_recent = (now - timedelta(hours=2)).astimezone(timezone(timedelta(hours=3))).isoformat()

    watcher._dpi_events.extend([
        DPIEvent(timestamp=naive_recent, dpi_system="sni_inspector", action="blocked"),
        DPIEvent(timestamp=aware_recent, dpi_system="ja3_fingerprinter", action="evaded"),
    ])
    watcher._slot_events.extend([
        SlotEvent(timestamp=naive_recent, slot_index=1, env_var="CF_API_TOKEN_1", error_type="HTTP 403"),
        SlotEvent(timestamp=aware_recent, slot_index=2, env_var="CF_API_TOKEN_2", error_type="Timeout", recovered=True),
    ])
    watcher._self_heal_events.extend([
        SelfHealEvent(timestamp=naive_recent, action_type="reset_circuit"),
        SelfHealEvent(timestamp=aware_recent, action_type="auto_switch_provider"),
    ])

    summary = watcher.get_24h_summary()

    assert summary.total_dpi_events == 2
    assert summary.total_slot_failures == 2
    assert summary.total_self_heal_events == 2


def test_parse_ts_normalizes_to_utc_for_naive_and_aware_timestamps():
    naive = TelemetryWatcher._parse_ts("2026-06-24T12:00:00")
    aware = TelemetryWatcher._parse_ts("2026-06-24T15:30:00+03:30")

    assert naive == datetime(2026, 6, 24, 12, 0, tzinfo=UTC)
    assert aware == datetime(2026, 6, 24, 12, 0, tzinfo=UTC)
