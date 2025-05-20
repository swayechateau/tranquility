use std::{fs, io, path::PathBuf};

use crate::{common::run_shell_command, config::TranquilityConfig, models::VPSConfig, print_info, print_warn};
use dialoguer::{theme::ColorfulTheme, Select};
use tabled::{Table, Tabled};

/// Internal struct to display VPS info in a table.
#[derive(Tabled)]
struct VPSDisplay {
    #[tabled(rename = "Index")]
    index: usize,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "User")]
    username: String,
    #[tabled(rename = "Host")]
    host: String,
    #[tabled(rename = "Port")]
    port: String,
}

pub fn load_vps_entries(path: &PathBuf) -> io::Result<Vec<VPSConfig>> {
    let content = fs::read_to_string(path)?;
    let entries: Vec<VPSConfig> = serde_json::from_str(&content)?;
    Ok(entries)
}

pub fn list_vps_entries(vps_entries: &[VPSConfig]) {
    let table_data: Vec<VPSDisplay> = vps_entries
        .iter()
        .enumerate()
        .map(|(i, vps)| VPSDisplay {
            index: i + 1,
            name: vps.name.clone().unwrap_or_else(|| "-".to_string()),
            username: vps.username.clone().unwrap_or_else(|| "user".to_string()),
            host: vps.host.clone(),
            port: vps.port.clone().unwrap_or_else(|| "22".to_string()),
        })
        .collect();

    let table = Table::new(table_data).to_string();
    println!("{table}");
}

pub fn connect_to_vps(list: bool) -> io::Result<()> {
    println!("Running VPS command");
    let config = TranquilityConfig::load_or_init()?;
    println!("Config loaded: {:?}", config.vps_file);
    let vps_entries = load_vps_entries(&config.vps_file)?;
    println!("Loaded {} VPS entries", vps_entries.len());
    
    if vps_entries.is_empty() {
        print_warn!("⚠️  No VPS entries found in your configuration.");
        return Ok(());
    }

    if list {
        println!("\n📋 Configured VPS instances:\n");
        list_vps_entries(&vps_entries);
        return Ok(());
    }

    let items: Vec<String> = vps_entries
        .iter()
        .map(|vps| {
            format!(
                "{}@{}:{}",
                vps.username.as_deref().unwrap_or("user"),
                vps.host,
                vps.port.as_deref().unwrap_or("22")
            )
        })
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🔌 Select a VPS to connect to")
        .items(&items)
        .default(0)
        .interact().map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Dialog error: {e}")))?;

    let selected = &vps_entries[selection];
    println!(
        "\n🔗 Connecting to {}@{}...\n",
        selected.username.as_deref().unwrap_or("user"),
        selected.host
    );

    connect(selected)?;
    Ok(())
}

fn connect(vps: &VPSConfig) -> io::Result<()> {
    let username = vps.username.as_deref().unwrap_or("user");
    let port = vps.port.as_deref().unwrap_or("22");
    let private_key = vps
        .private_key
        .as_ref()
        .map(|p| format!("-i {}", p.display()))
        .unwrap_or_default();

    let command = format!("ssh {} -p {} {}@{}", private_key, port, username, vps.host);
    print_info!("▶ Executing: {command}");

    run_shell_command(&command);
    Ok(())
}
