// src/shell/command.rs
use std::process::{Command, Stdio, Output};
use colored::Colorize;
use crate::{print_error, print_info, print_warn};

#[derive(Debug)]
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

    /// Get a string representation of the full command.
    pub fn as_string(&self) -> String {
        let mut parts = Vec::new();
        if self.use_sudo && !cfg!(windows) {
            parts.push("sudo".to_string());
        }
        parts.push(self.command.clone());
        parts.extend(self.args.clone());
        parts.join(" ")
    }

    /// Print the command instead of executing it (dry run).
    pub fn dry_run(&self) {
        println!("💡 [Dry Run] {}", self.as_string().cyan());
    }

    /// Execute the command and return raw output.
    pub fn execute(&self) -> std::io::Result<Output> {
        let mut cmd = if cfg!(target_os = "windows") {
            let mut full_command = self.command.clone();
            if !self.args.is_empty() {
                full_command.push(' ');
                full_command.push_str(&self.args.join(" "));
            }
            let mut c = Command::new("cmd");
            c.args(&["/C", &full_command]);
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

    pub fn execute_with_dry_run(&self, dry_run: bool) -> Option<std::io::Result<Output>> {
        if dry_run {
            println!("💡 [Dry Run] Would run: {}", self.as_string().cyan());
            return None;
        }

        Some(self.execute())
    }

    /// Run the command and print stdout/stderr (with colors).
    pub fn run_verbose(&self, dry_run: bool) {
        println!("🚀 Running: {}", self.as_string().cyan());

        if dry_run {
            println!("💡 [Dry Run] Skipped execution.");
            return;
        }

        match self.execute() {
            Ok(output) => {
                if output.status.success() {
                    let out = String::from_utf8_lossy(&output.stdout);
                    if !out.trim().is_empty() {
                        println!("{}", out.green());
                    }
                    if !output.stderr.is_empty() {
                        let err = String::from_utf8_lossy(&output.stderr);
                        eprintln!("{}", err.yellow());
                    }
                } else {
                    let err = String::from_utf8_lossy(&output.stderr);
                    eprintln!("{}", err.red());
                }
            }
            Err(e) => {
                eprintln!("❌ Failed to execute: {}", e);
            }
        }
    }

    /// Executes and returns just the status result
    pub fn run_and_return_status(&self, dry_run: bool) -> Option<bool> {
        if dry_run {
            self.dry_run();
            return Some(true);
        }

        match self.execute() {
            Ok(output) => Some(output.status.success()),
            Err(_) => Some(false),
        }
    }

    /// For shell scripts or piped commands (`sh -c "echo foo && bar"`)
    pub fn from_shell(script: &str, sudo: bool) -> Self {
        ShellCommand {
            command: "sh".to_string(),
            args: vec!["-c".to_string(), script.to_string()],
            use_sudo: sudo,
        }
    }

    /// For PowerShell commands on Windows
    pub fn from_powershell(script: &str, sudo: bool) -> Self {
        ShellCommand {
            command: "powershell".to_string(),
            args: vec!["-Command".to_string(), script.to_string()],
            use_sudo: sudo,
        }
    }
}

/// 🔐 Return `true` if the current process is running as root (Unix) or admin (Windows).
fn check_sudo() -> bool {
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
