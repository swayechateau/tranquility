// src/common.rs

use std::{env, process::Command};

/// Check whether `cmd` exists (and, optionally, can print its version).
/// Prints a ✅ or ❌ with `friendly_name` and returns true if available.
pub fn check_command(cmd: &str, friendly_name: &str, check_version: bool) -> bool {
    let args = if check_version {
        // special case: Go’s version flag is `version` not `--version`
        if cmd == "go" { vec!["version"] } else { vec!["--version"] }
    } else {
        Vec::new()
    };

    let ok = Command::new(cmd)
        .args(&args)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if ok {
        println!("✅ {} is installed.", friendly_name);
    } else {
        println!("❌ {} is not installed.", friendly_name);
    }
    ok
}

/// Return `true` if we’re running as root (Unix) or admin (Windows).
pub fn check_sudo() -> bool {
    if cfg!(target_os = "windows") {
        // On Windows, `net session` requires admin privileges
        Command::new("net")
            .arg("session")
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    } else {
        // On Unix, `id -u` prints the effective UID; root has UID 0
        match Command::new("id").arg("-u").output() {
            Ok(output) if output.status.success() => {
                // Convert bytes to string and trim whitespace
                let uid = String::from_utf8_lossy(&output.stdout);
                uid.trim() == "0"
            }
            _ => false,
        }
    }
}

/// Open a URL in the user’s default browser (cross‐platform via `open` crate).
pub fn open_url(url: &str) {
    // On Windows, `start` is a shell built-in, so we go via `cmd /C`
    // The empty string after `start` is the window title placeholder.
    let result = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "start", "", url])
            .spawn()
    // macOS has `open`
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(url)
            .spawn()
    // Linux (and most other unices) use `xdg-open`
    } else {
        Command::new("xdg-open")
            .arg(url)
            .spawn()
    };

    if let Err(err) = result {
        eprintln!("❌ Failed to open URL {}: {}", url, err);
    }
}

/// On Linux: parses `/proc/cpuinfo` for the “model name”
/// On macOS: uses `sysctl -n machdep.cpu.brand_string`
/// On Windows: uses `wmic cpu get Name`
pub fn determine_cpu_brand() -> String {
    if cfg!(target_os = "linux") {
        if let Ok(out) = std::fs::read_to_string("/proc/cpuinfo") {
            for line in out.lines() {
                if let Some(rest) = line.strip_prefix("model name\t: ") {
                    return rest.trim().to_string();
                }
            }
        }
    } else if cfg!(target_os = "macos") {
        if let Ok(output) = Command::new("sysctl")
            .args(&["-n", "machdep.cpu.brand_string"])
            .output()
        {
            if output.status.success() {
                if let Ok(s) = String::from_utf8(output.stdout) {
                    return s.trim().to_string();
                }
            }
        }
    } else if cfg!(target_os = "windows") {
        if let Ok(output) = Command::new("wmic")
            .args(&["cpu", "get", "Name", "/value"])
            .output()
        {
            if output.status.success() {
                if let Ok(s) = String::from_utf8(output.stdout) {
                    // output like "Name=Intel(R) ...\r\n\r\n"
                    for line in s.lines() {
                        if let Some(v) = line.strip_prefix("Name=") {
                            return v.trim().to_string();
                        }
                    }
                }
            }
        }
    }
    "Unknown".into()
}

/// Run a shell command under `sh -c` (Unix) or via PowerShell (Windows).
/// Exits on error.
pub fn run_shell_command(command: &str) {
    println!("🚀 Running: {}", command);

    // Spawn via PowerShell on Windows, `sh -c` elsewhere
    let status_res = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(&["-Command", command])
            .status()
    } else {
        Command::new("sh")
            .args(&["-c", command])
            .status()
    };

    match status_res {
        // Everything succeeded
        Ok(status) if status.success() => return,

        // The command ran but exited with a non-zero code
        Ok(status) => {
            let code = status.code().map_or(-1, |c| c);
            eprintln!(
                "❌ Command `{}` exited with non-zero status code: {}",
                command, code
            );
            std::process::exit(1);
        }

        // Failed to spawn / execute at all
        Err(error) => {
            eprintln!("❌ Failed to execute `{}`: {}", command, error);
            std::process::exit(1);
        }
    }
}

/// Convenience for calling PowerShell directly.
pub fn run_powershell_command(command: &str) {
    if let Err(err) = Command::new("powershell")
        .args(&["-Command", command])
        .status()
    {
        eprintln!("❌ PowerShell command failed `{}`: {}", command, err);
        std::process::exit(1);
    }
}

/// Human‐readable OS name.
pub fn determine_os() -> String {
    if cfg!(target_os = "windows") {
        "Windows"
    } else if cfg!(target_os = "macos") {
        "macOS"
    } else if cfg!(target_os = "linux") {
        "Linux"
    } else {
        "Unknown"
    }
    .into()
}

/// CPU architecture, e.g. “x86_64”, “aarch64”
pub fn determine_arch() -> String {
    env::consts::ARCH.into()
}

/// On Linux, runs `lsb_release -d`; otherwise “Unknown”
pub fn determine_distro() -> String {
    if cfg!(target_os = "linux") {
        if let Ok(output) = Command::new("lsb_release").arg("-d").output() {
            if output.status.success() {
                if let Ok(desc) = String::from_utf8(output.stdout) {
                    // desc is like "Description:\tUbuntu 24.04\n"
                    return desc
                        .splitn(2, '\t')
                        .nth(1)
                        .unwrap_or(&desc)
                        .trim()
                        .to_string();
                }
            }
        }
    }
    "Unknown".into()
}

{
    "name": "swaye.dev",
    "username": "swaye",
    "host": "swaye.dev",
    "port": 22,
    "privateKey_location": "/home/kevin/.ssh/swaye.dev_ed25519",
}