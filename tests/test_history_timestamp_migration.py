from __future__ import annotations

from datetime import UTC, datetime

from sources import direct_scraper, history_utils, legacy_scraper


def _sample_history() -> dict[str, object]:
    return {
        "legacy-bridge": "2026-06-05T07:45:38",
        "dict-bridge": {
            "raw": "obfs4 203.0.113.1:443 cert=abc iat-mode=0",
            "transport": "obfs4",
            "ip_version": "ipv4",
            "first_seen": "2026-06-05T07:45:38",
            "last_seen": "2026-06-05T08:45:38+03:30",
            "tcp_reachable": True,
            "capabilities": {"kept": True, "nested": {"score": 10}},
        },
    }


def _assert_normalized(history: dict[str, object]) -> None:
    assert history["legacy-bridge"] == "2026-06-05T07:45:38+00:00"

    entry = history["dict-bridge"]
    assert isinstance(entry, dict)
    assert entry["first_seen"] == "2026-06-05T07:45:38+00:00"
    assert entry["last_seen"] == "2026-06-05T05:15:38+00:00"
    assert entry["raw"] == "obfs4 203.0.113.1:443 cert=abc iat-mode=0"
    assert entry["transport"] == "obfs4"
    assert entry["ip_version"] == "ipv4"
    assert entry["tcp_reachable"] is True
    assert entry["capabilities"] == {"kept": True, "nested": {"score": 10}}


def test_shared_history_timestamp_migration_normalizes_without_dropping_fields() -> None:
    history = _sample_history()

    assert history_utils.normalize_history_timestamps(history) is history

    _assert_normalized(history)


def test_direct_history_timestamp_migration_normalizes_without_dropping_fields() -> None:
    history = _sample_history()

    assert direct_scraper.normalize_history_timestamps(history) is history

    _assert_normalized(history)


def test_legacy_history_timestamp_migration_normalizes_without_dropping_fields() -> None:
    history = _sample_history()

    assert legacy_scraper.normalize_history_timestamps(history) is history

    _assert_normalized(history)


def test_direct_and_legacy_cleanup_use_same_last_seen_behavior(monkeypatch) -> None:
    monkeypatch.setattr(history_utils, "utc_now", lambda: datetime(2026, 6, 24, tzinfo=UTC))
    history = {
        "stale": "2026-05-01T00:00:00",
        "fresh": "2026-06-23T00:00:00",
        "old-first-new-last": {
            "raw": "obfs4 203.0.113.2:443 cert=def iat-mode=0",
            "transport": "obfs4",
            "ip_version": "ipv4",
            "first_seen": "2026-05-01T00:00:00+00:00",
            "last_seen": "2026-06-23T00:00:00+00:00",
            "tcp_reachable": False,
            "capabilities": {"transport_options": {"iat-mode": "0"}},
        },
    }

    direct_cleaned = direct_scraper.cleanup_history(history.copy(), 30)
    legacy_cleaned = legacy_scraper.cleanup_history(history.copy(), 30)

    assert direct_cleaned == legacy_cleaned
    assert "stale" not in direct_cleaned
    assert "fresh" in direct_cleaned
    assert "old-first-new-last" in direct_cleaned
    entry = direct_cleaned["old-first-new-last"]
    assert isinstance(entry, dict)
    assert entry["raw"] == "obfs4 203.0.113.2:443 cert=def iat-mode=0"
    assert entry["transport"] == "obfs4"
    assert entry["ip_version"] == "ipv4"
    assert entry["first_seen"] == "2026-05-01T00:00:00+00:00"
    assert entry["last_seen"] == "2026-06-23T00:00:00+00:00"
    assert entry["tcp_reachable"] is False
    assert entry["capabilities"] == {"transport_options": {"iat-mode": "0"}}


def test_cleanup_retains_dict_entry_when_last_seen_is_within_retention(monkeypatch) -> None:
    monkeypatch.setattr(history_utils, "utc_now", lambda: datetime(2026, 6, 24, tzinfo=UTC))
    history = {
        "kept-by-last-seen": {
            "first_seen": "2026-05-01T00:00:00+00:00",
            "last_seen": "2026-06-23T00:00:00+00:00",
        },
    }

    assert history_utils.cleanup_history(history, 30) == {
        "kept-by-last-seen": {
            "first_seen": "2026-05-01T00:00:00+00:00",
            "last_seen": "2026-06-23T00:00:00+00:00",
        },
    }


def test_cleanup_can_prefer_first_seen_when_requested(monkeypatch) -> None:
    monkeypatch.setattr(history_utils, "utc_now", lambda: datetime(2026, 6, 24, tzinfo=UTC))
    history = {
        "old-first-new-last": {
            "first_seen": "2026-05-01T00:00:00+00:00",
            "last_seen": "2026-06-23T00:00:00+00:00",
        },
    }

    assert history_utils.cleanup_history(history, 30, prefer_last_seen=False) == {}
