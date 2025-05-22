// src/logging.rs

use std::{fs::OpenOptions, io::Write, path::PathBuf};
use serde::Serialize;
use chrono::Utc;

use crate::model::config::TranquilityConfig;
use std::sync::atomic::{AtomicU8, Ordering};

static LOG_LEVEL: AtomicU8 = AtomicU8::new(1); // 0 = error, 1 = warn, 2 = info

pub fn set_log_level(level: &str) {
    let value = match level {
        "error" => 0,
        "warn" => 1,
        _ => 2,
    };
    LOG_LEVEL.store(value, Ordering::Relaxed);
}

fn should_log(level: &str) -> bool {
    let current = LOG_LEVEL.load(Ordering::Relaxed);
    let entry_level = match level {
        "error" => 0,
        "warn" => 1,
        _ => 2,
    };
    entry_level <= current
}

/// A structured log entry that captures all actions
#[derive(Serialize)]
pub struct LogEntry<'a> {
    pub timestamp: String,
    pub level: &'a str,
    pub action: &'a str,
    pub app: &'a str,
    pub status: &'a str,
    pub duration_secs: Option<f64>,
}

/// Log an event using the config-defined log file path
/// Automatically loads the config (use only for occasional logging)
pub fn log_event(
    level: &str,
    action: &str,
    app: &str,
    status: &str,
    duration_secs: Option<f64>,
) {
    let config = TranquilityConfig::load_or_init().unwrap();
    let path = config.log_file;
    log_event_with_path(&path, level, action, app, status, duration_secs);
}

/// More efficient logging interface when config is already loaded
pub fn log_event_with_path(
    path: &PathBuf,
    level: &str,
    action: &str,
    app: &str,
    status: &str,
    duration_secs: Option<f64>,
) {
    let entry = LogEntry {
        timestamp: Utc::now().to_rfc3339(),
        level,
        action,
        app,
        status,
        duration_secs,
    };
    log_(path, &entry);
}

/// Appends both a human-readable and JSON entry to the log file
pub fn log_(path: &PathBuf, entry: &LogEntry) {
    if !should_log(entry.level) {
        return;
    }

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        let json = serde_json::to_string(entry).unwrap_or_default();
        let human = format!(
            "[{}] [{}] {} {} - {} ({:?})",
            entry.timestamp,
            entry.level.to_uppercase(),
            entry.action,
            entry.app,
            entry.status,
            entry.duration_secs
        );

        let _ = writeln!(file, "{human}");
        let _ = writeln!(file, "{json}");
    }
}

