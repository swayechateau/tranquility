// Module: Command/Vps/Script
// Location: cli/src/command/vps/script.rs

use std::{fs, io, path::Path};
use dialoguer::{Select, theme::ColorfulTheme};
use shellexpand::tilde;

use crate::{core::{config::TranquilityConfig, model::vps::json::VpsConfig, utils::shell::script_runner::{ScriptSource, ShellScriptRunner}}, log_error, print_warn};

pub fn run(
    id: Option<String>,
    inline: Option<String>,
    file: Option<String>,
    dry_run: bool
) -> anyhow::Result<()> {
    if let Err(e) = run_vps_script(id, inline, file, dry_run) {
        log_error!("script", "vps", &format!("❌ Failed to run VPS script: {e}"));
        anyhow::bail!(format!("❌ Failed to run VPS script: {e}"));
    }
    Ok(())
}

fn run_vps_script(id: Option<String>, inline: Option<String>, file: Option<String>, dry_run: bool) -> io::Result<()> {
    let config = TranquilityConfig::load_once();
    let mut vps_config = VpsConfig::load_from_file(&config.vps_file)?;
    vps_config.fix();

    // STEP 1: Resolve VPS entry
    let vps = match &id {
        Some(id) => vps_config.vps.iter().find(|v| v.id.as_deref() == Some(id)),
        None => {
            if vps_config.vps.is_empty() {
                print_warn!("⚠️ No VPS entries found in your configuration.");
                return Ok(());
            }

            let options: Vec<String> = vps_config.vps.iter().map(|v| {
                let user = v.effective_user();
                let name = v.name.clone().unwrap_or_else(|| "-".into());
                format!("{}@{} ({})", user, v.host, name)
            }).collect();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("💻 Select a VPS to run the script on")
                .items(&options)
                .default(0)
                .interact()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Prompt failed: {e}")))?;

            Some(&vps_config.vps[selection])
        }
    };

    let vps = match vps {
        Some(v) => v,
        None => {
            print_warn!("❌ No VPS found with the specified ID.");
            return Ok(());
        }
    };

    // STEP 2: Resolve script source: inline > file > post_connect_script
    let script = match resolve_script(inline, file)? {
        Some(script) => script,
        None => {
            if let Some(script_str) = &vps.post_connect_script {
                ScriptSource::Inline(script_str.clone())
            } else {
                print_warn!("⚠️ No script provided via --inline, --file, or VPS config. Aborting.");
                return Ok(());
            }
        }
    };

    // STEP 3: Prepare and run script
    let remote = format!("{}@{}", vps.effective_user(), vps.host);

    let script_content = match &script {
        ScriptSource::Inline(content) => content.clone(),
        ScriptSource::File(path) => fs::read_to_string(path)?,
    };

    let runner = ShellScriptRunner {
        script: script_content,
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
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Script file not found: {}", expanded),
            ));
        }

        return Ok(Some(ScriptSource::File(expanded)));
    }

    Ok(None)
}
