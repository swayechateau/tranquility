// Module: Command/Vps/Add
// Location: cli/src/command/vps/add.rs

use crate::{
    config::TranquilityConfig,
    log_error,
    models::vps::{
        generate_id,
        json::{FlexibleValue, VpsConfig, VpsEntry},
    },
    print_info, print_success, print_warn,
};
use clap::Args;
use dialoguer::{Input, theme::ColorfulTheme};
use shellexpand::tilde;
use std::{fs, io};

#[derive(Args, Debug)]
pub struct VpsAddCommand {
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

pub fn vps_command_add(cmd: VpsAddCommand, dry_run: bool) {
    if let Err(e) = prompt_and_add_vps(cmd, dry_run) {
        log_error!("add", "vps", &format!("❌ Failed to add VPS: {e}"));
    } else {
        println!("✅ VPS entry added successfully.");
    }
}
fn prompt_and_add_vps(cmd: VpsAddCommand, dry_run: bool) -> io::Result<()> {
    let config = TranquilityConfig::load_once();
    let path = &config.vps_file;
    let mut vps_config =
        VpsConfig::load_from_file(path).unwrap_or_else(|_| VpsConfig { vps: vec![] });

    let is_full_interactive = cmd.host.is_none();

    let name = cmd.name.unwrap_or_else(|| {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Name")
            .interact_text()
            .unwrap()
    });

    let host = cmd.host.unwrap_or_else(|| {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Host")
            .interact_text()
            .unwrap()
    });

    let username = cmd.user.or_else(|| {
        if is_full_interactive {
            Some(
                Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("User")
                    .default("root".into())
                    .interact_text()
                    .unwrap(),
            )
        } else {
            None
        }
    });

    let port = cmd
        .port
        .or_else(|| {
            if is_full_interactive {
                Some(
                    Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Port")
                        .allow_empty(true)
                        .default("".into())
                        .interact_text()
                        .unwrap(),
                )
            } else {
                None
            }
        })
        .and_then(|p| {
            let trimmed = p.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(FlexibleValue::from(trimmed.to_string()))
            }
        });

    let private_key = match cmd.private_key {
        Some(ref v) if !v.trim().is_empty() => Some(tilde(v).to_string()),
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
                Some(tilde(&input).to_string())
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
                    log_error!(
                        "script",
                        "vps",
                        &format!("❌ Failed to read script file: {e}")
                    );
                    None
                }
            }
        } else {
            Some(input)
        }
    } else {
        None
    };

    let mut id = cmd
        .id
        .or_else(|| Some(generate_id(Some(&name), &host, username.as_deref())));

    // Check for duplicate ID
    if let Some(ref id_str) = id
        && let Some(existing) = vps_config
            .vps
            .iter()
            .find(|v| v.id.as_deref() == Some(id_str))
    {
        print_warn!("⚠️ A VPS entry with ID '{}' already exists:", id_str);
        println!("{}", serde_json::to_string_pretty(&existing).unwrap());

        let options = &["Cancel", "Overwrite existing", "Use a different ID"];
        let choice = dialoguer::Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do?")
            .items(options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => {
                print_info!("❌ Aborted. VPS entry not added.");
                return Ok(());
            }
            1 => {
                // Remove old entry with same ID
                vps_config.vps.retain(|v| v.id.as_deref() != Some(id_str));
                print_info!("🔄 Overwriting existing entry with ID '{}'", id_str);
            }
            2 => {
                // Auto-suggest a new, available ID
                let base = id_str.clone();
                let mut count = 1;
                let mut suggested = format!("{}-{}", base, count);
                let existing_ids: std::collections::HashSet<_> = vps_config
                    .vps
                    .iter()
                    .filter_map(|v| v.id.as_deref())
                    .collect();

                while existing_ids.contains(suggested.as_str()) {
                    count += 1;
                    suggested = format!("{}-{}", base, count);
                }

                let new_id: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter a new ID")
                    .default(suggested)
                    .interact_text()
                    .unwrap();

                id = Some(new_id);
            }
            _ => unreachable!(),
        }
    }

    let new_vps = VpsEntry {
        id,
        name: Some(name),
        host,
        user: username,
        port,
        private_key,
        post_connect_script,
    };

    print_info!("\n📦 New VPS entry:");
    println!("{}", serde_json::to_string_pretty(&new_vps).unwrap());

    if dry_run {
        print_info!("(dry run) VPS entry not saved.");
        return Ok(());
    }

    vps_config.push(new_vps);
    vps_config.save_to_file(path)?;

    print_success!("✅ VPS entry saved to {}", path.display());
    Ok(())
}
