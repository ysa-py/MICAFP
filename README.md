# 🌐 Tor Bridges Ultra Collector

> Auto-collected, tested, and Iran-scored Tor bridges.<br>
> GitHub Actions runs every hour — fresh bridges always available.<br>
> **Last update:** `2026-06-28 15:01 UTC`

## ⚠️ Notes for Iran Users

- **Internet cut (شبکه ملی):** Use `export/iran_cut_pack.txt` — contains Snowflake and WebTunnel bridges that survive NIN.
- **Normal censorship:** Use `export/iran_pack.txt` — top-ranked obfs4/WebTunnel bridges for Iran's DPI.
- **Port 443 bridges** are prioritised — Iran almost never blocks HTTPS.
- **IPv4 is more stable** than IPv6 inside Iran.

## ✅ Tested & Active (Recommended)

| Transport | IPv4 Tested | Count |
| :--- | :--- | :--- |
| **obfs4** | [obfs4_tested.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/bridge/obfs4_tested.txt) | **0** |
| **WebTunnel** | [webtunnel_tested.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/bridge/webtunnel_tested.txt) | **0** |
| **Snowflake** | [snowflake_tested.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/bridge/snowflake_tested.txt) | **0** |
| **Vanilla** | [vanilla_tested.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/bridge/vanilla_tested.txt) | **0** |
| **meek-lite** | [meek_lite_tested.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/bridge/meek_lite_tested.txt) | **0** |

## 🕐 Fresh Bridges (Last 72h)

| Transport | IPv4 | Count | IPv6 | Count |
| :--- | :--- | :--- | :--- | :--- |
| **obfs4** | [obfs4_72h.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/bridge/obfs4_72h.txt) | **8** | [obfs4_72h_ipv6.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/bridge/obfs4_72h_ipv6.txt) | **4** |
| **WebTunnel** | [webtunnel_72h.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/bridge/webtunnel_72h.txt) | **0** | [webtunnel_72h_ipv6.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/bridge/webtunnel_72h_ipv6.txt) | **1** |
| **Vanilla** | [vanilla_72h.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/bridge/vanilla_72h.txt) | **5** | [vanilla_72h_ipv6.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/bridge/vanilla_72h_ipv6.txt) | **0** |

## 📦 Full Archive

| Transport | IPv4 | Count | IPv6 | Count |
| :--- | :--- | :--- | :--- | :--- |
| **obfs4** | [obfs4.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/bridge/obfs4.txt) | **527** | [obfs4_ipv6.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/bridge/obfs4_ipv6.txt) | **282** |
| **WebTunnel** | [webtunnel.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/bridge/webtunnel.txt) | **179** | [webtunnel_ipv6.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/bridge/webtunnel_ipv6.txt) | **2** |
| **Snowflake** | [snowflake.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/bridge/snowflake.txt) | **4** | — | — |
| **Vanilla** | [vanilla.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/bridge/vanilla.txt) | **456** | [vanilla_ipv6.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/bridge/vanilla_ipv6.txt) | **0** |
| **meek-lite** | [meek_lite.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/bridge/meek_lite.txt) | **2** | — | — |

## 🇮🇷 Iran Optimised Packs

| Pack | Description |
| :--- | :--- |
| [iran_pack.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/export/iran_pack.txt) | Top 100 bridges ranked by Iran effectiveness score |
| [iran_cut_pack.txt](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/export/iran_cut_pack.txt) | Bridges for internet cut / شبکه ملی scenarios |
| [bridges_api.json](https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main/export/bridges_api.json) | Machine-readable JSON API |

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
