// Module: command/vps/Update
// Location: cli/src/command/vps/update.rs
use clap::Args;
use tranquility::{log_error, log_warn, print_info, print_success, print_warn};

use std::{fs, io};

use crate::{
    config::TranquilityConfig,
    model::vps::{VPSConfig, VPSList},
};

#[derive(Args, Debug)]
pub struct VpsUpdateCommand {
    #[arg(long)]
    id: Option<String>, // Required to identify which VPS to update
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

pub fn vps_command_update(cmd: VpsUpdateCommand, dry_run: bool) {
    if let Err(e) = update_vps_entry(cmd, dry_run) {
        log_error!("update", "vps", &format!("❌ Failed to update VPS entry: {e}"));
    }
}

fn update_vps_entry(cmd: VpsUpdateCommand, dry_run: bool) -> io::Result<()> {
    let config = TranquilityConfig::load_or_init()?;
    let path = &config.vps_file;

    if cmd.id.is_none() {
        print_warn!("❌ --id is required to update a specific VPS entry.");
        return Ok(());
    }

    let id = cmd.id.unwrap();

    if !path.exists() {
        log_warn!("update", "vps", &format!("⚠️ VPS config file not found at {}", path.display()));
        return Ok(());
    }

    let mut list = VPSList::load_from_file(path)?;

    let index = match list.vps.iter().position(|v| v.id.as_deref() == Some(&id)) {
        Some(i) => i,
        None => {
            print_warn!("❌ No VPS entry found with ID: {}", id);
            return Ok(());
        }
    };

    let entry = &mut list.vps[index];

    // Show the original entry
    print_info!("✏️ Updating VPS entry: {}@{}", entry.user.as_deref().unwrap_or("user"), entry.host);

    if let Some(name) = cmd.name {
        entry.name = Some(name);
    }
    if let Some(host) = cmd.host {
        entry.host = host;
    }
    if let Some(user) = cmd.user {
        entry.user = Some(user);
    }
    if let Some(port) = cmd.port {
        entry.port = Some(port);
    }
    if let Some(pk) = cmd.private_key {
        entry.private_key = Some(pk.into());
    }

    // Show the result
    println!("\n🔄 Updated entry:");
    println!("{}", serde_json::to_string_pretty(&entry).unwrap());

    if dry_run {
        print_warn!("(dry run) Changes not written to file.");
        return Ok(());
    }

    list.save_to_file(path)?;
    print_success!("✅ VPS entry updated in {}", path.display());
    Ok(())
}
