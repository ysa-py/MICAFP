# MICAFP — MASTER ENGINEERING DIRECTIVE (v4+v5+v6 Combined)
# Paste this entire document to GLM as one prompt.

You are the execution agent for **MICAFP** (project name and app name — use this name exclusively, on every platform, in every screen and build artifact). MICAFP is a VPN/anti-censorship system currently existing as a Rust/Kotlin native Android app and a parallel Flutter app, both sharing a Rust core (24 transport protocols, obfuscation engines, AI DPI-evasion modules, licensing system, scanner subsystem).

You will execute three combined workstreams, in this fixed order: **(1) Enterprise UI/UX overhaul, (2) License anti-forgery + AI load balancer + AI API layer, (3) iOS + PC expansion.** Read this entire document before writing any code. Confirm your understanding of Section A in writing before starting anything.

---

## SECTION A — RULES YOU MUST NEVER BREAK (apply to all three workstreams, always)

**A1. ZERO DELETION — ADDITIVE ONLY.**
Never delete, remove, disable, comment-out, rename-away, or functionally regress any existing feature, screen, module, protocol, setting, menu item, backend function, or code path in any app (Android, Flutter, or the new iOS/PC apps once they exist). This includes all 24 existing transport protocols, all obfuscation engines, the AI DPI-evasion modules, the licensing/expiry system, the scanner subsystem, diagnostics, and profile management. Everything you build is a NEW LAYER on top of what exists — never a replacement. If a file you must touch also contains unrelated existing logic, preserve that logic exactly. Before every PR, include a written checklist item: "Confirmed: no existing feature, screen, or backend function was removed or disabled."

**A2. NO FABRICATION.**
Every number or status you display (latency, cipher suite, leak-test result, fingerprint score, protocol-switch reasoning, threat level, license verification status) must come from real backend/Rust-core data. If real data isn't wired up yet, label the UI element "Pending backend wiring" — do not fake it, do not claim it's done. Every PR must include real build output, a real diff, and real test logs. Never report a failure as a success. Never claim a compiled artifact (APK, iOS build, PC build) exists or works without attaching real, reproducible build/run evidence.

**A3. HUMAN MERGE GATE.**
Any change touching cryptography, transport protocols, DPI-evasion logic, license/expiry logic, or the Rust-core FFI boundary requires explicit human approval before merge — even if the change looks purely cosmetic or "just a scheduler."

**A4. ONE SCREEN / ONE COMPONENT AT A TIME.**
Submit one screen or component per PR. Do not start the next one until the current one is approved. Each PR must state exactly what real data source each UI element is bound to.

**A5. PLATFORM PARITY.**
Every new screen/feature must eventually exist across all live platforms (Android native, Flutter, and — once workstream 3 begins — iOS and PC), with matching visual language. Build the Rust/Kotlin version of each screen first, get it approved, then match it on other platforms, referencing the approved version as source of truth.

---

## SECTION B — WORKSTREAM 1: ENTERPRISE UI/UX OVERHAUL

### B1. Design target
Calm, minimalist, enterprise-security aesthetic — Stripe Dashboard / Linear / 1Password. NOT gaming, NOT cyberpunk, NOT neon. Banned: neon glow, RGB gradients, cyberpunk fonts, particle effects, glitch animation, excessive motion, aggressive purple/red gaming palettes.

### B2. Design tokens
- Background: dark neutral #0A0E14 → #1A1F29
- One accent color only (deep emerald or petrol blue) — pick one, use everywhere, all platforms
- Body font: Inter / SF Pro. Numeric/technical font: JetBrains Mono / Roboto Mono
- Medium corner radius (not pill-shaped, not sharp)
- Minimal shadows for depth only, no glow
- UI transitions 200–400ms; connection-state transitions 600–800ms
- Subtle single haptic pulse on handshake completion (mobile) — never strong/repeated
- Loading states always carry a real message ("Establishing encrypted handshake…"), never a bare spinner
- Every displayed number animates smoothly between values, never jump-cuts

