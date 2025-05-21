use std::process::{Command, Output};

use colored::Colorize;
use crate::print_error;

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

    /// Run the command and print stdout/stderr (with colors).
    pub fn run_verbose(&self) {
        println!("🚀 Running: {}", self.as_string().cyan());

        match self.execute() {
            Ok(output) => {
                if output.status.success() {
                    let out = String::from_utf8_lossy(&output.stdout);
                    if !out.trim().is_empty() {
                        println!("{}", out.green());
                    }
                } else {
                    let err = String::from_utf8_lossy(&output.stderr);
                    print_error!("❌ Command failed:\n{}", err.red());
                }
            }
            Err(e) => {
                print_error!("❌ Failed to execute: {}", e);
            }
        }
    }
}
