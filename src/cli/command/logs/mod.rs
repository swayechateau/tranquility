// Module: Command/Logs
// Location: cli/src/command/logs/mod.rs
use clap::{Args, ValueEnum};
use strum::Display;

use crate::{config::TranquilityConfig, core::logger::default_log_path, log_error, log_warn};

use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum, Display)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

#[derive(Args, Debug)]
pub struct LogsCommand {
    /// Only show JSON lines
    #[arg(long)]
    json_only: bool,
    /// Only show the log file path
    #[arg(long)]
    path: bool,

    /// Show last N log lines
    #[arg(long, default_value_t = 50)]
    tail: usize,

    /// Filter logs by date (YYYY-MM-DD)
    #[arg(long)]
    date: Option<String>,

    /// Filter logs by level
    #[arg(long, default_value = "info")]
    level: String,
}

/// Show filtered logs or just print the log path
pub fn show_logs(
    tail: usize,
    level: &str,
    json_only: bool,
    date: Option<String>,
    show_log_path_only: bool,
) {
    let config = TranquilityConfig::load_once();

    let base_dir = config
        .log_file()
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

pub fn handle_logs_command(cmd: LogsCommand, dry_run: bool) {
    if dry_run {
        println!("Would show logs with options: {:?}", cmd);
        return;
    }
    show_logs(cmd.tail, &cmd.level, cmd.json_only, cmd.date, cmd.path);
}