### B3. Screens to build (additive, in this order)
1. **Dashboard enhancement — Security Status Hub**: large circular status indicator (Protected / Establishing / Vulnerable / Domestic-only — this last state reflects real Phase D behavior where only the international tunnel is down). Three live chips: active protocol, live latency, current threat level. Expandable "Security Layers" list: Encryption (live cipher), DPI Evasion (active protocol), DNS Leak (last check + timestamp), Kill Switch (armed/disarmed).
2. **Protocol Intelligence (new screen)**: "Why this protocol?" card with real reasoning from the AI DPI-evasion module on switch events. 24h horizontal timeline of real protocol switches. Manual override for advanced users with a non-blocking warning. Must surface all 24 existing protocols, none hidden.
3. **Security Center (new screen)**: Cipher Suite Inspector with real selection reasoning. Handshake Verification Badge (PFS confirmed). Key Rotation Counter (real session count). Immutable, read-only, timestamped Audit Log, exportable to PDF/CSV. Auto Leak Test (periodic DNS/WebRTC/IP check with pass/fail + timestamp) alongside the existing manual "run now" button (keep it).
4. **Network Map (new screen)**: node-to-node visualization (Device → Obfuscation Layer → Exit Node) with a subtle animated flow. Split Tunneling panel as a new visual layer over the EXISTING split-tunneling backend — do not reimplement the routing logic.
5. **Diagnostics/Scanner**: restyle only to match B2 tokens. Do not touch detection logic.
6. **Account & License**: add a real status line confirming Phase D + license-signature behavior (see Workstream 2). Underlying logic untouched beyond what Workstream 2 requires.

---

## SECTION C — WORKSTREAM 2: LICENSE ANTI-FORGERY + AI LOAD BALANCER + AI API LAYER

### C1. License/serial anti-forgery system
- Default license expiry: **Azar 19, 1405 (December 10, 2026)**, stored as a single configurable field in a signed license payload — not hardcoded in multiple places.
- Validate against a trusted time source, not solely local device clock.
- License keys must be cryptographically signed (Ed25519 or equivalent) by a private key held only server-side. The client holds only the public verification key, embedded at build time.
- Forbidden: hardcoded "valid" strings, plain date/string comparison without signature verification, trusting local clock alone.
- License payload includes at minimum: license ID, expiry timestamp, signature over all fields.
- Consider a lightweight revocation-list check, but offline devices must still respect the signed expiry already on the license — revocation is not the sole gatekeeper.
- License-check network calls route through the existing obfuscated transport layer — never plain unobfuscated HTTPS to a single easily-blockable endpoint.
- Account & License screen shows real signature-verification status, never a static checkmark.
- Behavior on expiry (unchanged from Phase D): only the international tunnel is cut. Domestic routing, settings, profile, diagnostics, and the AI advisory panel remain fully functional.

### C2. AI-driven protocol load balancer
- New coordination layer on top of the existing 24 protocol implementations — selects among them, does not reimplement them.
- Selection factors in real measured signals: recent connection success/failure per protocol, latency, existing censorship-detection signal.
- Every rotation logs real reasoning to the Immutable Audit Log (e.g., "Protocol X showed elevated failure rate in the last N minutes → switched to Protocol Y") — sourced from actual telemetry, never templated/fabricated text.
- Respects the manual override from B3.2 — never silently overrides a user's pinned protocol choice.
- Touches DPI-evasion logic → Human Merge Gate applies, no exceptions.
- Must never remove, merge, or hide any of the 24 protocols. UI copy must describe real mechanism, not overstate capability with unearned superlatives.

### C3. AI API integration layer (pluggable, provider-agnostic)
- Thin provider-abstraction interface (config-driven adapter: base URL, auth method, request/response mapping) so a new AI provider/model can be added via configuration, not a code rewrite.
- Route AI API calls through the same obfuscated transport layer used for VPN traffic when the direct endpoint is unreachable/filtered — reuse existing obfuscation, do not build a second parallel evasion system.
- Graceful degradation: if a provider is unreachable, show a real "temporarily unavailable" state — never fabricate an AI response.
- No API keys/credentials hardcoded in client code shipped to users.

---

## SECTION D — WORKSTREAM 3: iOS + PC EXPANSION

### D1. Architecture (mandatory approach — do not deviate)
**Do not rewrite the core. Export it. Build thin native UI shells around it.**
- The existing Rust core is compiled as a shared library for each new target: iOS via UniFFI (or cbindgen), PC via a C-compatible API for a Tauri-based app (Rust backend + web-based frontend).
- iOS: native SwiftUI shell calling into the Rust core, implementing the same B3 screen set, same B2 design tokens.
- PC: Tauri-based shell calling into the same Rust core, same screen set, same design tokens.
- All protocol logic, crypto, license verification, and DPI-evasion logic is reused, never reimplemented per platform. Proposals to rewrite VPN/crypto/protocol logic separately in Swift or C++/C# for each platform are explicitly REJECTED — this triples attack surface and forks security-critical logic.
- Platform-required integrations are mandatory, not optional: iOS Network Extension (Packet Tunnel Provider) for tunneling; PC OS-level TUN/TAP or WinTun integration.

### D2. Out of scope
- Forking protocol/crypto/obfuscation logic per platform.
- Any change to Android/Flutter apps beyond what's needed to keep the shared Rust core building for all targets — and if a core API signature must change, update all consuming platforms consistently, never leave one broken.
- App Store distribution mechanics — Apple's review policy for VPN/circumvention apps is a separate non-technical decision to flag to the human reviewer before iOS work begins, not to assume away.

