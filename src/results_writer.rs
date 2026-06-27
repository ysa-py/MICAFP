//! Parity port of `results_writer.py` file-output classification helpers.
//!
//! The Python original remains the source of truth. This module ports the pure
//! bridge categorisation and sorted-file writing path from
//! `results_writer.py::write_result_files`, with typed I/O errors instead of
//! process exits so parity tests can compare every branch deterministically.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use serde_json::Value;
const WORKING_STATUSES: &[&str] = &["iran_likely_working"];
const UNKNOWN_REACHABLE: &[&str] = &["iran_unknown"];
const BLOCKED_STATUSES: &[&str] = &["iran_likely_blocked", "iran_frequently_blocked"];
const WORKING_TRANSPORTS: &[&str] = &["obfs4", "webtunnel", "vanilla", "snowflake", "meek_lite"];
const GLOBAL_TRANSPORTS: &[&str] = &["obfs4", "webtunnel", "vanilla"];

/// File-generation failures for the Rust `results_writer.py` parity port.
#[derive(Debug)]
pub enum ResultsWriterError {
    /// Creating the bridge output directory failed.
    CreateDir {
        path: PathBuf,
        source: std::io::Error,
    },
    /// Writing one of the generated bridge files failed.
    WriteFile {
        path: PathBuf,
        source: std::io::Error,
    },
    /// Reading back a generated file for the mandatory integrity assertion failed.
    ReadFile {
        path: PathBuf,
        source: std::io::Error,
    },
    /// The mandatory sorted/deduplicated/no-blank post-write assertion failed.
    Integrity { path: PathBuf },
    /// Python-compatible AttributeError for malformed bridge records.
    AttributeError { message: String },
}

impl std::fmt::Display for ResultsWriterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreateDir { path, source } => {
                write!(
                    f,
                    "failed to create bridge output directory {}: {source}",
                    path.display()
                )
            }
            Self::WriteFile { path, source } => {
                write!(
                    f,
                    "failed to write generated bridge file {}: {source}",
                    path.display()
                )
            }
            Self::ReadFile { path, source } => {
                write!(
                    f,
                    "failed to read generated bridge file {}: {source}",
                    path.display()
                )
            }
            Self::Integrity { path } => {
                write!(
                    f,
                    "integrity assertion failed for {}: file is not sorted and deduplicated",
                    path.display()
                )
            }
            Self::AttributeError { message } => f.write_str(message),
        }
    }
}

impl std::error::Error for ResultsWriterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CreateDir { source, .. }
            | Self::WriteFile { source, .. }
            | Self::ReadFile { source, .. } => Some(source),
            Self::Integrity { .. } | Self::AttributeError { .. } => None,
        }
    }
}

