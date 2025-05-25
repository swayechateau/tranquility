// src/command/logs.rs
use crate::log_error;
// src/command/logs.rs
use crate::logger::default_log_path;
use crate::{config::TranquilityConfig, log_warn};

use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
    path::PathBuf,
};

/// Show filtered logs or just print the log path
pub fn show_logs(
    tail: usize,
    level: &str,
    json_only: bool,
    date: Option<String>,
    show_log_path_only: bool,
) {
    let config = TranquilityConfig::load_or_init().unwrap_or_else(|e| {
        log_warn!("fallback", "config", &format!("Using default config due to error: {e}"));
        TranquilityConfig::default().expect("Failed to generate default config")
    });

    let base_dir = config
        .log_file
        .parent()
        .map(PathBuf::from)
        .unwrap_or_else(|| default_log_path().parent().unwrap().to_path_buf());

    let path = resolve_log_file(&base_dir, date);

    if show_log_path_only {
        println!("{}", path.display());
        return;
    }

    if !path.exists() {
        log_warn!("resolve", "log", &format!("❌ Missing: {}", path.display()));
        return;
    }

    if let Ok(f) = File::open(&path) {
        let reader = BufReader::new(f);
        let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

        let filtered: Vec<String> = lines
            .into_iter()
            .rev()
            .filter(|line| {
                if json_only && !line.trim_start().starts_with('{') {
                    return false;
                }

                extract_level(line)
                    .map(|lvl| matches_log_level(&lvl, level))
                    .unwrap_or(true)
            })
            .take(tail)
            .collect();

        println!(
            "\n📄 Showing last {} logs from {}:",
            filtered.len(),
            path.display()
        );
        for line in filtered.into_iter().rev() {
            println!("{}", line);
        }
    } else {
        log_error!("open", "log", &format!("❌ Failed: {}", path.display()));
    }
}

fn resolve_log_file(logs_dir: &PathBuf, date: Option<String>) -> PathBuf {
    let name = if let Some(date_str) = date {
        format!("{}-tranquility.log", date_str)
    } else {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        format!("{}-tranquility.log", today)
    };

    let path = logs_dir.join(name);

    // Create the file silently if it doesn't exist
    if !path.exists() {
        let _ = OpenOptions::new().create(true).append(true).open(&path);
    }

    path
}

fn extract_level(line: &str) -> Option<String> {
    if line.trim_start().starts_with('{') {
        let json: serde_json::Value = serde_json::from_str(line).ok()?;
        json.get("level")?.as_str().map(|s| s.to_lowercase())
    } else {
        line.split_once('[')?
            .1
            .split(']')
            .next()
            .map(|s| s.to_lowercase())
    }
}

fn matches_log_level(entry: &str, min_level: &str) -> bool {
    let level_map = |s: &str| match s {
        "error" => 0,
        "warn" => 1,
        "info" => 2,
        _ => 2,
    };
    level_map(entry) <= level_map(min_level)
}
