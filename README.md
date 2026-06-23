# 🌐 Tor Bridges Ultra Collector

> Auto-collected, tested, and Iran-scored Tor bridges.  
> GitHub Actions runs every hour — fresh bridges always available.  
> **Last update:** `2026-06-11 14:09 UTC`

## ⚠️ Notes for Iran Users

- **Internet cut (شبکه ملی):** Use `export/iran_cut_pack.txt` — contains Snowflake and WebTunnel bridges that survive NIN.
- **Normal censorship:** Use `export/iran_pack.txt` — top-ranked obfs4/WebTunnel bridges for Iran's DPI.
- **Port 443 bridges** are prioritised — Iran almost never blocks HTTPS.
- **IPv4 is more stable** than IPv6 inside Iran.

## ✅ Tested & Active (Recommended)

| Transport | IPv4 Tested | Count |
| :--- | :--- | :--- |
| **obfs4** | [obfs4_tested.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/bridge/obfs4_tested.txt) | **9** |
| **WebTunnel** | [webtunnel_tested.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/bridge/webtunnel_tested.txt) | **2** |
| **Snowflake** | [snowflake_tested.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/bridge/snowflake_tested.txt) | **8** |
| **Vanilla** | [vanilla_tested.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/bridge/vanilla_tested.txt) | **2** |
| **meek-lite** | [meek_lite_tested.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/bridge/meek_lite_tested.txt) | **2** |

## 🕐 Fresh Bridges (Last 72h)

| Transport | IPv4 | Count | IPv6 | Count |
| :--- | :--- | :--- | :--- | :--- |
| **obfs4** | [obfs4_72h.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/bridge/obfs4_72h.txt) | **16** | [obfs4_72h_ipv6.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/bridge/obfs4_72h_ipv6.txt) | **2** |
| **WebTunnel** | [webtunnel_72h.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/bridge/webtunnel_72h.txt) | **2** | [webtunnel_72h_ipv6.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/bridge/webtunnel_72h_ipv6.txt) | **0** |
| **Vanilla** | [vanilla_72h.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/bridge/vanilla_72h.txt) | **2** | [vanilla_72h_ipv6.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/bridge/vanilla_72h_ipv6.txt) | **0** |

## 📦 Full Archive

| Transport | IPv4 | Count | IPv6 | Count |
| :--- | :--- | :--- | :--- | :--- |
| **obfs4** | [obfs4.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/bridge/obfs4.txt) | **16** | [obfs4_ipv6.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/bridge/obfs4_ipv6.txt) | **2** |
| **WebTunnel** | [webtunnel.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/bridge/webtunnel.txt) | **2** | [webtunnel_ipv6.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/bridge/webtunnel_ipv6.txt) | **0** |
| **Snowflake** | [snowflake.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/bridge/snowflake.txt) | **8** | — | — |
| **Vanilla** | [vanilla.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/bridge/vanilla.txt) | **2** | [vanilla_ipv6.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/bridge/vanilla_ipv6.txt) | **0** |
| **meek-lite** | [meek_lite.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/bridge/meek_lite.txt) | **3** | — | — |

## 🇮🇷 Iran Optimised Packs

