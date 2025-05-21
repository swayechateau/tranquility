// src/common.rs
use std::process::{Command, Stdio, Output};
use colored::Colorize;

use crate::{print_error, print_info, print_warn};

/// ✅ Check whether `cmd` exists in PATH and print user-friendly feedback.
pub fn check_command(cmd: &str, friendly_name: &str) -> bool {
    let ok = command_exists(cmd);

    if ok {
        print_info!("✅ {} is installed.", friendly_name.green());
    } else {
        print_warn!("❌ {} is not installed.", friendly_name.red());
    }

    ok
}

/// 🔍 Cross-platform check if a command exists in PATH.
pub fn command_exists(cmd: &str) -> bool {
    Command::new(if cfg!(windows) { "where" } else { "which" })
        .arg(cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
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
        print_error!("❌ {}: {}", "Failed to open URL".red(), err);
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
            print_error!(
                "❌ {}: exited with status code {}",
                "Command failed".red(),
                code
            );
            std::process::exit(1);
        }
        Err(e) => {
            print_error!("❌ {}: {}", "Failed to execute command".red(), e);
            std::process::exit(1);
        }
    }
}



pub struct ShellCommand {
    pub command: String,
    pub args: Vec<String>,
    pub use_sudo: bool,
}

impl ShellCommand {
    pub fn new(command: &str) -> Self {
        ShellCommand {
            command: command.to_string(),
            args: vec![],
            use_sudo: false,
        }
    }

    pub fn with_args(mut self, args: &[&str]) -> Self {
        self.args = args.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_sudo(mut self, enable: bool) -> Self {
        self.use_sudo = enable;
        self
    }

    pub fn execute(&self) -> std::io::Result<Output> {
        let mut cmd = if cfg!(target_os = "windows") {
            let mut c = Command::new("cmd");
            c.arg("/C").arg(&self.command);
            c
        } else {
            let mut full_cmd = if self.use_sudo {
                vec!["sudo".to_string(), self.command.clone()]
            } else {
                vec![self.command.clone()]
            };
            full_cmd.extend(self.args.clone());
            let mut c = Command::new(&full_cmd[0]);
            for arg in &full_cmd[1..] {
                c.arg(arg);
            }
            c
        };

        cmd.output()
    }
}
