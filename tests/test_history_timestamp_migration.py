from __future__ import annotations

from sources import direct_scraper, legacy_scraper


def _sample_history() -> dict[str, object]:
    return {
        "legacy-bridge": "2026-06-05T07:45:38",
        "dict-bridge": {
            "raw": "obfs4 203.0.113.1:443 cert=abc iat-mode=0",
            "transport": "obfs4",
            "first_seen": "2026-06-05T07:45:38",
            "last_seen": "2026-06-05T08:45:38+03:30",
            "tcp_reachable": True,
            "capabilities": {"kept": True},
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
    assert entry["tcp_reachable"] is True
    assert entry["capabilities"] == {"kept": True}


def test_direct_history_timestamp_migration_normalizes_without_dropping_fields() -> None:
    history = _sample_history()

    assert direct_scraper.normalize_history_timestamps(history) is history

    _assert_normalized(history)


def test_legacy_history_timestamp_migration_normalizes_without_dropping_fields() -> None:
    history = _sample_history()

    assert legacy_scraper.normalize_history_timestamps(history) is history

    _assert_normalized(history)
