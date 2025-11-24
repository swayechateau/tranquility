// Module: Core/Logger
// Location: cli/src/core/logger.rs
use std::fs::{self, OpenOptions};
use std::io::{stderr, stdout, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

use chrono::Utc;
use colored::Colorize;
use serde::Serialize;

use crate::config::{LogOutput, TranquilityConfig};

const MAX_LOG_SIZE: u64 = 5 * 1024 * 1024; // 5 MB

static LOG_LEVEL: AtomicU8 = AtomicU8::new(1); // 0 = error, 1 = warn, 2 = info
static DEBUG_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn set_debug(enabled: bool) {
    let env_debug = std::env::var("RUST_LOG")
        .map(|val| val.to_lowercase().contains("debug"))
        .unwrap_or(false);

    let active = enabled || env_debug;
    DEBUG_ENABLED.store(active, Ordering::Relaxed);

    if active {
        eprintln!("[logger] Debug mode enabled");
    }
}

pub enum LogDestination {
    Primary(PathBuf),
    Stdout,
}

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

fn should_log(level: &str) -> bool {
    let current = LOG_LEVEL.load(Ordering::Relaxed);
    let entry_level = match level {
        "error" => 0,
        "warn" => 1,
        _ => 2,
    };
    entry_level <= current
}

pub fn log_event(
    level: &str,
    action: &str,
    app: &str,
    status: &str,
    duration_secs: Option<f64>,
    source: Option<&str>,
) {
    let cfg = TranquilityConfig::load_once();
    let destination = match cfg.log_output {
        LogOutput::Stdout => LogDestination::Stdout,
        LogOutput::Primary => {
            let path = if cfg.log_file().as_os_str().is_empty() {
                default_log_path()
            } else {
                cfg.log_file()
            };
            LogDestination::Primary(path)
        }
    };

    log_to_full(
        level,
        action,
        app,
        status,
        duration_secs,
        source,
        destination,
    );
}

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

    // Always show errors on stderr
    if entry.level == "error" {
        let _ = write_log(&mut stderr(), &entry);
    }

    match destination {
        LogDestination::Primary(ref path) => {
            rotate_if_needed(path);
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
                write_log(&mut file, &entry);

                // Also show info/warn to stdout in debug mode
                if DEBUG_ENABLED.load(Ordering::Relaxed) {
                    match entry.level {
                        "warn" | "info" => {
                            let _ = write_log(&mut stdout(), &entry);
                        }
                        _ => {}
                    }
                }
            } else {
                let _ = write_log(&mut stderr(), &entry);
            }
        }
        LogDestination::Stdout => {
            // Don't double-print errors here
            if entry.level != "error" {
                let _ = write_log(&mut stdout(), &entry);
            }
        }
    }
}

fn write_log(writer: &mut dyn Write, entry: &LogEntry) {
    let json = serde_json::to_string(entry).unwrap_or_default();

    let colorized_status = match entry.level {
        "error" => entry.status.red().bold(),
        "warn" => entry.status.yellow().bold(),
        "info" => entry.status.blue(),
        _ => entry.status.normal(),
    };

    let level_str = match entry.level {
        "error" => "ERROR".red().bold(),
        "warn" => "WARN".yellow().bold(),
        "info" => "INFO".blue(),
        _ => entry.level.normal(),
    };

    let human = format!(
        "[{}] [{}] {} {} - {} ({:?}){}",
        entry.timestamp,
        level_str,
        entry.action,
        entry.app,
        colorized_status,
        entry.duration_secs,
        match entry.source {
            Some(src) => format!(" [{}]", src),
            None => "".to_string(),
        }
    );

    let _ = writeln!(writer, "{human}");
    let _ = writeln!(writer, "{json}");
}

fn rotate_if_needed(path: &PathBuf) {
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.len() > MAX_LOG_SIZE {
            let rotated = path.with_extension("old");
            let _ = fs::rename(path, rotated);
        }
    }
}

pub fn default_log_path() -> PathBuf {
    let base = dirs::config_dir()
        .and_then(|p| Some(p.join("tranquility")))
        .unwrap_or_else(|| PathBuf::from("/tmp/tranquility"));

    let logs = base.join("logs");
    if let Err(e) = fs::create_dir_all(&logs) {
        eprintln!("Failed to create log directory: {}", e);
        return PathBuf::from("/tmp/tranquility/default.log");
    }

    logs.join(format!(
        "{}-tranquility.log",
        chrono::Local::now().format("%Y-%m-%d")
    ))
}
