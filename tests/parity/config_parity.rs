use std::{collections::HashMap, path::Path, process::Command};

use serde_json::{json, Value};
use torshield_ir_ultra::config::load_config_from_map;

const CONFIG_KEYS: &[&str] = &[
    "MAX_WORKERS",
    "CONNECTION_TIMEOUT",
    "SSL_TIMEOUT",
    "MAX_RETRIES",
    "MAX_TEST_PER_TYPE",
    "RECENT_HOURS",
    "HISTORY_RETENTION_DAYS",
    "BRIDGE_DIR",
    "EXPORT_DIR",
    "HISTORY_FILE",
    "SCORES_FILE",
    "REPO_URL",
    "IS_GITHUB",
    "CF_N_SLOTS",
    "CF_ACCOUNT_IDS",
    "CF_API_TOKENS",
    "CF_AI_GATEWAY_URLS",
    "CF_VALID_SLOTS",
    "CEREBRAS_API_KEY",
    "PORTKEY_API_KEY",
    "PORTKEY_GATEWAY_URL",
    "PORTKEY_VIRTUAL_KEYS",
    "GROQ_API_KEY",
    "GITHUB_TOKEN",
    "GITHUB_REPOSITORY",
    "GITHUB_SHA",
    "GH_PAT_AUTOFIX",
    "GH_REPO_OWNER",
    "GH_REPO_NAME",
    "TELEGRAM_BOT_TOKEN",
    "TELEGRAM_CHAT_ID",
    "TELEGRAM_UPLOAD",
    "HTTP_PROXY",
    "HTTPS_PROXY",
    "USE_TORPROJECT_SCRAPER",
    "USE_MOAT_API",
    "USE_BRIDGEDB_API",
    "USE_TELEGRAM_SOURCES",
    "USE_STATIC_BRIDGES",
    "USE_GITHUB_SOURCES",
    "DEEP_TEST",
    "IRAN_PREFERRED_PORTS",
    "IRAN_CDN_FRONTS",
    "NIN_MODE",
    "IRAN_BRIDGE_PRIORITIZATION_ENABLED",
    "IRAN_BRIDGE_PRIORITIZATION_WEIGHT_PORT",
    "IRAN_BRIDGE_PRIORITIZATION_WEIGHT_TRANSPORT",
    "IRAN_BRIDGE_PRIORITIZATION_WEIGHT_RECENCY",
    "IRAN_BRIDGE_PRIORITIZATION_WEIGHT_REACHABILITY",
    "RIPE_ATLAS_API_KEY",
    "ANTI_DPI_MODE",
    "ANTI_FILTER_MODE",
    "TORSHIELD_IRAN_MODE",
    "AUTO_DEBUG_MODE",
    "UTLS_EVASION_MODE",
    "UTLS_PROFILE_ROTATION",
    "ELITE_REGISTRY_ENABLED",
    "ELITE_REGISTRY_REFRESH_HRS",
    "CIRCUIT_BREAKER_ENABLED",
    "CIRCUIT_BREAKER_MAX_FAILURES",
    "CIRCUIT_BREAKER_RESET_SECS",
    "SESSION_BLACKLIST_DURATION_SECS",
    "TELEMETRY_ENABLED",
    "TELEMETRY_AUTO_DEBUG_THRESHOLD",
    "TELEMETRY_LOG_MAX_MB",
    "IRST_HIGH_CENSORSHIP_START",
    "IRST_HIGH_CENSORSHIP_END",
    "IRST_ULTRA_STEALTH_START",
    "IRST_ULTRA_STEALTH_END",
];

fn python_config(envs: &[(&str, &str)]) -> std::process::Output {
    let script = format!(
        r#"
import json
import config
keys = {keys}
print(json.dumps({{k: getattr(config, k) for k in keys}}, sort_keys=True, separators=(",", ":")))
"#,
        keys = serde_json::to_string(CONFIG_KEYS).expect("static keys serialize")
    );
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut cmd = Command::new("/usr/bin/python3");
    cmd.current_dir(repo_root)
        .env_clear()
        .env("PYTHONPATH", repo_root)
        .arg("-c")
        .arg(script);
    for (key, value) in envs {
        cmd.env(key, value);
    }
    cmd.output().expect("python config helper must execute")
}

