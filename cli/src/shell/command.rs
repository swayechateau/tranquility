// src/shell/command.rs
use crate::{print_error, print_info, print_warn};
use colored::Colorize;
use std::process::{Command, Output, Stdio};

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

    pub fn execute(&self) -> std::io::Result<Output> {
        let mut cmd = if cfg!(target_os = "windows") {
            let full_cmd = format!("{} {}", self.command, self.args.join(" "));
            let mut c = Command::new("cmd");
            c.args(&["/C", &full_cmd]);
            c
        } else {
            let mut full_cmd = if self.requires_sudo {
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

    // pub fn run_status_only(&self, dry_run: bool) -> Option<bool> {
    //     if dry_run {
    //         self.dry_run();
    //         return Some(true);
    //     }

    //     self.execute().ok().map(|out| out.status.success())
    // }

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

    pub fn from_script(script: &str, sudo: bool) -> Self {
        if cfg!(windows) {
            Self {
                command: "powershell".to_string(),
                args: vec!["-Command".to_string(), script.to_string()],
                requires_sudo: sudo,
            }
        } else {
            Self {
                command: "sh".to_string(),
                args: vec!["-c".to_string(), script.to_string()],
                requires_sudo: sudo,
            }
        }
    }

    pub fn from_args(args: &[&str]) -> Vec<String> {
        args.iter().map(|s| s.to_string()).collect()
    }

    pub fn run_interactive(&self, dry_run: bool) -> std::io::Result<()> {
        if dry_run {
            self.dry_run();
            return Ok(());
        }

        let mut cmd;

        if cfg!(target_os = "windows") {
            let full_cmd = format!("{} {}", self.command, self.args.join(" "));
            cmd = Command::new("cmd");
            cmd.args(&["/C", &full_cmd]);
        } else {
            let mut full_cmd = if self.requires_sudo {
                vec!["sudo".to_string(), self.command.clone()]
            } else {
                vec![self.command.clone()]
            };
            full_cmd.extend(self.args.clone());

            cmd = Command::new(&full_cmd[0]);
            for arg in &full_cmd[1..] {
                cmd.arg(arg);
            }
        }

        cmd.stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        let mut child = cmd.spawn()?;
        child.wait()?;
        Ok(())
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

// pub fn open_url(url: &str) {
//     let result = if cfg!(target_os = "windows") {
//         Command::new("cmd").args(&["/C", "start", "", url]).spawn()
//     } else if cfg!(target_os = "macos") {
//         Command::new("open").arg(url).spawn()
//     } else {
//         Command::new("xdg-open").arg(url).spawn()
//     };

//     if let Err(err) = result {
//         print_error!("❌ {}: {}", "Failed to open URL".red(), err);
//     }
// }

pub fn run_shell_command(command: &str) {
    println!("🚀 Running: {}", command.cyan());

    let status = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(&["-Command", command])
            .status()
    } else {
        Command::new("sh").args(&["-c", command]).status()
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
        .with_args(ShellCommand::from_args(args))
        .with_sudo(sudo)
        .run_verbose(dry_run);
}

// pub fn check_sudo() -> bool {
//     if cfg!(target_os = "windows") {
//         Command::new("net")
//             .arg("session")
//             .status()
//             .map(|s| s.success())
//             .unwrap_or(false)
//     } else {
//         match Command::new("id").arg("-u").output() {
//             Ok(output) if output.status.success() => {
//                 let uid = String::from_utf8_lossy(&output.stdout);
//                 uid.trim() == "0"
//             }
//             _ => false,
//         }
//     }
// }
