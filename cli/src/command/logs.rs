// src/command/logs.rs
use crate::config::TranquilityConfig;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub fn show_logs(tail: usize, level: &str, json_only: bool, date: Option<String>) {
    let config = TranquilityConfig::load_or_init().unwrap();
    let base_dir = config.log_file.parent().unwrap().to_path_buf();

    let path = match resolve_log_file(&base_dir, date) {
        Some(p) => p,
        None => {
            eprintln!("❌ Could not find matching log file.");
            return;
        }
    };

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

fn resolve_log_file(logs_dir: &PathBuf, date: Option<String>) -> Option<PathBuf> {
    let name = if let Some(date_str) = date {
        format!("tranquility-{}.log", date_str)
    } else {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        format!("tranquility-{}.log", today)
    };
    let path = logs_dir.join(name);
    if path.exists() {
        Some(path)
    } else {
        None
    }
}

fn extract_level(line: &str) -> Option<String> {
    if line.trim_start().starts_with('{') {
        // JSON format
        let json: serde_json::Value = serde_json::from_str(line).ok()?;
        json.get("level")?.as_str().map(|s| s.to_lowercase())
    } else {
        // human-readable: [2025-05-21T..] [INFO] ...
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
