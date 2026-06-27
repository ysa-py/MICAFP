#![recursion_limit = "256"]

//! Rust migration anchor crate for TorShield-IR Ultra VIP Edition.
//!
//! Python modules remain the source of truth until each module has a parity
//! test proving byte-identical behavior against its Rust replacement.

pub mod config;
pub mod generated_json_loader;
pub mod results_writer;

/// Cargo features mirroring pytest markers used for selective test execution.
pub const PYTEST_MARKER_FEATURES: &[&str] = &[
    "network",
    "iran",
    "slow",
    "tor",
    "iran_bridge",
    "bridge",
    "dpi",
    "nin",
];
