// Module: command/vps/List
// Location: cli/src/command/vps/list.rs
use clap::Args;
use std::io;
use tabled::{Table, Tabled};
use tranquility::{print_info, print_warn};

use crate::model::vps::{get_vps_entries, VPSConfig};

#[derive(Args, Debug)]
pub struct VpsListCommad {
    #[arg(long)]
    user: Option<String>,
    #[arg(long)]
    host: Option<String>,
}

#[derive(Tabled)]
struct VPSDisplay {
    #[tabled(rename = "Id")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "User")]
    username: String,
    #[tabled(rename = "Host")]
    host: String,
    #[tabled(rename = "Port")]
    port: String,
}

pub fn vps_command_list(cmd: VpsListCommad, dry_run: bool) {
    if let Err(e) = vps_list(cmd.user, cmd.host, dry_run) {
        print_warn!("❌ Failed to list VPS entries: {e}");
    }
}

pub fn vps_list(user: Option<String>, host: Option<String>, dry_run: bool) -> io::Result<()> {
    if dry_run {
        print_warn!("[dry run] Simulating VPS list output.");

        if let Some(ref u) = user {
            print_info!("Would filter by user: {u}");
        }

        if let Some(ref h) = host {
            print_info!("Would filter by host: {h}");
        }

        // Simulated entry (optional)
        let simulated = VPSConfig {
            id: Some("dry-run-id".into()),
            name: Some("DryRunVPS".into()),
            user: Some("dryuser".into()),
            host: "dry.run.host".into(),
            port: Some("2222".into()),
            private_key: None,
            post_connect_script: None,
        };

        let mut simulated_entries = vec![simulated];

        // Apply filters in dry-run mode too
        if let Some(u) = user {
            simulated_entries.retain(|v| v.user.as_deref().unwrap_or("user") == u);
        }

        if let Some(h) = host {
            simulated_entries.retain(|v| v.host == h);
        }

        if simulated_entries.is_empty() {
            print_warn!("⚠️  No simulated entries match the specified filters.");
            return Ok(());
        }

        list_vps_entries(&simulated_entries);
        return Ok(());
    }
    let mut vps_entries=get_vps_entries()?;

    if vps_entries.is_empty() {
        print_warn!("⚠️  No VPS entries found in your configuration.");
        return Ok(());
    }

    // Apply filters
    if let Some(user_filter) = user {
        vps_entries.retain(|v| v.user.as_deref().unwrap_or("user") == user_filter);
    }

    if let Some(host_filter) = host {
        vps_entries.retain(|v| v.host == host_filter);
    }

    if vps_entries.is_empty() {
        print_warn!("⚠️  No VPS entries match the specified filters.");
        return Ok(());
    }

    print_info!("\n📋 VPS instances:\n");
    list_vps_entries(&vps_entries);
    Ok(())
}

fn list_vps_entries(vps_entries: &[VPSConfig]) {
    let table_data: Vec<VPSDisplay> = vps_entries
        .iter()
        .enumerate()
        .map(|(i, vps)| VPSDisplay {
            id: vps.id.clone().unwrap_or_else(|| format!("{}", i + 1)),
            name: vps.name.clone().unwrap_or_else(|| "-".into()),
            username: vps.user.clone().unwrap_or_else(|| "user".into()),
            host: vps.host.clone(),
            port: vps.port.clone().unwrap_or_else(|| "22".into()),
        })
        .collect();

    println!("{}", Table::new(table_data));
}
