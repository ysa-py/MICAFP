//! Parity port of `config.py` environment-derived constants.
//!
//! Python remains the source of truth until this module's parity tests stay
//! branch-complete. This Rust module evaluates the same environment-variable
//! defaults and parse failures without touching the process environment when
//! tests provide an explicit map.

use std::{collections::HashMap, env, path::Path};

use serde_json::{json, Value};

/// Error raised while evaluating Python-compatible configuration values.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConfigError {
    pub variable: &'static str,
    pub kind: &'static str,
    pub value: String,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to parse {} as {}: {:?}",
            self.variable, self.kind, self.value
        )
    }
}

impl std::error::Error for ConfigError {}

/// Load `config.py`-compatible values from the real process environment.
pub fn load_config_from_env() -> Result<Value, ConfigError> {
    let values: HashMap<String, String> = env::vars().collect();
    load_config_from_map(&values)
}

/// Load `config.py`-compatible values from an explicit environment map.
///
/// Behavior traced to `config.py`: string defaults come from `os.getenv`, bools
/// use `.lower() == "true"`, numeric fields use Python `int()`/`float()` style
/// parsing for the covered finite values, Cloudflare slot values are stripped,
/// and path fields are derived from `BRIDGE_DIR`.
pub fn load_config_from_map(envs: &HashMap<String, String>) -> Result<Value, ConfigError> {
    let max_workers = parse_i64(envs, "MAX_WORKERS", "150")?;
    let connection_timeout = parse_f64(envs, "CONNECTION_TIMEOUT", "8")?;
    let ssl_timeout = parse_f64(envs, "SSL_TIMEOUT", "6")?;
    let max_retries = parse_i64(envs, "MAX_RETRIES", "2")?;
    let max_test_per_type = parse_i64(envs, "MAX_TEST_PER_TYPE", "1000")?;
    let recent_hours = parse_i64(envs, "RECENT_HOURS", "72")?;
    let history_retention_days = parse_i64(envs, "HISTORY_RETENTION_DAYS", "45")?;

    let bridge_dir = get(envs, "BRIDGE_DIR", "bridge");
    let export_dir = get(envs, "EXPORT_DIR", "export");
    let history_file = join_path(&bridge_dir, "bridge_history.json");
    let scores_file = join_path(&bridge_dir, "bridge_scores.json");

    let repo_url = get(
        envs,
        "REPO_URL",
        "https://raw.githubusercontent.com/YOUR_USERNAME/YOUR_REPO/refs/heads/main",
    );
    let is_github = get(envs, "GITHUB_ACTIONS", "") == "true";

    let cf_n_slots = parse_i64(envs, "CF_N_SLOTS", "11")?;
    let mut cf_account_ids = Vec::new();
    let mut cf_api_tokens = Vec::new();
    let mut cf_ai_gateway_urls = Vec::new();
    let mut cf_valid_slots = Vec::new();
    for i in 1..=cf_n_slots {
        let acc = get(envs, &format!("CF_ACCOUNT_ID_{i}"), "")
            .trim()
            .to_string();
        let tok = get(envs, &format!("CF_API_TOKEN_{i}"), "")
            .trim()
            .to_string();
        let gw = get(envs, &format!("CF_AI_GATEWAY_URL_{i}"), "")
            .trim()
            .to_string();
        if !acc.is_empty() && !tok.is_empty() {
            cf_valid_slots.push(json!([acc.clone(), tok.clone(), gw.clone()]));
        }
        cf_account_ids.push(Value::String(acc));
        cf_api_tokens.push(Value::String(tok));
        cf_ai_gateway_urls.push(Value::String(gw));
    }

    let portkey_virtual_keys = (1..=3)
        .map(|i| Value::String(get(envs, &format!("PORTKEY_VIRTUAL_KEY_{i}"), "")))
        .collect::<Vec<_>>();

    Ok(json!({
        "MAX_WORKERS": max_workers,
        "CONNECTION_TIMEOUT": connection_timeout,
        "SSL_TIMEOUT": ssl_timeout,
        "MAX_RETRIES": max_retries,
        "MAX_TEST_PER_TYPE": max_test_per_type,
        "RECENT_HOURS": recent_hours,
        "HISTORY_RETENTION_DAYS": history_retention_days,
        "BRIDGE_DIR": bridge_dir,
        "EXPORT_DIR": export_dir,
        "HISTORY_FILE": history_file,
        "SCORES_FILE": scores_file,
        "REPO_URL": repo_url,
        "IS_GITHUB": is_github,
        "CF_N_SLOTS": cf_n_slots,
        "CF_ACCOUNT_IDS": cf_account_ids,
        "CF_API_TOKENS": cf_api_tokens,
        "CF_AI_GATEWAY_URLS": cf_ai_gateway_urls,
        "CF_VALID_SLOTS": cf_valid_slots,
        "CEREBRAS_API_KEY": get(envs, "CEREBRAS_API_KEY", ""),
        "PORTKEY_API_KEY": get(envs, "PORTKEY_API_KEY", ""),
        "PORTKEY_GATEWAY_URL": get(envs, "PORTKEY_GATEWAY_URL", "https://api.portkey.ai/v1"),
        "PORTKEY_VIRTUAL_KEYS": portkey_virtual_keys,
        "GROQ_API_KEY": get(envs, "GROQ_API_KEY", ""),
        "GITHUB_TOKEN": get(envs, "GITHUB_TOKEN", ""),
        "GITHUB_REPOSITORY": get(envs, "GITHUB_REPOSITORY", ""),
        "GITHUB_SHA": get(envs, "GITHUB_SHA", ""),
        "GH_PAT_AUTOFIX": get(envs, "GH_PAT_AUTOFIX", ""),
        "GH_REPO_OWNER": get(envs, "GH_REPO_OWNER", ""),
        "GH_REPO_NAME": get(envs, "GH_REPO_NAME", ""),
        "TELEGRAM_BOT_TOKEN": get(envs, "TELEGRAM_BOT_TOKEN", ""),
        "TELEGRAM_CHAT_ID": get(envs, "TELEGRAM_CHAT_ID", ""),
        "TELEGRAM_UPLOAD": bool_var(envs, "TELEGRAM_UPLOAD", "false"),
        "HTTP_PROXY": get(envs, "HTTP_PROXY", ""),
        "HTTPS_PROXY": get(envs, "HTTPS_PROXY", ""),
        "USE_TORPROJECT_SCRAPER": bool_var(envs, "USE_TORPROJECT_SCRAPER", "true"),
        "USE_MOAT_API": bool_var(envs, "USE_MOAT_API", "true"),
        "USE_BRIDGEDB_API": bool_var(envs, "USE_BRIDGEDB_API", "true"),
        "USE_TELEGRAM_SOURCES": bool_var(envs, "USE_TELEGRAM_SOURCES", "false"),
        "USE_STATIC_BRIDGES": bool_var(envs, "USE_STATIC_BRIDGES", "true"),
        "USE_GITHUB_SOURCES": bool_var(envs, "USE_GITHUB_SOURCES", "true"),
        "DEEP_TEST": bool_var(envs, "DEEP_TEST", "false"),
        "IRAN_PREFERRED_PORTS": [443, 80, 8080, 8443, 2083, 2087, 2096],
        "IRAN_CDN_FRONTS": [
            "fastly.net", "cdn.arvancloud.com", "arvancloud.ir", "cloudfront.net",
            "azureedge.net", "ajax.aspnetcdn.com", "googlevideo.com", "gstatic.com"
        ],
        "NIN_MODE": bool_var(envs, "NIN_MODE", "false"),
        "IRAN_BRIDGE_PRIORITIZATION_ENABLED": bool_var(envs, "IRAN_BRIDGE_PRIORITIZATION_ENABLED", "false"),
        "IRAN_BRIDGE_PRIORITIZATION_WEIGHT_PORT": parse_f64(envs, "IRAN_BRIDGE_PRIORITIZATION_WEIGHT_PORT", "1.0")?,
        "IRAN_BRIDGE_PRIORITIZATION_WEIGHT_TRANSPORT": parse_f64(envs, "IRAN_BRIDGE_PRIORITIZATION_WEIGHT_TRANSPORT", "1.0")?,
        "IRAN_BRIDGE_PRIORITIZATION_WEIGHT_RECENCY": parse_f64(envs, "IRAN_BRIDGE_PRIORITIZATION_WEIGHT_RECENCY", "1.0")?,
        "IRAN_BRIDGE_PRIORITIZATION_WEIGHT_REACHABILITY": parse_f64(envs, "IRAN_BRIDGE_PRIORITIZATION_WEIGHT_REACHABILITY", "1.0")?,
        "RIPE_ATLAS_API_KEY": get(envs, "RIPE_ATLAS_API_KEY", ""),
        "ANTI_DPI_MODE": bool_var(envs, "ANTI_DPI_MODE", "false"),
        "ANTI_FILTER_MODE": bool_var(envs, "ANTI_FILTER_MODE", "false"),
        "TORSHIELD_IRAN_MODE": bool_var(envs, "TORSHIELD_IRAN_MODE", "false"),
        "AUTO_DEBUG_MODE": bool_var(envs, "AUTO_DEBUG_MODE", "false"),
        "UTLS_EVASION_MODE": bool_var(envs, "UTLS_EVASION_MODE", "true"),
        "UTLS_PROFILE_ROTATION": parse_i64(envs, "UTLS_PROFILE_ROTATION", "30")?,
        "ELITE_REGISTRY_ENABLED": bool_var(envs, "ELITE_REGISTRY_ENABLED", "true"),
        "ELITE_REGISTRY_REFRESH_HRS": parse_i64(envs, "ELITE_REGISTRY_REFRESH_HRS", "6")?,
        "CIRCUIT_BREAKER_ENABLED": bool_var(envs, "CIRCUIT_BREAKER_ENABLED", "true"),
        "CIRCUIT_BREAKER_MAX_FAILURES": parse_i64(envs, "CIRCUIT_BREAKER_MAX_FAILURES", "3")?,
        "CIRCUIT_BREAKER_RESET_SECS": parse_f64(envs, "CIRCUIT_BREAKER_RESET_SECS", "300")?,
        "SESSION_BLACKLIST_DURATION_SECS": parse_f64(envs, "SESSION_BLACKLIST_DURATION_SECS", "3600")?,
        "TELEMETRY_ENABLED": bool_var(envs, "TELEMETRY_ENABLED", "true"),
        "TELEMETRY_AUTO_DEBUG_THRESHOLD": parse_i64(envs, "TELEMETRY_AUTO_DEBUG_THRESHOLD", "2")?,
        "TELEMETRY_LOG_MAX_MB": parse_i64(envs, "TELEMETRY_LOG_MAX_MB", "10")?,
        "IRST_HIGH_CENSORSHIP_START": parse_i64(envs, "IRST_HIGH_CENSORSHIP_START", "18")?,
        "IRST_HIGH_CENSORSHIP_END": parse_i64(envs, "IRST_HIGH_CENSORSHIP_END", "1")?,
        "IRST_ULTRA_STEALTH_START": parse_i64(envs, "IRST_ULTRA_STEALTH_START", "20")?,
        "IRST_ULTRA_STEALTH_END": parse_i64(envs, "IRST_ULTRA_STEALTH_END", "23")?,
    }))
}

fn get(envs: &HashMap<String, String>, key: &str, default: &str) -> String {
    envs.get(key)
        .cloned()
        .unwrap_or_else(|| default.to_string())
}

fn bool_var(envs: &HashMap<String, String>, key: &str, default: &str) -> bool {
    get(envs, key, default).to_lowercase() == "true"
}

fn parse_i64(
    envs: &HashMap<String, String>,
    key: &'static str,
    default: &'static str,
) -> Result<i64, ConfigError> {
    let value = get(envs, key, default);
    value.parse::<i64>().map_err(|_| ConfigError {
        variable: key,
        kind: "int",
        value,
    })
}

fn parse_f64(
    envs: &HashMap<String, String>,
    key: &'static str,
    default: &'static str,
) -> Result<f64, ConfigError> {
    let value = get(envs, key, default);
    value.parse::<f64>().map_err(|_| ConfigError {
        variable: key,
        kind: "float",
        value,
    })
}

fn join_path(base: &str, child: &str) -> String {
    Path::new(base).join(child).to_string_lossy().into_owned()
}
