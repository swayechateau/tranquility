// Module: Shell/Command
// Location: cli/src/shell/command.rs
use colored::Colorize;
use std::process::{Command, Output, Stdio};
use crate::{print_error, print_info, print_warn};

#[derive(Debug)]
pub struct ShellCommand {
    pub command: String,
    pub args: Vec<String>,
    pub requires_sudo: bool,
}

impl ShellCommand {
    pub fn new(command: &str) -> Self {
        Self {
            command: command.to_owned(),
            args: vec![],
            requires_sudo: false,
        }
    }

    pub fn with_args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.args = args.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_sudo(mut self, enable: bool) -> Self {
        self.requires_sudo = enable;
        self
    }

    pub fn as_string(&self) -> String {
        let mut parts = Vec::new();
        if self.requires_sudo && !cfg!(windows) {
            parts.push("sudo".to_owned());
        }
        parts.push(self.command.clone());
        parts.extend(self.args.clone());
        parts.join(" ")
    }

    pub fn dry_run(&self) {
        println!("💡 [Dry Run] {}", self.as_string().cyan());
    }

    fn build_command(&self) -> Command {
        if cfg!(windows) {
            let full_cmd = format!("{} {}", self.command, self.args.join(" "));
            let mut cmd = Command::new("cmd");
            cmd.args(["/C", &full_cmd]);
            cmd
        } else {
            let mut cmd = if self.requires_sudo {
                let mut c = Command::new("sudo");
                c.arg(&self.command);
                c
            } else {
                Command::new(&self.command)
            };
            cmd.args(&self.args);
            cmd
        }
    }

    pub fn execute(&self) -> std::io::Result<Output> {
        self.build_command().output()
    }

    pub fn execute_with_dry_run(&self, dry_run: bool) -> Option<std::io::Result<Output>> {
        if dry_run {
            self.dry_run();
            None
        } else {
            Some(self.execute())
        }
    }

    pub fn run_verbose(&self, dry_run: bool) {
        println!("🚀 Running: {}", self.as_string().cyan());

        if let Some(result) = self.execute_with_dry_run(dry_run) {
            match result {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);

                    if output.status.success() {
                        if !stdout.trim().is_empty() {
                            println!("{}", stdout.green());
                        }
                        if !stderr.trim().is_empty() {
                            print_error!("{}", stderr.yellow());
                        }
                    } else {
                        print_error!("{}", stderr.red());
                    }
                }
                Err(e) => {
                    print_error!("❌ Failed to execute: {}", e);
                }
            }
        }
    }

    // pub fn run(&self, dry_run: bool) -> Option<std::io::Result<()>> {
    //     if dry_run {
    //         self.dry_run();
    //         return Some(Ok(()));
    //     }

    //     match self.execute() {
    //         Ok(output) => {
    //             if output.status.success() {
    //                 Some(Ok(()))
    //             } else {
    //                 Some(Err(std::io::Error::new(
    //                     std::io::ErrorKind::Other,
    //                     "Command failed",
    //                 )))
    //             }
    //         }
    //         Err(e) => Some(Err(e)),
    //     }
    // }

    pub fn run_interactive(&self, dry_run: bool) -> std::io::Result<()> {
        if dry_run {
            self.dry_run();
            return Ok(());
        }

        let mut child = self
            .build_command()
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        child.wait()?;
        Ok(())
    }

    // pub fn open_url(url: &str) {
    //     let result = if cfg!(windows) {
    //         Command::new("cmd").args(["/C", "start", "", url]).spawn()
    //     } else if cfg!(target_os = "macos") {
    //         Command::new("open").arg(url).spawn()
    //     } else {
    //         Command::new("xdg-open").arg(url).spawn()
    //     };

    //     if let Err(err) = result {
    //         print_error!("❌ {}: {}", "Failed to open URL".red(), err);
    //     }
    // }

    pub fn from_script(script: &str, sudo: bool) -> Self {
        if cfg!(windows) {
            Self::new("powershell")
                .with_args(["-Command", script])
                .with_sudo(sudo)
        } else {
            Self::new("sh")
                .with_args(["-c", script])
                .with_sudo(sudo)
        }
    }

    pub fn from_remote_script(remote: &str, script: &str, sudo: bool) -> Self {
        let wrapped = format!("ssh {} '{}'", remote, script);
        Self::from_script(&wrapped, sudo)
    }
}

pub fn command_exists(cmd: &str) -> bool {
    Command::new(if cfg!(windows) { "where" } else { "which" })
        .arg(cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn check_command(cmd: &str, friendly_name: &str) -> bool {
    let ok = command_exists(cmd);

    if ok {
        print_info!("✅ {} is installed.", friendly_name.green());
    } else {
        print_warn!("❌ {} is not installed.", friendly_name.red());
    }

    ok
}

pub fn run_shell_command(command: &str) {
    println!("🚀 Running: {}", command.cyan());

    let status = if cfg!(windows) {
        Command::new("powershell").args(["-Command", command]).status()
    } else {
        Command::new("sh").args(["-c", command]).status()
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
        }
        Err(e) => {
            print_error!("❌ {}: {}", "Failed to execute command".red(), e);
        }
    }
}

pub fn execute_package_cmd(cmd: &str, args: &[&str], sudo: bool, dry_run: bool) {
    ShellCommand::new(cmd)
        .with_args(args.iter().copied())
        .with_sudo(sudo)
        .run_verbose(dry_run);
}
