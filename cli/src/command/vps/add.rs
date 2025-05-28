// Module: Command/Vps/Add
// Location: cli/src/command/vps/add.rs
use clap::Args;
use tranquility::{log_error, print_info, print_success};
use dialoguer::{Input, theme::ColorfulTheme};

use std::{fs, io, path::PathBuf};

use crate::{
    config::TranquilityConfig,
    model::vps::{VPSConfig, load_vps_entries, VPSList, generate_id},
};
use shellexpand::tilde;

#[derive(Args, Debug)]
pub struct VpsAddCommad {
    #[arg(long)]
    id: Option<String>,
    #[arg(long)]
    name: Option<String>,
    #[arg(long)]
    host: Option<String>,
    #[arg(long)]
    user: Option<String>,
    #[arg(long)]
    port: Option<String>,
    #[arg(long = "private-key")]
    private_key: Option<String>,
}

pub fn vps_command_add(cmd: VpsAddCommad, dry_run: bool) {
    if let Err(e) = prompt_and_add_vps(cmd, dry_run) {
        log_error!("add", "vps", &format!("❌ Failed to add VPS: {e}"));
    } else {
        println!("✅ VPS entry added successfully.");
    }
}

fn prompt_and_add_vps(cmd: VpsAddCommad, dry_run: bool) -> io::Result<()> {
    let config = TranquilityConfig::load_or_init()?;
    let path = &config.vps_file;
    let mut vps_entries = load_vps_entries(path).unwrap_or_else(|_| Vec::new());

    let is_full_interactive = cmd.host.is_none();

    let name = cmd.name.unwrap_or_else(|| Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Name")
        .interact_text()
        .unwrap());

    let host = cmd.host.unwrap_or_else(|| Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Host")
        .interact_text()
        .unwrap());

    let username = cmd.user.unwrap_or_else(|| Input::with_theme(&ColorfulTheme::default())
        .with_prompt("User")
        .default("root".into())
        .interact_text()
        .unwrap());

    let port = cmd.port.unwrap_or_else(|| {
        if is_full_interactive {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Port")
                .default("22".into())
                .interact_text()
                .unwrap()
        } else {
            "22".into()
        }
    });

    let private_key = match cmd.private_key {
        Some(v) if !v.trim().is_empty() => Some(PathBuf::from(tilde(&v).to_string())),
        _ if is_full_interactive => {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Private key path (leave blank for none)")
                .allow_empty(true)
                .default("".into())
                .interact_text()
                .unwrap();

            if input.trim().is_empty() {
                None
            } else {
                Some(PathBuf::from(tilde(&input).to_string()))
            }
        }
        _ => None,
    };

    let post_connect_script = if is_full_interactive {
        let input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Post-connect script (inline or @file)")
            .allow_empty(true)
            .default("".into())
            .interact_text()
            .unwrap();

        if input.trim().is_empty() {
            None
        } else if input.trim_start().starts_with('@') {
            let path = tilde(input.trim_start_matches('@').trim());
            match fs::read_to_string(path.to_string()) {
                Ok(content) => Some(content),
                Err(e) => {
                    log_error!("script", "vps", &format!("❌ Failed to read script file: {e}"));
                    None
                }
            }
        } else {
            Some(input)
        }
    } else {
        None
    };

    let new_vps = VPSConfig {
        id: cmd.id.or_else(|| Some(generate_id(Some(&name), &host, Some(&username)))),
        name: Some(name),
        host,
        user: Some(username),
        port: Some(port),
        private_key,
        post_connect_script,
    };

    print_info!("\n📦 New VPS entry:");
    println!("{}", serde_json::to_string_pretty(&new_vps).unwrap());

    if dry_run {
        print_info!("(dry run) VPS entry not saved.");
        return Ok(());
    }

    vps_entries.push(new_vps);
    let list = VPSList { vps: vps_entries };
    list.save_to_file(path)?;

    print_success!("✅ VPS entry saved to {}", path.display());
    Ok(())
}
