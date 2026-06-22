# Secrets / Environment Variable Manifest

**Generated:** 2026-06-21, as part of the zero-error remediation pass.
**Do not hand-edit this list out of sync with `configs/env_template.sh`** —
regenerate it (or at least re-diff it) whenever that file changes. The
canonical source of truth for *which keys exist and what they're for* is
`configs/env_template.sh`; this document explains *which of the three CI
systems needs which keys, and how each one gets them*.

---

## Why this file exists

The migration from GitHub Actions → GitLab CI → CircleCI silently dropped
16 real configuration keys the first time (circuit breaker tuning, IRST
stealth-window scheduling, telemetry, uTLS evasion — see
`docs/REMEDIATION_CHANGELOG.md`), and a secrets-manifest cross-check then
found a second, independent gap of 14 more keys (provider API keys
referenced by the dormant GitHub Actions workflows and actively used by
current code, but present in neither `.env` nor the template on either
side of the migration). This file exists so a *third* migration can't
repeat that silently — diff this list against whatever the next platform
needs before cutting over.

---

## How each platform actually gets these values

| Platform | Mechanism |
|---|---|
| **CircleCI** (active) | `scripts/circleci_env_bootstrap.sh` walks `configs/env_template.sh` line by line and pulls a matching value from the `torshield-ir-secrets` CircleCI **Context** for every key found, writing the result to a runtime `.env`. If a key isn't in the template, it is never bootstrapped — this is exactly the bug this remediation fixed. |
| **GitLab CI** (dormant fallback) | GitLab CI/CD **Project Variables** are auto-exported as real process environment variables for every job — no YAML reference needed per key. Whatever is configured in *Settings → CI/CD → Variables* with these exact names is automatically visible to `os.getenv()`. |
| **GitHub Actions** (dormant fallback) | Each workflow YAML explicitly interpolates `${{ secrets.KEY_NAME }}` — only keys that are *both* listed below *and* explicitly referenced in a workflow file are actually wired on this platform. |

---

## Full key inventory (98 keys in `configs/env_template.sh` as of this remediation)

Categories below mirror the template's own section headers.

### AI Providers
`CEREBRAS_API_KEY`, `CEREBRAS_API_KEY_1`, `CEREBRAS_API_KEY_2`,
`CEREBRAS_API_KEY_3`, `CF_ACCOUNT_ID_1`…`CF_ACCOUNT_ID_11`,
`CF_API_TOKEN_1`…`CF_API_TOKEN_11`, `CF_AI_GATEWAY_URL_1`…`CF_AI_GATEWAY_URL_11`,
`PORTKEY_API_KEY`, `PORTKEY_API_KEY_1`, `PORTKEY_API_KEY_2`, `PORTKEY_API_KEY_3`,
`PORTKEY_GATEWAY_URL`, `PORTKEY_VIRTUAL_KEY_1`…`PORTKEY_VIRTUAL_KEY_3`,
`PORTKEY_HEALTH_MODEL`, `PORTKEY_PROVIDER_KEY`, `GROQ_API_KEY`,
`DEEPSEEK_API_KEY`, `GEMINI_API_KEY`, `HUGGINGFACE_API_KEY`,
`HYPERBOLIC_API_KEY`, `MISTRAL_API_KEY`

### GitHub Actions / Self-Heal (GitHub-platform-specific; not needed by CircleCI or GitLab)
`GITHUB_TOKEN`, `GITHUB_REPOSITORY`, `GITHUB_SHA`, `GH_PAT_AUTOFIX`,
`GH_REPO_OWNER`, `GH_REPO_NAME`

### Network / Testing
`MAX_WORKERS`, `CONNECTION_TIMEOUT`, `SSL_TIMEOUT`, `MAX_RETRIES`,
`MAX_TEST_PER_TYPE`

### Time Windows
`RECENT_HOURS`, `HISTORY_RETENTION_DAYS`

### File Paths
`BRIDGE_DIR`, `EXPORT_DIR`

### Repository
`REPO_URL`

### Telegram
`TELEGRAM_BOT_TOKEN`, `TELEGRAM_CHAT_ID`, `TELEGRAM_UPLOAD`

### Proxy
`HTTP_PROXY`, `HTTPS_PROXY`

### Collection Sources
`USE_TORPROJECT_SCRAPER`, `USE_MOAT_API`, `USE_BRIDGEDB_API`,
`USE_TELEGRAM_SOURCES`, `USE_STATIC_BRIDGES`

### Iran-Specific
`NIN_MODE`, `DEEP_TEST`

### v4 — uTLS / Elite Registry / Circuit Breaker / Telemetry / IRST
**(the 16 keys fixed by this remediation — see §1.3 of the engineering prompt)**
`UTLS_EVASION_MODE`, `UTLS_PROFILE_ROTATION`, `ELITE_REGISTRY_ENABLED`,
`ELITE_REGISTRY_REFRESH_HRS`, `CIRCUIT_BREAKER_ENABLED`,
`CIRCUIT_BREAKER_MAX_FAILURES`, `CIRCUIT_BREAKER_RESET_SECS`,
`SESSION_BLACKLIST_DURATION_SECS`, `TELEMETRY_ENABLED`,
`TELEMETRY_AUTO_DEBUG_THRESHOLD`, `TELEMETRY_LOG_MAX_MB`,
`IRST_HIGH_CENSORSHIP_START`, `IRST_HIGH_CENSORSHIP_END`,
`IRST_ULTRA_STEALTH_START`, `IRST_ULTRA_STEALTH_END`, `DATABASE_URL`

### RIPE Atlas
`RIPE_ATLAS_API_KEY`

---

## Per-platform action items if reactivating a dormant CI system

- **GitLab CI:** configure every key above (except the GitHub-Actions-only
  block) as a Project CI/CD Variable with the exact same name. No YAML
  edits needed — `.gitlab-ci.yml` / `.gitlab/ci/*.yml` already expect
  `os.getenv()` to find them via GitLab's automatic export.
- **GitHub Actions:** configure every key above as a Repository Secret,
  *and* verify each workflow file actually interpolates
  `${{ secrets.KEY_NAME }}` for the keys it needs — a secret existing in
  the repo settings does nothing if the workflow YAML never references it.
- **CircleCI:** add/update the `torshield-ir-secrets` Context with every
  key above; `scripts/circleci_env_bootstrap.sh` + the now-complete
  `configs/env_template.sh` handle the rest automatically.
