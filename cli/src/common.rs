use std::{process::Command};
use colored::Colorize;

/// ✅ Check whether `cmd` exists in PATH and print user-friendly feedback.
pub fn check_command(cmd: &str, friendly_name: &str) -> bool {
    let ok = command_exists(cmd);

    if ok {
        println!("✅ {} is installed.", friendly_name.green());
    } else {
        println!("❌ {} is not installed.", friendly_name.red());
    }

    ok
}

/// 🔍 Cross-platform check if a command exists in PATH.
pub fn command_exists(cmd: &str) -> bool {
    Command::new(if cfg!(windows) { "where" } else { "which" })
        .arg(cmd)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// 🔐 Return `true` if the current process is running as root (Unix) or admin (Windows).
pub fn check_sudo() -> bool {
    if cfg!(target_os = "windows") {
        Command::new("net")
            .arg("session")
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    } else {
        match Command::new("id").arg("-u").output() {
            Ok(output) if output.status.success() => {
                let uid = String::from_utf8_lossy(&output.stdout);
                uid.trim() == "0"
            }
            _ => false,
        }
    }
}

/// 🌐 Open a URL using the default system browser.
pub fn open_url(url: &str) {
    let result = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "start", "", url])
            .spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(url)
            .spawn()
    } else {
        Command::new("xdg-open")
            .arg(url)
            .spawn()
    };

    if let Err(err) = result {
        eprintln!("❌ {}: {}", "Failed to open URL".red(), err);
    }
}

/// 🖥️ Run a shell command (Unix: `sh -c`, Windows: PowerShell).
/// Exits the process if it fails.
pub fn run_shell_command(command: &str) {
    println!("🚀 Running: {}", command.cyan());

    let status = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(&["-Command", command])
            .status()
    } else {
        Command::new("sh")
            .args(&["-c", command])
            .status()
    };

    match status {
        Ok(s) if s.success() => {}
        Ok(s) => {
            let code = s.code().unwrap_or(-1);
            eprintln!(
                "❌ {}: exited with status code {}",
                "Command failed".red(),
                code
            );
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("❌ {}: {}", "Failed to execute command".red(), e);
            std::process::exit(1);
        }
    }
}

/// 🪟 Run a command via PowerShell directly (Windows-only).
pub fn run_powershell_command(command: &str) {
    println!("💻 PowerShell: {}", command.cyan());

    match Command::new("powershell")
        .args(&["-Command", command])
        .status()
    {
        Ok(s) if s.success() => {}
        Ok(s) => {
            let code = s.code().unwrap_or(-1);
            eprintln!(
                "❌ {}: exited with status code {}",
                "PowerShell command failed".red(),
                code
            );
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("❌ {}: {}", "Failed to run PowerShell command".red(), e);
            std::process::exit(1);
        }
    }
}
