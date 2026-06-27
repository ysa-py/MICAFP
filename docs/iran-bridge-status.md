# ✅ TorShield-IR — Iran Bridge Status Report

**Generated:** `2026-06-27 03:09 UTC`<br>
**Pipeline:** Python scraper → Go iran_tester → Rust bridge-probe → OONI correlator

---

## Summary

| Metric | Value |
| :--- | :--- |
| Total bridges analysed | `1440` |
| Composite score > 0.5 | `449` (31%) |
| OONI clean (Iran) | `0` |
| OONI anomaly/blocked | `0` |
| OONI no data | `1440` |
| Quality gate (≥ 30 %) | `PASS ✅` |

---

## Iran DPI Intelligence

Iran's censorship infrastructure (SIAM) uses:
- **TLS fingerprinting** — JA3 hash matching for known Tor patterns (`e7d705a3286e19ea42f587b344ee6865`)
- **Port-based blocking** — Ports 9001, 9030, 9050 are consistently blocked
- **IP-based blocking** — Known Tor relay/bridge IPs are blocklisted within 24–48 h of first use
- **Traffic volume anomaly detection** — Unusual traffic shapes are flagged

### Recommended Transport Priority for Iran

```
Snowflake → WebTunnel (CDN-fronted) → obfs4 (port 443) → meek-lite → vanilla
```

---

## Top 20 Working Bridges (composite score > 0.5)

| Host:Port | Transport | TCP | OONI-IR | Score |
| :--- | :---: | :---: | :---: | :---: |
| `137.220.65.28:9500` | 🟡 | ✅ | ❓ | `0.68` |
| `158.69.55.8:2025` | 🟡 | ✅ | ❓ | `0.68` |
| `158.69.55.8:445` | 🟡 | ✅ | ❓ | `0.68` |
| `158.69.55.8:25001` | 🟡 | ✅ | ❓ | `0.68` |
| `121.127.33.34:443` | 🟡 | ✅ | ❓ | `0.68` |
| `173.21.185.152:55443` | 🟡 | ✅ | ❓ | `0.68` |
| `134.209.113.118:9000` | 🟡 | ✅ | ❓ | `0.68` |
| `129.153.78.39:9959` | 🟡 | ✅ | ❓ | `0.68` |
| `159.65.125.21:9002` | 🟡 | ✅ | ❓ | `0.68` |
| `141.95.3.138:1025` | 🟡 | ✅ | ❓ | `0.68` |
| `142.93.128.78:1356` | 🟡 | ✅ | ❓ | `0.68` |
| `103.149.168.242:9443` | 🟡 | ✅ | ❓ | `0.68` |
| `141.144.242.150:9999` | 🟡 | ✅ | ❓ | `0.68` |
| `152.53.129.122:465` | 🟡 | ✅ | ❓ | `0.68` |
| `141.95.17.236:9333` | 🟡 | ✅ | ❓ | `0.68` |
| `152.53.237.196:59001` | 🟡 | ✅ | ❓ | `0.68` |
| `109.90.115.13:26875` | 🟡 | ✅ | ❓ | `0.68` |
| `141.14.15.217:443` | 🟡 | ✅ | ❓ | `0.68` |
| `109.104.14.213:443` | 🟡 | ✅ | ❓ | `0.68` |
| `134.209.113.118:9001` | 🟡 | ✅ | ❓ | `0.68` |

---

## Classification Definitions

| Status | Meaning |
| :--- | :--- |
| `iran_likely_working` | OONI shows clean results from Iranian probes in last 7 days |
| `iran_likely_blocked` | OONI shows anomaly/confirmed block from Iranian probes |
| `iran_frequently_blocked` | Recurrence rate > 2 blocks per 30-day period |
| `iran_unknown` | No OONI data from Iranian probes; TCP reachable from GitHub Actions |
| `tcp_unreachable` | TCP connection failed from GitHub Actions runner (likely globally down) |
| `iran_asn_blocked` | Bridge IP resolves to an Iranian ISP ASN — excluded from all packs |

---

## DPI Risk Flags

| Flag | Description |
| :--- | :--- |
| `iran_dpi_high_risk` | Bridge uses a JA3 fingerprint or port known to Iran's DPI blocklist |
| `iran_port_high_risk` | Bridge is on port 9001, 9030, or 9050 |
| `domain_front_degraded` | WebTunnel front domain resolves to a non-CDN IP |
| `domain_front_cdn_ok` | WebTunnel front domain resolves to a known CDN (Cloudflare, Azure, Fastly) |

---

*This report is generated automatically by [TorShield-IR](https://github.com/user/torshield-ir).*
