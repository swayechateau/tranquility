// Module: Shell/ScriptRunner
// Location: cli/src/shell/script_runner.rs
use crate::core::shell::ShellCommand;
use colored::Colorize;
// use std::fs;

pub enum ScriptSource {
    Inline(String),
    File(String),
}

pub struct ShellScriptRunner {
    pub script: String,
    pub source: ScriptSource,
    pub remote: Option<String>,
    pub use_sudo: bool,
    pub dry_run: bool,
}

impl ShellScriptRunner {
    // pub fn from_inline(script: &str, use_sudo: bool, dry_run: bool) -> Self {
    //     Self {
    //         script: script.to_string(),
    //         source: ScriptSource::Inline(script.to_string()),
    //         remote: None,
    //         use_sudo,
    //         dry_run,
    //     }
    // }

    // pub fn from_file(path: &str, use_sudo: bool, dry_run: bool) -> std::io::Result<Self> {
    //     let contents = fs::read_to_string(path)?;
    //     Ok(Self {
    //         script: contents,
    //         source: ScriptSource::File(path.to_string()),
    //         remote: None,
    //         use_sudo,
    //         dry_run,
    //     })
    // }

    // pub fn with_remote(mut self, remote: &str) -> Self {
    //     self.remote = Some(remote.to_string());
    //     self
    // }

    pub fn run_verbose(&self) {
        let label = match &self.source {
            ScriptSource::Inline(_) => "[inline]",
            ScriptSource::File(path) => path,
        };

        let remote_label = self.remote
            .as_ref()
            .map(|r| format!(" over SSH ({})", r))
            .unwrap_or_default();

        println!(
            "📜 Running script {}{}{}",
            label.cyan(),
            if self.dry_run { " (dry-run)" } else { "" },
            remote_label
        );

        self.to_command().run_verbose(self.dry_run);
    }

    // pub fn run_silent(&self) -> Option<std::io::Result<()>> {
    //     self.to_command().run(self.dry_run)
    // }

    // pub fn run(&self) {
    //     self.run_verbose();
    // }

    fn to_command(&self) -> ShellCommand {
        match &self.remote {
            Some(remote) => ShellCommand::from_remote_script(remote, &self.script, self.use_sudo),
            None => ShellCommand::from_script(&self.script, self.use_sudo),
        }
    }
}