// Module: Command/Vps/Script
// Location: cli/src/command/vps/script.rs
use clap::Args;
use std::{fs, io, path::Path};

use crate::{
    config::TranquilityConfig,
    model::vps::{load_vps_entries},
    shell::script_runner::{ScriptSource, ShellScriptRunner},
};
use tranquility::{log_error, print_error, print_warn};
use shellexpand::tilde;

#[derive(Args, Debug)]
pub struct VpsScriptCommand {
    #[arg(long)]
    id: Option<String>,
    #[arg(long)]
    inline: Option<String>,
    #[arg(long)]
    file: Option<String>,
}

pub fn vps_command_script(cmd: VpsScriptCommand, dry_run: bool) {
    if let Err(e) = run_vps_script(cmd, dry_run) {
        log_error!("script", "vps", &format!("❌ Failed to run VPS script: {e}"));
    }
}

fn run_vps_script(cmd: VpsScriptCommand, dry_run: bool) -> io::Result<()> {
    let id = match cmd.id {
        Some(id) => id,
        None => {
            print_warn!("❌ --id is required to identify which VPS to target.");
            return Ok(());
        }
    };

    let config = TranquilityConfig::load_or_init()?;
    let vps_entries = load_vps_entries(&config.vps_file)?;

    let vps = match vps_entries.into_iter().find(|v| v.id.as_deref() == Some(&id)) {
        Some(v) => v,
        None => {
            print_warn!("❌ No VPS found with ID: {}", id);
            return Ok(());
        }
    };

    let script = match resolve_script(cmd.inline, cmd.file) {
        Ok(Some(script)) => script,
        Ok(None) => {
            print_warn!("⚠️ No script provided via --inline or --file.");
            return Ok(());
        }
        Err(e) => {
            print_error!("❌ Failed to resolve script: {e}");
            return Ok(());
        }
    };

    let remote = match &vps.user {
        Some(user) => format!("{}@{}", user, vps.host),
        None => vps.host.clone(),
    };

    let runner = ShellScriptRunner {
        script: match &script {
            ScriptSource::Inline(content) => content.clone(),
            ScriptSource::File(path) => std::fs::read_to_string(path)?,
        },
        source: script,
        remote: Some(remote),
        use_sudo: false,
        dry_run,
    };

    runner.run_verbose();
    Ok(())
}

fn resolve_script(inline: Option<String>, file: Option<String>) -> io::Result<Option<ScriptSource>> {
    if let Some(script) = inline {
        if !script.trim().is_empty() {
            return Ok(Some(ScriptSource::Inline(script)));
        }
    }

    if let Some(path) = file {
        let expanded = shellexpand::env(tilde(&path).as_ref())
            .unwrap_or_else(|_| tilde(&path).into())
            .to_string();

        if !Path::new(&expanded).exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("Script file not found: {}", expanded)));
        }

        return Ok(Some(ScriptSource::File(expanded)));
    }

    Ok(None)
}

