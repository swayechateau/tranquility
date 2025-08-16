pub mod font;
pub mod logger;
pub mod shell;
pub mod print;
pub mod fs;

pub fn expand_home(input: &str) -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| "~".to_string());

    if input.starts_with("~/") {
        return input.replacen("~", &home, 1);
    }

    if input.contains("$HOME") {
        return input.replace("$HOME", &home);
    }

    if input.contains("%HOME%") {
        return input.replace("%HOME%", &home);
    }

    input.to_string()
}

use std::path::{PathBuf};

#[cfg(target_os = "windows")]
pub fn default_hosts_path() -> PathBuf {
    PathBuf::from(r"C:\Windows\System32\drivers\etc\hosts")
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn default_hosts_path() -> PathBuf {
    PathBuf::from("/etc/hosts")
}
