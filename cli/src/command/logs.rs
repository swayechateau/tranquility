// src/command/logs.rs
use crate::config::TranquilityConfig;
use crate::logger::default_log_path;

use std::{
    fs::File,
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
    let config = TranquilityConfig::load_or_init().unwrap_or_else(|_| {
        // fallback if config fails
        TranquilityConfig {
            applications_file: PathBuf::new(),
            vps_file: PathBuf::new(),
            log_file: default_log_path(),
        }
    });

    let base_dir = config
        .log_file
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| default_log_path().parent().unwrap().to_path_buf());

    let path = resolve_log_file(&base_dir, date);

    if show_log_path_only {
        println!("{}", path.display());
        return;
    }

    if !path.exists() {
        eprintln!("❌ Could not find matching log file: {}", path.display());
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

                if let Some(lvl) = extract_level(line) {
                    matches_log_level(&lvl, level)
                } else {
                    true
                }
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
        eprintln!("❌ Could not open log file: {}", path.display());
    }
}

fn resolve_log_file(logs_dir: &PathBuf, date: Option<String>) -> PathBuf {
    let name = if let Some(date_str) = date {
        format!("{}-tranquility.log", date_str)
    } else {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        format!("{}-tranquility.log", today)
    };

    logs_dir.join(name)
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
