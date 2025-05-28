// Module: command/vps/Delete
// Location: cli/src/command/vps/delete.rs
use clap::Args;
use tranquility::{log_error, log_warn, print_success, print_warn};
use dialoguer::{Confirm, Select, theme::ColorfulTheme};

use std::{fs, io};

use crate::{
    config::TranquilityConfig,
    model::vps::{VPSList},
};

#[derive(Args, Debug)]
pub struct VpsDeleteCommand {
    #[arg(long)]
    id: Option<String>,
}

pub fn vps_command_delete(cmd: VpsDeleteCommand, dry_run: bool) {
    if let Err(e) = confirm_and_delete_vps_entry(cmd, dry_run) {
        log_error!("delete", "vps", &format!("❌ Failed to delete VPS entry: {e}"));
    }
}

fn confirm_and_delete_vps_entry(cmd: VpsDeleteCommand, dry_run: bool) -> io::Result<()> {
    let config = TranquilityConfig::load_or_init()?;
    let path = &config.vps_file;

    if !path.exists() {
        log_warn!("delete", "vps", &format!("⚠️ VPS config does not exist at {}", path.display()));
        return Ok(());
    }

    let mut list = VPSList::load_from_file(path)?;

    if list.vps.is_empty() {
        print_warn!("⚠️ No VPS entries to delete.");
        return Ok(());
    }

    // Delete by ID with no prompt
    if let Some(ref id) = cmd.id {
        if let Some(index) = list.vps.iter().position(|v| v.id.as_deref() == Some(id)) {
            let entry = &list.vps[index];

            print_warn!(
                "🗑️ Deleting entry with ID '{}': {}@{}",
                id,
                entry.user.as_deref().unwrap_or("user"),
                entry.host
            );

            if dry_run {
                print_warn!("(dry run) VPS entry would have been deleted.");
                return Ok(());
            }

            list.vps.remove(index);
            list.save_to_file(path)?;
            print_success!("✅ VPS entry with ID '{}' deleted.", id);
            return Ok(());
        } else {
            print_warn!("❌ No VPS entry found with ID: {}", id);
            return Ok(());
        }
    }

    // No --id, prompt to select entry
    let items: Vec<String> = list
        .vps
        .iter()
        .map(|vps| {
            let name = vps.name.clone().unwrap_or_else(|| "-".into());
            let user = vps.user.clone().unwrap_or_else(|| "user".into());
            format!("{}@{} ({})", user, vps.host, name)
        })
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🗑️ Select a VPS to delete")
        .items(&items)
        .default(0)
        .interact()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Prompt failed: {e}")))?;

    let entry = &list.vps[selection];

    print_warn!(
        "❗ Entry to delete: {}@{}",
        entry.user.as_deref().unwrap_or("user"),
        entry.host
    );

    let confirm = Confirm::new()
        .with_prompt("Are you sure you want to delete this VPS entry?")
        .default(false)
        .interact()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Prompt failed: {e}")))?;

    if !confirm {
        print_warn!("❌ Deletion canceled.");
        return Ok(());
    }

    if dry_run {
        print_warn!("(dry run) VPS entry would have been deleted.");
        return Ok(());
    }

    list.vps.remove(selection);
    list.save_to_file(path)?;

    print_success!("🗑️ VPS entry deleted from {}", path.display());
    Ok(())
}


pub fn confirm_and_delete_vps_config(dry_run: bool) -> io::Result<()> {
    let config = TranquilityConfig::load_or_init()?;
    let vps_path = config.vps_file;

    if !vps_path.exists() {
        log_warn!(
            "delete",
            "vps",
            &format!("⚠️ VPS config does not exist at {}", vps_path.display())
        );
        return Ok(());
    }

    let confirm = Confirm::new()
        .with_prompt(format!(
            "Are you sure you want to permanently delete {}?",
            vps_path.display()
        ))
        .default(false)
        .interact()
        .unwrap_or(false);

    if !confirm {
        print_warn!("❌ Deletion canceled.");
        return Ok(());
    }

    if dry_run {
        print_warn!("(dry run) VPS config would have been deleted: {}", vps_path.display());
        return Ok(());
    }

    fs::remove_file(&vps_path)?;
    print_success!("🗑️ Deleted VPS config: {}", vps_path.display());

    Ok(())
}
