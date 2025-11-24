use clap::Args;
use dialoguer::{Confirm, Select, theme::ColorfulTheme};

use std::{fs, io};

use crate::{log_error, log_warn, print_success, print_warn,
    config::TranquilityConfig,
    models::vps::json::VpsConfig,
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
    let config = TranquilityConfig::load_once();
    let path = &config.vps_file;

    if !path.exists() {
        log_warn!("delete", "vps", &format!("⚠️ VPS config does not exist at {}", path.display()));
        return Ok(());
    }

    let mut vps_config = VpsConfig::load_from_file(path)?;

    if vps_config.vps.is_empty() {
        print_warn!("⚠️ No VPS entries to delete.");
        return Ok(());
    }

    // Direct deletion by --id
    if let Some(ref id) = cmd.id {
        if let Some(index) = vps_config.vps.iter().position(|v| v.id.as_deref() == Some(id)) {
            let entry = &vps_config.vps[index];
            print_warn!(
                "🗑️ Deleting entry with ID '{}': {}@{}",
                id,
                entry.effective_user(),
                entry.host
            );

            if dry_run {
                print_warn!("(dry run) VPS entry would have been deleted.");
                return Ok(());
            }

            vps_config.vps.remove(index);
            vps_config.save_to_file(path)?;
            print_success!("✅ VPS entry with ID '{}' deleted.", id);
        } else {
            print_warn!("❌ No VPS entry found with ID: {}", id);
        }
        return Ok(());
    }

    // Interactive deletion if --id is not provided
    let items: Vec<String> = vps_config
        .vps
        .iter()
        .map(|v| {
            let name = v.name.as_deref().unwrap_or("-");
            format!("{}@{} ({})", v.effective_user(), v.host, name)
        })
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🗑️ Select a VPS to delete")
        .items(&items)
        .default(0)
        .interact()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Prompt failed: {e}")))?;

    let entry = &vps_config.vps[selection];

    print_warn!(
        "❗ Entry to delete: {}@{} ({})",
        entry.effective_user(),
        entry.host,
        entry.name.as_deref().unwrap_or("-")
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

    vps_config.vps.remove(selection);
    vps_config.save_to_file(path)?;
    print_success!("🗑️ VPS entry deleted from {}", path.display());

    Ok(())
}

pub fn confirm_and_delete_vps_config(dry_run: bool) -> io::Result<()> {
    let config = TranquilityConfig::load_once();
    let vps_path = config.vps_file.clone();

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