/// Categorise bridges and write all `results_writer.py` bridge text outputs.
///
/// Behavior traced to `results_writer.py::write_result_files`:
/// * ignores bridges with blank or missing `line`;
/// * writes lexicographically sorted, deduplicated files;
/// * Tier 1 (`iran_likely_working`) wins over Tier 2 for each transport;
/// * Tier 2 includes `iran_unknown` bridges when TCP-reachable, and always for
///   `snowflake`/`webtunnel` because Python treats those transports specially;
/// * blocked and global files are produced independently of working tiers.
pub fn write_result_files(
    bridge_dir: &Path,
    bridges: &[Value],
) -> Result<BTreeMap<String, usize>, ResultsWriterError> {
    fs::create_dir_all(bridge_dir).map_err(|source| ResultsWriterError::CreateDir {
        path: bridge_dir.to_path_buf(),
        source,
    })?;

    let mut stats = BTreeMap::new();
    let mut t1_by_transport = empty_transport_map(WORKING_TRANSPORTS);
    let mut t2_by_transport = empty_transport_map(WORKING_TRANSPORTS);
    let mut blocked_lines: Vec<String> = Vec::new();
    let mut global_by_transport = empty_transport_map(GLOBAL_TRANSPORTS);

    for bridge in bridges {
        let bridge = bridge
            .as_object()
            .ok_or_else(|| ResultsWriterError::AttributeError {
                message: format!(
                    "'{}' object has no attribute 'get'",
                    python_type_name_for_value(bridge)
                ),
            })?;

        let line = match bridge.get("line") {
            Some(Value::String(line)) => line.trim().to_string(),
            Some(value) => {
                return Err(ResultsWriterError::AttributeError {
                    message: format!(
                        "'{}' object has no attribute 'strip'",
                        python_type_name_for_value(value)
                    ),
                });
            }
            None => String::new(),
        };
        if line.is_empty() {
            continue;
        }

        let transport = bridge
            .get("transport")
            .and_then(Value::as_str)
            .unwrap_or("unknown");
        let status = bridge
            .get("iran_status")
            .and_then(Value::as_str)
            .unwrap_or("");
        let tcp_ok = bridge.get("tcp_reachable").is_some_and(python_truthy);

        if WORKING_STATUSES.contains(&status) {
            if let Some(bucket) = t1_by_transport.get_mut(transport) {
                bucket.push(line.clone());
            }
        }

        if UNKNOWN_REACHABLE.contains(&status)
            && (tcp_ok || matches!(transport, "snowflake" | "webtunnel"))
        {
            if let Some(bucket) = t2_by_transport.get_mut(transport) {
                bucket.push(line.clone());
            }
        }

        if BLOCKED_STATUSES.contains(&status) {
            blocked_lines.push(line.clone());
        }

        if (tcp_ok || transport == "snowflake") && global_by_transport.contains_key(transport) {
            if let Some(bucket) = global_by_transport.get_mut(transport) {
                bucket.push(line);
            }
        }
    }

    let mut all_working = Vec::new();
    for transport in WORKING_TRANSPORTS {
        let t1_lines = t1_by_transport.remove(*transport).unwrap_or_default();
        let t2_lines = t2_by_transport.remove(*transport).unwrap_or_default();
        let combined = if !t1_lines.is_empty() {
            t1_lines
        } else {
            t2_lines
        };
        if combined.is_empty() {
            continue;
        }
        let filename = format!("iran_likely_working_{transport}.txt");
        let count = write_sorted_file(&bridge_dir.join(&filename), &combined)?;
        stats.insert(filename, count);
        all_working.extend(combined);
    }

    let all_path = bridge_dir.join("iran_likely_working_all.txt");
    stats.insert(
        "iran_likely_working_all.txt".to_string(),
        write_sorted_file(&all_path, &all_working)?,
    );
    assert_integrity(&all_path)?;

    stats.insert(
        "iran_blocked.txt".to_string(),
        write_sorted_file(&bridge_dir.join("iran_blocked.txt"), &blocked_lines)?,
    );

    for transport in GLOBAL_TRANSPORTS {
        let lines = global_by_transport.remove(*transport).unwrap_or_default();
        let filename = format!("tested_global_{transport}.txt");
        stats.insert(
            filename.clone(),
            write_sorted_file(&bridge_dir.join(filename), &lines)?,
        );
    }

    Ok(stats)
}

fn python_truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(value) => *value,
        Value::Number(number) => {
            number.as_i64().is_some_and(|n| n != 0)
                || number.as_u64().is_some_and(|n| n != 0)
                || number.as_f64().is_some_and(|n| n != 0.0)
        }
        Value::String(value) => !value.is_empty(),
        Value::Array(value) => !value.is_empty(),
        Value::Object(value) => !value.is_empty(),
    }
}

fn python_type_name_for_value(value: &Value) -> &'static str {
    match value {
        Value::Null => "NoneType",
        Value::Bool(_) => "bool",
        Value::Number(number) if number.is_i64() || number.is_u64() => "int",
        Value::Number(_) => "float",
        Value::String(_) => "str",
        Value::Array(_) => "list",
        Value::Object(_) => "dict",
    }
}

fn empty_transport_map(transports: &[&str]) -> BTreeMap<String, Vec<String>> {
    transports
        .iter()
        .map(|transport| ((*transport).to_string(), Vec::new()))
        .collect()
}

fn write_sorted_file(path: &Path, lines: &[String]) -> Result<usize, ResultsWriterError> {
    let clean: BTreeSet<String> = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect();
    let mut text = clean.iter().cloned().collect::<Vec<_>>().join("\n");
    if !clean.is_empty() {
        text.push('\n');
    }
    fs::write(path, text).map_err(|source| ResultsWriterError::WriteFile {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(clean.len())
}

fn assert_integrity(path: &Path) -> Result<(), ResultsWriterError> {
    let text = fs::read_to_string(path).map_err(|source| ResultsWriterError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;
    let lines: Vec<&str> = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let expected: Vec<&str> = lines
        .iter()
        .copied()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    if lines != expected {
        return Err(ResultsWriterError::Integrity {
            path: path.to_path_buf(),
        });
    }
    Ok(())
}