| Pack | Description |
| :--- | :--- |
| [iran_pack.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/export/iran_pack.txt) | Top 100 bridges ranked by Iran effectiveness score |
| [iran_cut_pack.txt](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/export/iran_cut_pack.txt) | Bridges for internet cut / شبکه ملی scenarios |
| [bridges_api.json](https://gitlab.com/ultra2200325/ultra/-/raw/main/ultra2200325/ultra/refs/heads/main/export/bridges_api.json) | Machine-readable JSON API |

## 📡 Transport Guide for Iran

| Transport | Anti-DPI | Works during cut | Speed | Recommended |
| :--- | :--- | :--- | :--- | :--- |
| Snowflake | ⭐⭐⭐⭐⭐ | ✅ | Medium | **Yes** |
| WebTunnel | ⭐⭐⭐⭐⭐ | ✅ (CDN) | Fast | **Yes** |
| obfs4 | ⭐⭐⭐⭐ | ❌ | Fast | **Yes** |
| meek-lite | ⭐⭐⭐⭐ | ✅ (Azure) | Slow | Fallback |
| Vanilla | ⭐ | ❌ | Fast | No |

## Disclaimer

For educational and archival purposes. Use bridges responsibly.

## 🔬 Advanced Anti-Filtering Features (NEW)

### AI-Powered Anti-DPI Engine (`--anti-dpi`)
Detects and counters Iran's DPI systems (Arvan DPI, SIAM, Kowsar, NGFW) with:
- Real-time threat analysis and risk scoring
- TLS fingerprint randomization (JA3 evasion)
- SNI evasion strategies (domain fronting, ECH encryption, padding)
- Traffic shaping recommendations (iat-mode=2, burst obfuscation, flow morphing)
- Entropy analysis for statistical fingerprinting detection

### Smart Anti-Filtering Engine (`--anti-filter`)
Comprehensive censorship circumvention system for Iran:
- Real-time censorship level monitoring (Level 1-5)
- ISP-specific blocking predictions (MCI, IRANCELL, Rightel, Shatel, Asiatech)
- Smart bridge selection optimized for current censorship state
- Automatic transport switching when DPI patterns change
- Temporal blocking pattern analysis (best connection windows)
- CDN front selection for NIN scenarios
- Bridge rotation scheduling to avoid fingerprinting

### Auto-Debug System (`--auto-debug`)
Fully autonomous debugging and self-healing:
- Python syntax error detection and auto-fix
- YAML workflow validation and repair
- Import dependency checking
- AI Gateway connectivity verification with LocalAIEngine fallback
- Bridge pipeline health monitoring
- Configuration integrity checks
- Automatic directory structure repair

### LocalAIEngine Fallback
When all external AI providers (Cerebras, Portkey, Cloudflare) are unavailable:
- Zero-dependency rule-based scoring engine activates automatically
- Iran-specific DPI knowledge base (Arvan, SIAM, Kowsar, NGFW, NIN)
- ISP-specific blocking predictions
- Censorship level detection (Level 1-5)
- Transport stack recommendations
- Bridge scoring and ranking
- The gateway **never fails** — always returns a valid response

## 🛠️ Development & Contributing

### Quality Gates

This repository enforces strict quality checks to ensure code reliability:

#### Pre-Push Hook (Local)
Before pushing to `main`, the local pre-push hook automatically runs:
```bash
# After first clone, enable the hook:
git config core.hooksPath .githooks

# The hook will run on push to main:
# - go build ./...
# - go vet ./...
# - gofmt check (bridge package)
# - Python syntax validation
```

#### CI Quality Gate (GitHub Actions)
Every push to `main` and every pull request triggers:
- **Go Quality Gate** (`.github/workflows/go-quality-gate.yml`):
  - `go build ./...` — Must succeed with no errors
  - `go vet ./...` — Static analysis; no warnings
  - `gofmt -l ./internal/bridge/` — Code formatting (bridge package)
  - `go test ./... -v` — All tests must pass
- **Python Quality Check**:
  - `py_compile` on all Python files — Syntax must be valid

### Building the Project

```bash
# Clone and set up
git clone https://github.com/ysa-py/MICAFP.git
cd MICAFP
git config core.hooksPath .githooks

# Install Go dependencies
go mod download

# Build the Go binaries
go build -o bin/iran_tester ./cmd/iran_tester/
go build -o bin/probe_scheduler ./cmd/probe_scheduler/

# Run tests
go test ./... -short
```

### Bridge Package API

The `internal/bridge` package provides parsing and testing for Tor bridge lines:

```go
import "github.com/ysa-py/MICAFP/internal/bridge"

// Parse a bridge line
b, err := bridge.Parse("obfs4 1.2.3.4:9999 cert=abc123 iat-mode=0")
if err != nil {
    log.Fatal(err)
}

// Test connectivity with timeout
ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
defer cancel()
reachable := bridge.TestWithContext(ctx, b, 5*time.Second)
```

Supported bridge formats:
- `vanilla` — Direct IP:port connections
- `obfs4` — Obfuscation protocol v4 with fingerprints
- `webtunnel` — HTTPS tunneling with URLs
- `meek_lite` — Meek protocol via CDN
- `snowflake` — WebRTC-based transport

### Anti-Censorship / Anti-DPI Architecture

The system coordinates multiple existing, actively-maintained tools:

1. **Bridge Collection** (`core/collector.py`) — Gathers bridges from official Tor sources
2. **Connectivity Testing** (`internal/bridge`, `core/tester.py`) — Probes reachability with transport-specific strategies
3. **Scoring & Ranking** (`core/smart_iran_scorer.py`) — Ranks bridges by effectiveness in Iran
4. **Dynamic Selection** (`torshield_ai_gateway/`) — AI-guided transport selection based on ongoing measurements
5. **Result Publishing** (`results_writer.py`, `reports/report_generator.py`) — Distributes optimized bridge lists

The system is **continuously adaptive**:
- Historical scores are stored (JSON/YAML config in `config/`)
- Probe results feed back into the scorer
- Bridges marked as "burned" (recently blocked) are deprioritized
- Effective transports (obfs4, webtunnel, snowflake) are promoted
- Logs are structured for analysis (`monitoring/structured_logger.py`)

#### Recommended Transports for Iran

- **Snowflake** — Highest anti-DPI rating; works during internet cuts (CDN-fronted)
- **WebTunnel** — High anti-DPI; fast HTTPS tunneling
- **obfs4** — Strong obfuscation; blocks quickly once discovered
- **Vanilla** — No obfuscation; blocks immediately

Design principles:
- Use maintained, community-audited transports (obfs4, webtunnel, snowflake from Tor Project)
- Avoid novel, unaudited obfuscation schemes — they get fingerprinted quickly
- Layer multiple strategies: transport rotation + fingerprint randomization + traffic shaping
- Monitor and adapt: deprioritize bridges that stop working, promote ones that do