fn python_config_json(envs: &[(&str, &str)]) -> Value {
    let output = python_config(envs);
    assert!(
        output.status.success(),
        "python config helper failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("python helper emits JSON")
}

fn rust_config_json(envs: &[(&str, &str)]) -> Value {
    let map = envs
        .iter()
        .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
        .collect::<HashMap<_, _>>();
    load_config_from_map(&map).expect("rust config should parse")
}

#[test]
fn parity_default_environment_matches_python() {
    assert_eq!(rust_config_json(&[]), python_config_json(&[]));
}

#[test]
fn parity_overrides_slots_paths_booleans_and_provider_values() {
    let envs = [
        ("MAX_WORKERS", "7"),
        ("CONNECTION_TIMEOUT", "2.5"),
        ("SSL_TIMEOUT", "3.25"),
        ("MAX_RETRIES", "4"),
        ("MAX_TEST_PER_TYPE", "8"),
        ("RECENT_HOURS", "9"),
        ("HISTORY_RETENTION_DAYS", "10"),
        ("BRIDGE_DIR", "tmp/bridges"),
        ("EXPORT_DIR", "tmp/export"),
        ("REPO_URL", "https://example.invalid/repo"),
        ("GITHUB_ACTIONS", "true"),
        ("CF_N_SLOTS", "3"),
        ("CF_ACCOUNT_ID_1", " acc1 "),
        ("CF_API_TOKEN_1", " tok1 "),
        ("CF_AI_GATEWAY_URL_1", " gw1 "),
        ("CF_ACCOUNT_ID_2", ""),
        ("CF_API_TOKEN_2", "tok2"),
        ("CF_ACCOUNT_ID_3", "acc3"),
        ("CF_API_TOKEN_3", "tok3"),
        ("CEREBRAS_API_KEY", "cerebras"),
        ("PORTKEY_API_KEY", "portkey"),
        ("PORTKEY_GATEWAY_URL", "https://portkey.example"),
        ("PORTKEY_VIRTUAL_KEY_1", "vk1"),
        ("PORTKEY_VIRTUAL_KEY_3", "vk3"),
        ("GROQ_API_KEY", "groq"),
        ("GITHUB_TOKEN", "gh"),
        ("GITHUB_REPOSITORY", "owner/repo"),
        ("GITHUB_SHA", "abc"),
        ("GH_PAT_AUTOFIX", "pat"),
        ("GH_REPO_OWNER", "owner"),
        ("GH_REPO_NAME", "repo"),
        ("TELEGRAM_BOT_TOKEN", "bot"),
        ("TELEGRAM_CHAT_ID", "chat"),
        ("TELEGRAM_UPLOAD", "TRUE"),
        ("HTTP_PROXY", "socks5://127.0.0.1:1080"),
        ("HTTPS_PROXY", "https://proxy"),
        ("USE_TORPROJECT_SCRAPER", "False"),
        ("USE_MOAT_API", "false"),
        ("USE_BRIDGEDB_API", "TRUE"),
        ("USE_TELEGRAM_SOURCES", "true"),
        ("USE_STATIC_BRIDGES", "false"),
        ("USE_GITHUB_SOURCES", "false"),
        ("DEEP_TEST", "true"),
        ("NIN_MODE", "true"),
        ("IRAN_BRIDGE_PRIORITIZATION_ENABLED", "true"),
        ("IRAN_BRIDGE_PRIORITIZATION_WEIGHT_PORT", "1.5"),
        ("IRAN_BRIDGE_PRIORITIZATION_WEIGHT_TRANSPORT", "2.5"),
        ("IRAN_BRIDGE_PRIORITIZATION_WEIGHT_RECENCY", "3.5"),
        ("IRAN_BRIDGE_PRIORITIZATION_WEIGHT_REACHABILITY", "4.5"),
        ("RIPE_ATLAS_API_KEY", "ripe"),
        ("ANTI_DPI_MODE", "true"),
        ("ANTI_FILTER_MODE", "true"),
        ("TORSHIELD_IRAN_MODE", "true"),
        ("AUTO_DEBUG_MODE", "true"),
        ("UTLS_EVASION_MODE", "false"),
        ("UTLS_PROFILE_ROTATION", "60"),
        ("ELITE_REGISTRY_ENABLED", "false"),
        ("ELITE_REGISTRY_REFRESH_HRS", "12"),
        ("CIRCUIT_BREAKER_ENABLED", "false"),
        ("CIRCUIT_BREAKER_MAX_FAILURES", "5"),
        ("CIRCUIT_BREAKER_RESET_SECS", "42.5"),
        ("SESSION_BLACKLIST_DURATION_SECS", "99.5"),
        ("TELEMETRY_ENABLED", "false"),
        ("TELEMETRY_AUTO_DEBUG_THRESHOLD", "6"),
        ("TELEMETRY_LOG_MAX_MB", "20"),
        ("IRST_HIGH_CENSORSHIP_START", "17"),
        ("IRST_HIGH_CENSORSHIP_END", "2"),
        ("IRST_ULTRA_STEALTH_START", "21"),
        ("IRST_ULTRA_STEALTH_END", "22"),
    ];
    let rust = rust_config_json(&envs);
    let python = python_config_json(&envs);
    assert_eq!(rust, python);
    assert_eq!(rust["CF_VALID_SLOTS"], json!([["acc1", "tok1", "gw1"], ["acc3", "tok3", ""]]));
}

#[test]
fn parity_bool_values_only_match_true_after_lowercase() {
    let envs = [("TELEGRAM_UPLOAD", " true "), ("DEEP_TEST", "yes")];
    let rust = rust_config_json(&envs);
    assert_eq!(rust, python_config_json(&envs));
    assert_eq!(rust["TELEGRAM_UPLOAD"], json!(false));
    assert_eq!(rust["DEEP_TEST"], json!(false));
}

#[test]
fn parity_invalid_integer_fails_like_python_import() {
    let envs = [("MAX_WORKERS", "not-an-int")];
    assert!(!python_config(&envs).status.success());
    let map = HashMap::from([("MAX_WORKERS".to_string(), "not-an-int".to_string())]);
    let err = load_config_from_map(&map).expect_err("rust should reject invalid int");
    assert_eq!(err.variable, "MAX_WORKERS");
    assert_eq!(err.kind, "int");
}

#[test]
fn parity_invalid_float_fails_like_python_import() {
    let envs = [("SSL_TIMEOUT", "not-a-float")];
    assert!(!python_config(&envs).status.success());
    let map = HashMap::from([("SSL_TIMEOUT".to_string(), "not-a-float".to_string())]);
    let err = load_config_from_map(&map).expect_err("rust should reject invalid float");
    assert_eq!(err.variable, "SSL_TIMEOUT");
    assert_eq!(err.kind, "float");
}
