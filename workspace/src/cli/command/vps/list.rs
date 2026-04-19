// Module: Command/Vps/List
// Location: cli/src/command/vps/list.rs

use clap::Args;
use std::io;
use tabled::settings::Style;
use tabled::{Table, Tabled};

use crate::{
    config::TranquilityConfig,
    models::vps::json::{VpsConfig, VpsEntry},
    print_info, print_warn,
};

#[derive(Args, Debug)]
pub struct VpsListCommand {
    #[arg(long)]
    user: Option<String>,
    #[arg(long)]
    host: Option<String>,
    #[arg(long)]
    dry_run: bool,
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

pub fn vps_command_list(cmd: VpsListCommand, dry_run: bool) {
    if let Err(e) = vps_list(cmd.user, cmd.host, dry_run) {
        print_warn!("❌ Failed to list VPS entries: {e}");
    }
}

pub fn vps_list(user: Option<String>, host: Option<String>, dry_run: bool) -> io::Result<()> {
    let mut vps_entries: Vec<VpsEntry>;

    if dry_run {
        print_warn!("[dry run] Simulating VPS list output.");

        if let Some(ref u) = user {
            print_info!("Would filter by user: {u}");
        }

        if let Some(ref h) = host {
            print_info!("Would filter by host: {h}");
        }

        vps_entries = vec![VpsEntry {
            id: Some("dry-run-id".into()),
            name: Some("DryRunVPS".into()),
            user: Some("dryuser".into()),
            host: "dry.run.host".into(),
            port: Some("2222".into()),
            private_key: None,
            post_connect_script: None,
        }];
    } else {
        let config = TranquilityConfig::load_once();
        let vps_config = VpsConfig::load_from_file(&config.vps_file)?;
        vps_entries = vps_config.vps;
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

fn list_vps_entries(vps_entries: &[VpsEntry]) {
    let table_data: Vec<VPSDisplay> = vps_entries
        .iter()
        .map(|vps| VPSDisplay {
            id: vps.effective_id().to_string(),
            name: vps.name.clone().unwrap_or_else(|| "-".into()),
            username: vps.effective_user(),
            host: vps.host.clone(),
            port: vps.effective_port().to_string(),
        })
        .collect();

    let mut table = Table::new(table_data);
    table.with(Style::modern());
    println!("{}", table);
}
