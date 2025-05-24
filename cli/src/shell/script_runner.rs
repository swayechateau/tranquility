// src/shell/script_runner.rs
// use crate::shell::ShellCommand;
// use colored::Colorize;
// use std::fs;

// pub struct ShellScriptRunner {
//     pub script: String,
//     pub from_file: bool,
//     pub use_sudo: bool,
//     pub dry_run: bool,
// }

// impl ShellScriptRunner {
//     /// Create a runner from inline script
//     pub fn from_inline(script: &str, use_sudo: bool, dry_run: bool) -> Self {
//         Self {
//             script: script.to_string(),
//             from_file: false,
//             use_sudo,
//             dry_run,
//         }
//     }

//     /// Create a runner from a script file (.sh or .ps1)
//     pub fn from_file(path: &str, use_sudo: bool, dry_run: bool) -> std::io::Result<Self> {
//         let contents = fs::read_to_string(path)?;
//         Ok(Self {
//             script: contents,
//             from_file: true,
//             use_sudo,
//             dry_run,
//         })
//     }

//     /// Run the script with shell and show output
//     pub fn run_verbose(&self) {
//         println!(
//             "📜 Running script {}{}",
//             if self.from_file { "[file] " } else { "" },
//             if self.dry_run { "(dry-run)".yellow() } else { "".normal() }
//         );

//         let cmd = ShellCommand::from_script(&self.script, self.use_sudo);
//         cmd.run_verbose(self.dry_run);
//     }

//     /// Run silently (e.g., for internal use or scripting)
//     pub fn run_silent(&self) -> Option<std::io::Result<()>> {
//         ShellCommand::from_script(&self.script, self.use_sudo).run(self.dry_run)
//     }

//     /// Unified run (default to verbose)
//     pub fn run(&self) {
//         self.run_verbose();
//     }
// }
