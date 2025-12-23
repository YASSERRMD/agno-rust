//! Tools module - provides various toolkits for agents.
//!
//! This module contains implementations of common tools that agents can use:
//! - Calculator: Math operations
//! - DuckDuckGo: Web search
//! - Shell: Command execution
//! - HTTP: API requests
//! - File: Advanced file operations

pub mod calculator;
pub mod duckduckgo;
pub mod shell;

pub use calculator::calculator_toolkit;
pub use duckduckgo::{duckduckgo_toolkit, DuckDuckGoConfig, SearchResult};
pub use shell::{shell_toolkit, ShellConfig};
