use std::{
    fs,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::engine::error::{Error, ErrorCode};

// ---------------------------------------------------------------------------
// Path helpers
// ---------------------------------------------------------------------------

fn get_log_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("tranquility")
        .join("logs")
}

fn resolve_log_file(date: Option<&str>) -> PathBuf {
    let date_str = date
        .map(|d| d.to_string())
        .unwrap_or_else(|| chrono::Local::now().format("%Y-%m-%d").to_string());
    get_log_dir().join(format!("{}-tranquility.log", date_str))
}

// ---------------------------------------------------------------------------
// Level filtering
// ---------------------------------------------------------------------------

/// Try to extract a lowercase log level from a log line.
///
/// Supports two formats:
/// - JSON lines:  `{"level": "info", ...}`
/// - Text lines:  `... [INFO] ...`
fn extract_level(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    if trimmed.starts_with('{') {
        let json: serde_json::Value = serde_json::from_str(trimmed).ok()?;
        return json.get("level")?.as_str().map(|s| s.to_lowercase());
    }

    // Try to find [LEVEL] anywhere in the line
    let after_bracket = line.split_once('[')?.1;
    let level_str = after_bracket.split(']').next()?;
    if level_str.chars().all(|c| c.is_ascii_alphabetic()) {
        Some(level_str.to_lowercase())
    } else {
        None
    }
}

/// Returns `true` if the line's level passes the given filter.
///
/// Filter semantics (case-insensitive):
/// - `"error"` → only errors
/// - `"warn"`  → warnings and errors
/// - anything else (e.g. `"info"`) → everything
fn matches_level_filter(line: &str, filter: &str) -> bool {
    match extract_level(line) {
        None => true, // no level found — include the line
        Some(level) => match filter.to_lowercase().as_str() {
            "error" => level == "error",
            "warn" | "warning" => matches!(level.as_str(), "warn" | "warning" | "error"),
            _ => true,
        },
    }
}

// ---------------------------------------------------------------------------
// Public capability
// ---------------------------------------------------------------------------

/// Show log entries from the Tranquility log file.
///
/// Log files are expected at `$XDG_DATA_HOME/tranquility/logs/{date}-tranquility.log`.
pub async fn show(
    tail: usize,
    level: String,
    json_only: bool,
    date: Option<String>,
    path_only: bool,
) -> Result<(), Error> {
    let log_path = resolve_log_file(date.as_deref());

    if path_only {
        println!("{}", log_path.display());
        return Ok(());
    }

    if !log_path.exists() {
        eprintln!(
            "No log file found for {}.\nExpected: {}",
            date.as_deref().unwrap_or("today"),
            log_path.display()
        );
        eprintln!(
            "Hint: logs are written to {} when the application is configured to log to a file.",
            get_log_dir().display()
        );
        return Ok(());
    }

    let file = fs::File::open(&log_path).map_err(|e| {
        Error::from_code(ErrorCode::ProcessFailure).with_context("open", e.to_string())
    })?;

    let reader = BufReader::new(file);
    let all_lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

    // Collect matching lines (most recent first) up to `tail`
    let matching: Vec<&str> = all_lines
        .iter()
        .rev()
        .filter(|line| {
            let trimmed = line.trim_start();
            if json_only && !trimmed.starts_with('{') {
                return false;
            }
            matches_level_filter(line, &level)
        })
        .take(tail)
        .map(|s| s.as_str())
        .collect();

    if matching.is_empty() {
        println!("No log entries match the current filters.");
        return Ok(());
    }

    println!(
        "\n--- {} (last {} matching entries) ---",
        log_path.display(),
        matching.len()
    );

    // Re-reverse so output is chronological
    for line in matching.into_iter().rev() {
        println!("{}", line);
    }

    Ok(())
}