### D3. Execution order for this workstream
1. Confirm understanding of D1 (shared-core architecture) in writing.
2. FFI boundary design document (function signatures) — human review before implementation.
3. FFI implementation (Rust side) with real unit tests — human merge gate.
4. iOS: minimal single-screen shell (Dashboard/Security Status Hub) wired to real FFI calls, with a real Network Extension connect/disconnect path. Submit for review before any other screen.
5. PC: same minimal single-screen approach, same review gate.
6. Only after both approved, proceed screen-by-screen through the rest of the B3 screen set, for iOS and PC respectively, one screen at a time.
7. Full regression pass confirming Android and Flutter apps still build and behave identically to before this workstream.

---

## SECTION E — OVERALL EXECUTION ORDER

1. Confirm understanding of Section A (all five rules) in writing.
2. Workstream 1 (UI/UX), screen by screen, Rust/Kotlin then Flutter, per B3 order.
3. Workstream 2 (license, load balancer, AI API layer), per C1→C2→C3 order, human merge gate on every core-touching change.
4. Workstream 3 (iOS/PC), only after 2 and 3 above are stable, per D3 order.
5. At the end of each workstream: full regression pass confirming zero existing features were removed anywhere.

---

## SECTION F — DEFINITION OF DONE (applies per screen/component, every workstream)

- [ ] All displayed values bound to real backend/Rust-core data, or explicitly marked "Pending backend wiring"
- [ ] Zero existing features/screens/functions removed or altered in behavior (checklist confirmation attached)
- [ ] Real build output + diff + test logs attached — including a real compiled/run artifact where applicable, never a claim without evidence
- [ ] Matches Section B design tokens (no banned visual elements)
- [ ] Human merge gate obtained for any crypto, protocol, DPI-evasion, license, or FFI-boundary change
- [ ] Platform parity maintained or explicitly flagged as pending

---

## SECTION G — NOTE FOR THE HUMAN REVIEWER (not for GLM to act on)

- iOS Network Extensions have tighter memory/background-execution limits than Android — AI/load-balancer logic running inside the extension process may need trimming to fit.
- Apple's App Store review is stricter around VPN/circumvention apps than Google Play; TestFlight/enterprise distribution/sideloading may be the practical path — decide this before iOS work begins.
- Workstream 3 realistically spans months, not a single directive round-trip. Budget review time accordingly.
- No APK, IPA, or PC installer will be delivered by GLM without real build evidence attached — do not expect a "ready to test" artifact until a PR explicitly includes it.

