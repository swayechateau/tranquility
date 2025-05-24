// src/logger.rs
use std::{fs::OpenOptions, io::{stderr, Write}, path::PathBuf};
use serde::Serialize;
use chrono::Utc;

use crate::model::config::TranquilityConfig;
use std::sync::atomic::{AtomicU8, Ordering};

static LOG_LEVEL: AtomicU8 = AtomicU8::new(1); // 0 = error, 1 = warn, 2 = info

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

/// Log an event using the config-defined log file path (fallbacks to stderr if needed)
pub fn log_event(
    level: &str,
    action: &str,
    app: &str,
    status: &str,
    duration_secs: Option<f64>,
) {
    let path_opt = TranquilityConfig::load_or_init()
        .ok()
        .and_then(|cfg| Some(cfg.log_file));

    log_event_with_path(path_opt.as_ref(), level, action, app, status, duration_secs);
}

/// Logging with known config, fallback if path is invalid
pub fn log_event_with_path(
    path: Option<&PathBuf>,
    level: &str,
    action: &str,
    app: &str,
    status: &str,
    duration_secs: Option<f64>,
) {
    if !should_log(level) {
        return;
    }

    let entry = LogEntry {
        timestamp: Utc::now().to_rfc3339(),
        level,
        action,
        app,
        status,
        duration_secs,
    };

    // Try writing to file, fallback to stderr
    if let Some(path) = path {
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
            write_log(&mut file, &entry);
            return;
        }
    }

    // Fallback to stderr
    let mut err = stderr();
    write_log(&mut err, &entry);
}

/// Write human and JSON to any writer
fn write_log(writer: &mut dyn Write, entry: &LogEntry) {
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

    let _ = writeln!(writer, "{human}");
    let _ = writeln!(writer, "{json}");
}
