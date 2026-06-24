from __future__ import annotations

"""Shared helpers for bridge history timestamp handling."""

from datetime import datetime, timedelta
from typing import Any

from core.dt_utils import coerce_utc_dt, utc_now


def parse_history_dt(value: Any) -> datetime:
    """Parse a history timestamp into a UTC-aware ``datetime``."""
    return coerce_utc_dt(value)


def normalize_history_timestamps(history: dict[str, Any]) -> dict[str, Any]:
    """Normalize history timestamps to UTC-aware ISO strings in-place.

    Legacy string entries are timestamp values and are normalized directly.
    Dict entries retain all existing metadata and only normalize known timestamp
    fields when those fields are strings.
    """
    for key, entry in history.items():
        if isinstance(entry, str):
            history[key] = parse_history_dt(entry).isoformat()
        elif isinstance(entry, dict):
            for field in ("first_seen", "last_seen"):
                value = entry.get(field)
                if isinstance(value, str):
                    entry[field] = parse_history_dt(value).isoformat()
    return history


def history_entry_timestamp(entry: Any) -> Any:
    """Return the preferred timestamp value for a bridge history entry."""
    if isinstance(entry, str):
        return entry
    if isinstance(entry, dict):
        return entry.get("last_seen") or entry.get("first_seen")
    return None


def cleanup_history(
    history: dict[str, Any],
    retention_days: int,
    prefer_last_seen: bool = True,
) -> dict[str, Any]:
    """Remove entries older than ``retention_days`` using UTC comparisons.

    By default, dict entries are retained based on ``last_seen`` when present,
    falling back to ``first_seen``. Set ``prefer_last_seen=False`` to prefer
    ``first_seen`` while still falling back to ``last_seen``.
    """
    cutoff = utc_now() - timedelta(days=retention_days)
    stale: list[str] = []
    for key, entry in history.items():
        if isinstance(entry, dict):
            if prefer_last_seen:
                timestamp = entry.get("last_seen") or entry.get("first_seen")
            else:
                timestamp = entry.get("first_seen") or entry.get("last_seen")
        else:
            timestamp = history_entry_timestamp(entry)

        if parse_history_dt(timestamp) < cutoff:
            stale.append(key)

    for key in stale:
        del history[key]
    return history
