// src/logger.rs
use std::fs::{self, OpenOptions};
use std::io::{stderr, stdout, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU8, Ordering};

use chrono::Utc;
use serde::Serialize;

use crate::model::config::TranquilityConfig;

const MAX_LOG_SIZE: u64 = 5 * 1024 * 1024; // 5 MB

static LOG_LEVEL: AtomicU8 = AtomicU8::new(1); // 0 = error, 1 = warn, 2 = info

/// Enum to represent log destinations
pub enum LogDestination {
    File(PathBuf),
    Stdout,
    Stderr,
}

/// A structured log entry
#[derive(Serialize)]
pub struct LogEntry<'a> {
    pub timestamp: String,
    pub level: &'a str,
    pub action: &'a str,
    pub app: &'a str,
    pub status: &'a str,
    pub duration_secs: Option<f64>,
    pub source: Option<&'a str>,
}

/// Determine if a message should be logged based on level
fn should_log(level: &str) -> bool {
    let current = LOG_LEVEL.load(Ordering::Relaxed);
    let entry_level = match level {
        "error" => 0,
        "warn" => 1,
        _ => 2,
    };
    entry_level <= current
}

/// Use config-based log destination, fallback to default path
pub fn log_event(
    level: &str,
    action: &str,
    app: &str,
    status: &str,
    duration_secs: Option<f64>,
) {
    let destination = TranquilityConfig::load_or_init()
        .ok()
        .map(|cfg| {
            if cfg.log_file.as_os_str().is_empty() {
                LogDestination::File(default_log_path())
            } else {
                LogDestination::File(cfg.log_file)
            }
        })
        .unwrap_or(LogDestination::File(default_log_path()));

    log_to_full(level, action, app, status, duration_secs, None, destination);
}

/// Core logger with source and destination
pub fn log_to_full(
    level: &str,
    action: &str,
    app: &str,
    status: &str,
    duration_secs: Option<f64>,
    source: Option<&str>,
    destination: LogDestination,
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
        source,
    };

    match destination {
        LogDestination::File(ref path) => {
            rotate_if_needed(path);
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
                write_log(&mut file, &entry);
            } else {
                let mut err = stderr();
                write_log(&mut err, &entry);
            }
        }
        LogDestination::Stdout => {
            let mut out = stdout();
            write_log(&mut out, &entry);
        }
        LogDestination::Stderr => {
            let mut err = stderr();
            write_log(&mut err, &entry);
        }
    }
}

/// Writes human-readable and JSON log
fn write_log(writer: &mut dyn Write, entry: &LogEntry) {
    let json = serde_json::to_string(entry).unwrap_or_default();
    let human = format!(
        "[{}] [{}] {} {} - {} ({:?}){}",
        entry.timestamp,
        entry.level.to_uppercase(),
        entry.action,
        entry.app,
        entry.status,
        entry.duration_secs,
        match entry.source {
            Some(src) => format!(" [{}]", src),
            None => "".to_string(),
        }
    );

    let _ = writeln!(writer, "{human}");
    let _ = writeln!(writer, "{json}");
}

/// Rotates the log file if too large
fn rotate_if_needed(path: &PathBuf) {
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.len() > MAX_LOG_SIZE {
            let rotated = path.with_extension("old");
            let _ = fs::rename(path, rotated);
        }
    }
}

/// Returns a default log file path
pub fn default_log_path() -> PathBuf {
    let base = dirs::config_dir()
        .map(|p| p.join("tranquility"))
        .unwrap_or_else(|| PathBuf::from("/tmp/tranquility"));

    let logs = base.join("logs");
    let _ = fs::create_dir_all(&logs);

    logs.join(format!(
        "{}-tranquility.log",
        chrono::Local::now().format("%Y-%m-%d")
    ))
}

