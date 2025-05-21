// src/vps.rs
use std::{fs, io, path::PathBuf};

use crate::{
    common::run_shell_command, config::TranquilityConfig, models::VPSConfig, print_error,
    print_info, print_success, print_warn,
};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use shellexpand::tilde;
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
    let vps_entries = match load_vps_entries(&config.vps_file) {
        Ok(entries) => entries,
        Err(e) => {
            print_error!(
                "❌ Failed to load VPS entries from {}: {}",
                config.vps_file.display(),
                e
            );
            return Ok(());
        }
    };
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
            set_connetion_string(vps)
        })
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🔌 Select a VPS to connect to")
        .items(&items)
        .default(0)
        .interact()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Dialog error: {e}")))?;

    let selected = &vps_entries[selection];
    println!(
        "\n🔗 Connecting to {}@{}...\n",
        selected.username.as_deref().unwrap_or("user"),
        selected.host
    );

    connect(selected)?;
    Ok(())
}

fn set_connetion_string(vps: &VPSConfig) -> String {

    let name = vps.name.clone();

    let mut conn = format!(
        "{}@{}",
        vps.username.as_deref().unwrap_or("user"),
        vps.host
    );

    if let Some(port) = vps.port.as_deref() {
        if port != "22" {
            conn = format!("{}:{}", conn, port);
        }
    }

    if name.is_some() {
        conn = format!("{} ({})", name.unwrap(), conn);
    }

    conn
}

fn connect(vps: &VPSConfig) -> io::Result<()> {
    let username = vps.username.as_deref().unwrap_or("user");
    let port = vps.port.as_deref().unwrap_or("22");

    let mut command_parts = vec!["ssh".to_string()];

    if let Some(private_key) = &vps.private_key {
        command_parts.push("-i".into());
        command_parts.push(private_key.display().to_string());
    }

    // Only add `-p` if port is not the default
    if port != "22" {
        command_parts.push("-p".into());
        command_parts.push(port.to_string());
    }

    command_parts.push(format!("{}@{}", username, vps.host));

    let command = command_parts.join(" ");

    print_info!("▶ Executing: {command}");
    run_shell_command(&command);

    Ok(())
}


pub fn json_schema_example() -> VPSConfig {
    VPSConfig {
        name: Some("Example VPS".into()),
        username: Some("ubuntu".into()),
        host: "example.com".into(),
        port: Some("22".into()),
        private_key: Some("/home/user/.ssh/id_rsa".into()),
    }
}

pub fn prompt_and_add_vps(
    name: Option<String>,
    host: Option<String>,
    username: Option<String>,
    port: Option<String>,
    private_key: Option<String>,
) -> io::Result<()> {
    let config = TranquilityConfig::load_or_init()?;
    let path = &config.vps_file;
    let is_full_interactive = host.is_none() || host.is_none(); 
    let mut vps_entries = load_vps_entries(path).unwrap_or_default();

    let name = match name {
        Some(v) => v,
        None => Input::new()
            .with_prompt("Name")
            .interact_text()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Dialog error: {e}")))?,
    };

    let host = match host {
        Some(v) => v,
        None => Input::new()
            .with_prompt("Host")
            .interact_text()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Dialog error: {e}")))?,
    };

    let username = match username {
        Some(v) => v,
        None => Input::new()
            .with_prompt("Username")
            .default("root".into())
            .interact_text()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Dialog error: {e}")))?,
    };

    let port = match port {
        Some(v) if !v.trim().is_empty() => v,
        Some(_) => "22".to_string(),
        None => {
            if is_full_interactive {
                // Only prompt if name wasn't passed (full interactive mode)
                Input::new()
                    .with_prompt("Port")
                    .default("22".into())
                    .interact_text()
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Dialog error: {e}")))?
            } else {
                "22".to_string()
            }
        }
    };

    let private_key = match private_key {
        Some(v) if !v.trim().is_empty() => Some(PathBuf::from(tilde(&v).to_string())),
        Some(_) => None,
        None => {
            if is_full_interactive {
                let input: String = Input::new()
                    .with_prompt("Private key path (leave blank for none)")
                    .allow_empty(true)
                    .default("".into())
                    .interact_text()
                    .map_err(|e| {
                        io::Error::new(io::ErrorKind::Other, format!("Dialog error: {e}"))
                    })?;

                if input.trim().is_empty() {
                    None
                } else {
                    Some(PathBuf::from(tilde(&input).to_string()))
                }
            } else {
                None
            }
        }
    };

    let new_vps = VPSConfig {
        name: Some(name),
        host,
        username: Some(username),
        port: Some(port),
        private_key,
    };

    print_info!("\n📦 New VPS entry:");
    println!("{}", serde_json::to_string_pretty(&new_vps).unwrap());

    vps_entries.push(new_vps);
    let json = serde_json::to_string_pretty(&vps_entries)?;
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, json)?;

    print_success!("✅ VPS entry saved to {}", path.display());
    Ok(())
}

pub fn confirm_and_delete_vps_config() -> io::Result<()> {
    let config = TranquilityConfig::load_or_init()?;
    let vps_path = config.vps_file;

    if !vps_path.exists() {
        println!("⚠️  VPS config does not exist at {}", vps_path.display());
        return Ok(());
    }

    let confirm = Confirm::new()
        .with_prompt(format!(
            "Are you sure you want to permanently delete {}?",
            vps_path.display()
        ))
        .default(false)
        .interact()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Dialog error: {e}")))?;

    if confirm {
        fs::remove_file(&vps_path)?;
        println!("🗑️  Deleted VPS config: {}", vps_path.display());
    } else {
        println!("❌ Deletion canceled.");
    }

    Ok(())
}