// Module: Shell/ScriptRunner
// Location: cli/src/shell/script_runner.rs
use crate::shell::ShellCommand;
use colored::Colorize;
use std::fs;

pub enum ScriptSource {
    Inline(String),
    File(String),
}

pub struct ShellScriptRunner {
    pub script: String,
    pub source: ScriptSource,
    pub remote: Option<String>, // e.g. "user@host"
    pub use_sudo: bool,
    pub dry_run: bool,
}

impl ShellScriptRunner {
    /// Create a runner from inline script content
    pub fn from_inline(script: &str, use_sudo: bool, dry_run: bool) -> Self {
        Self {
            script: script.to_string(),
            source: ScriptSource::Inline(script.to_string()),
            remote: None,
            use_sudo,
            dry_run,
        }
    }

    /// Create a runner from a file path (contents are loaded here)
    pub fn from_file(path: &str, use_sudo: bool, dry_run: bool) -> std::io::Result<Self> {
        let contents = fs::read_to_string(path)?;
        Ok(Self {
            script: contents.clone(),
            source: ScriptSource::File(path.to_string()),
            remote: None,
            use_sudo,
            dry_run,
        })
    }

    /// Set a remote SSH target (e.g. user@host) to run script remotely
    pub fn with_remote(mut self, remote: &str) -> Self {
        self.remote = Some(remote.to_string());
        self
    }

    /// Run the script, verbose output
    pub fn run_verbose(&self) {
        let label = match &self.source {
            ScriptSource::Inline(_) => "[inline]",
            ScriptSource::File(path) => path,
        };

        let remote_label = self
            .remote
            .as_ref()
            .map(|r| format!(" over SSH ({})", r))
            .unwrap_or_default();

        println!(
            "📜 Running script {}{}{}",
            label.cyan(),
            if self.dry_run { " (dry-run)" } else { "" },
            remote_label
        );

        let command = if let Some(remote_target) = &self.remote {
            let wrapped = format!("ssh {} '{}'", remote_target, self.script);
            ShellCommand::from_script(&wrapped, self.use_sudo)
        } else {
            ShellCommand::from_script(&self.script, self.use_sudo)
        };

        command.run_verbose(self.dry_run);
    }

    /// Run the script quietly
    pub fn run_silent(&self) -> Option<std::io::Result<()>> {
        let command = if let Some(remote_target) = &self.remote {
            let wrapped = format!("ssh {} '{}'", remote_target, self.script);
            ShellCommand::from_script(&wrapped, self.use_sudo)
        } else {
            ShellCommand::from_script(&self.script, self.use_sudo)
        };

        command.run(self.dry_run)
    }

    /// Default entry point (verbose by default)
    pub fn run(&self) {
        self.run_verbose();
    }
}
