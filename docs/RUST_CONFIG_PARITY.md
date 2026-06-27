# config.py Rust parity contract

This document records the Phase 1 parity anchor for `config.py`. The Python
source remains in place; deletion is blocked until continuous parity proves the
full module behavior.

## Rust replacement

* Rust module: `src/config.rs`
* Cargo parity test entrypoint: `tests/config_parity.rs`
* Branch-covering parity cases: `tests/parity/config_parity.rs`

## Behavior contract

| Source behavior | Required Rust behavior |
| --- | --- |
| `os.getenv(name, default)` for strings | Return the environment value when present, otherwise the exact Python default. |
| `int(os.getenv(...))` | Parse the effective string as an integer and fail when Python import would raise `ValueError`. |
| `float(os.getenv(...))` | Parse the effective string as a finite JSON-compatible float and fail when Python import would raise `ValueError`. |
| `os.getenv(...).lower() == "true"` | Match Python boolean handling exactly, including treating whitespace-padded values as false. |
| `CF_N_SLOTS` list comprehensions | Generate slot lists for `1..=CF_N_SLOTS`, applying `.strip()` to account, token, and gateway values. |
| `CF_VALID_SLOTS` | Include only slots with non-empty stripped account id and token; gateway may be empty. |
| `os.path.join(BRIDGE_DIR, ...)` | Derive `HISTORY_FILE` and `SCORES_FILE` from `BRIDGE_DIR` with platform path joining. |
| Literal Iran/NIN/CDN lists | Preserve list order and values exactly. |

No Python file is deleted by this parity anchor. `config.py` remains imported by
the parity tests at runtime.