Begin by replying with your understanding of Section A, then propose the first item (Workstream 1, Screen 1 — Dashboard Security Status Hub, Rust/Kotlin) and wait for approval before writing code.
اسم پروژه این MICAFP هست اسم اپلیکیشن هم MICAFP هست باید باشه حتماً بصورت آخر سره تست واقعی کن بصورت کامل متوجه شدی بیشتر توضیح بدم بهت  و قابلیت های پیشرفته اضافه کن با دت ذره بین بررسی کن ببین دقیق  وصل هم نمیشه خطا ها رو رفع کن بصورت کامل خودکار باید باشه حتماً بصورت کامل هوشمند ترین کن با ببین دقیق بعدش بررسی کن ببین دقیق بعدش انجامش بده بصورت کامل خودکار باید باشه حتماً بصورت کامل متوجه شدی بیشتر توضیح بدم بهت انجامش بده بصورت کامل متوجه شدی بیشتر توضیح بدم بهت و  خودت هم  درستش کن APK بساز برام که تست واقعی کنم اگر خطایی داشت خطاها رفع کن بصورت کامل خودکار باید باشه حتماً بصورت کامل هوشمند ترین کن ببین متوجه شدی بیشتر توضیح بدم بهت... غیره قابلیت های پیشرفته اضافه کن با دقت ذره بین بررسی کن ببین دقیق بعدش انجامش بده بصورت کامل خودکار باید باشه حتماً بصورت کامل هوشمند ترین کن بصورت کامل متوجه شدی بیشتر توضیح بدم بهت ولی هیچ چیزی قابلیتی پاک نشه و حذف نکن بصورت کامل متوجه شدی بیشتر توضیح بدم بهت و وصل بشه در فیلترینگ ایران باید وصل بشه بصورت کامل خودکار باید باشه حتماً هوشمند ترین کن حتماً بصورت کامل متوجه شدی بیشتر توضیح بدم بهت 
این پروژه این MICAFPمهندسی بنویس به که خطا نداشته باشه حتماً بصورت کامل متوجه شدی بیشتر توضیح بدم ولی هیچ چیزی قابلیتی پاک نشه و حذف نکن بصورت کامل متوجه شدی بیشتر توضیح بدم بهت و قابلیت های پیشرفته اضافه کن با دقت ذره بین بررسی کن ببین دقیق بعدش انجامش بده انجامش بهش بگو UI UX ولی حرفه‌ای ترین کنه  زیبا ترین کنه هوشمند ترین کنه تخصصی ترین قوی ترین پرسرعت ترین بشه بصورت کامل خودکار باید باشه حتماً بصورت کامل متوجه شدی بیشتر توضیح بدم بهت یک کادر اضافه کن بهش که API هوش مصنوعی هوش مصنوعی جدید ترین میاد باید ساپورت بشه بدون عوض کرده کدها  ساپورت بشه اضافه کنم اگر فیلترینگ فیلتر کن API ها  فایل بک هوش مصنوعی داخلی قوی ترین حرفه‌ای و صاحبوک ولی قوی ترین که ضد فیلترتنگ ایران باید باشه حتماً بصورت کامل خودکار هوشمند ترین باید باشه داینامیک باید باشه حتماً ضد DPI با هوش مصنوعی ایران باید باشه حتماً بصورت کامل متوجه شدی بیشتر توضیح بدم بهت قوی ترین حرفه‌ای جدید برای دور زدن فیلترینگ هوشمند ایران باید باشه حتماً و ضد DPI با هوش مصنوعی ایران باید باشه حتماً ووو غیره قابلیت های پیشرفته اضافه کن برای vpn  لحالت سریال نامبر و لایسنسس داشته باشه و حالت سریال نامبر و لایسنس داشته باشه ضد جعل  باید باشه حتماً ووو غیره قابلیت های پیشرفته اضافه کن با دقت ذره بین بررسی کن ببین دقیق 
دقت ذره بین بررسی کن ببین دقیق ببین دقیق در ایران فیلتر شده یا نشه بهم بگو با دقت ذره بین بررسی کن ببین دقیق بعدش بهم بگو با دقت ذره بین بررسی کن ببین  دقیق چکار فیلترینگ ایران  ن نتونه فیلترینگ هوشمند ایران باید باشه حتماً ضد DPI با هوش مصنوعی ایران باید باشه حتماً بصورت کامل متوجه شدی بیشتر توضیح بدم بده اینترنت بین‌المللی قطع هست باید کار کنه حتماً چکار کنم ضد فیلترینگ هوشمند ایران اذر 18 و 19 اذر 1404 برای تاریخ انقضا باید باشه که بهعداز تاریخ انقضا قطع بشه به اینترنت بین‌المللی بصورت کامل دسترسی قطع حتماً پس چکار کنم قدم به قدم بگو چکار کنم که انجامش بدم بهم بگو با دقت ذره بین بررسی کن ببین دقیق بعدش بهم بگو با دقت ذره بین بررسی کن ببین دقیق استفاده می کردی برای Android و iOS و PC  و برای app vpn پیشرفته می خوام باید باشه بسازه حتماً بصورت کامل متوجه شدی بیشتر توضیح بدم بهت این دوتا SlipNet.tar.xz و MICAFP-UnifiedShield-src.tar.gz بصورت کامل  SlipNet.tar.xz  UI UX دوست دارم ولی حرفه‌ای ترین باید بشه حتماً بصورت کامل خفن ترین تخصصی ترین باید باشه حتماً بصورت کامل متوجه شدی بیشتر توضیح بدم بهت Enterprise بشه UI UX Enterprise عوض بشه بصورت کامل متوجه شدی بیشتر توضیح بدم بهت ولی هیچ چیزی قابلیتی پاک نشه و حذف نکن بصورت کامل متوجه شدی بیشتر توضیح بدم بهت بدون هیچ Perez Holder باید باشه حتماً بصورت کامل خودکار باید باشه حتماً بصورت کامل متوجه شدی بیشتر توضیح بدم بهت و قابلیت های پیشرفته اضافه کن با دقت ذره بین بررسی کن 
Core قابلیت های پیشرفته اضافه کن که ضد فیلترتنگ ایران باید باشه حتماً بصورت کامل خودکار باید باشه حتماً بصورت کامل متوجه شدی بیشتر توضیح بدم ضد DPI با هوش مصنوعی ایران باید باشه حتماً بصورت کامل متوجه شدی بیشتر توضیح بدم و حذف نکن بصورت کامل متوجه شدی بیشتر توضیح بدم بهت و بصورت کامل لود بالانسر عوض بشه هر که شناسایی نشه Core پروتکل ها بصورت کامل خودکار هوشمند لود بالانسر عوض بشه که شناسایی نشه با دقت ذره بین بررسی کن ببین دقیق بعدش انجامش بده بصورت کامل متوجه شدی بیشتر توضیح بدم بهت
