//! VPS management capabilities for Tranquility.
//!
//! This module provides operations for managing VPS servers (add, connect, update, delete, run scripts).

use crate::engine::Result;
use crate::engine::models::vps::VpsEntry;
use std::fs;
use std::path::PathBuf;

/// A VPS config file can be `{ vps: [...] }` wrapper or a bare array.
#[derive(serde::Deserialize)]
struct VpsConfigWrapper {
    vps: Vec<VpsEntry>,
}

/// Load VPS configuration from file, handling both wrapped `{ vps: [...] }` and bare `[...]` formats.
fn load_vps_config(vps_file_path: &PathBuf) -> Result<Vec<VpsEntry>> {
    if !vps_file_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(vps_file_path).map_err(|e| {
        crate::engine::error::ErrorCode::IoFailure
            .error()
            .with_context("path", vps_file_path.display().to_string())
            .with_context("error", e.to_string())
    })?;

    let ext = vps_file_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "yaml" | "yml" => {
            // Try wrapped format first, then bare array
            if let Ok(wrapper) = serde_yaml::from_str::<VpsConfigWrapper>(&content) {
                Ok(wrapper.vps)
            } else {
                serde_yaml::from_str::<Vec<VpsEntry>>(&content).map_err(|e| {
                    crate::engine::error::ErrorCode::ConfigInvalid
                        .error()
                        .with_context("format", "yaml")
                        .with_context("error", e.to_string())
                })
            }
        }
        _ => {
            // JSON: try bare array first, then wrapped
            if let Ok(entries) = serde_json::from_str::<Vec<VpsEntry>>(&content) {
                Ok(entries)
            } else if let Ok(wrapper) = serde_json::from_str::<VpsConfigWrapper>(&content) {
                Ok(wrapper.vps)
            } else {
                Err(crate::engine::error::ErrorCode::ConfigInvalid
                    .error()
                    .with_context("format", "json")
                    .with_context("hint", "expected \"{ vps: [...] }\" or \"[...]\" format"))
            }
        }
    }
}

/// Row data for VPS listing — returned to callers for format-aware rendering.
pub struct VpsListRow {
    pub id: String,
    pub name: String,
    pub user: String,
    pub host: String,
    pub port: String,
}

/// Return rows for VPS listing with optional user/host filters.
pub fn vps_list_rows(
    vps_file_path: &PathBuf,
    user_filter: Option<&str>,
    host_filter: Option<&str>,
) -> crate::engine::Result<Vec<VpsListRow>> {
    let mut entries = load_vps_config(vps_file_path)?;

    if let Some(user) = user_filter {
        entries.retain(|v| v.user.as_deref().unwrap_or("") == user);
    }
    if let Some(host) = host_filter {
        entries.retain(|v| v.host == host);
    }

    Ok(entries
        .into_iter()
        .map(|entry| VpsListRow {
            id: entry.effective_id().to_string(),
            name: entry.name.as_deref().unwrap_or("-").to_string(),
            user: entry.effective_user(),
            port: entry.effective_port().to_string(),
            host: entry.host,
        })
        .collect())
}

/// Add a new VPS entry to the VPS configuration file (interactive).
pub async fn add(
    name: Option<String>,
    host: Option<String>,
    user: Option<String>,
    port: Option<u16>,
    _private_key: Option<PathBuf>,
    _post_connect_script: Option<String>,
    vps_file_path: &PathBuf,
) -> crate::engine::Result<()> {
    use dialoguer::Input;

    // Load existing config or create new one
    let mut entries = load_vps_config(vps_file_path).unwrap_or_default();

    // Get input for name
    let name = match name {
        Some(n) if !n.is_empty() => n,
        _ => Input::new()
            .with_prompt("VPS Name")
            .interact_text()
            .map_err(|e| {
                crate::engine::error::ErrorCode::UiPromptFailed
                    .error()
                    .with_context("field", "name")
                    .with_context("error", e.to_string())
            })?,
    };

    // Get input for host
    let host = match host {
        Some(h) if !h.is_empty() => h,
        _ => Input::new()
            .with_prompt("Host")
            .interact_text()
            .map_err(|e| {
                crate::engine::error::ErrorCode::UiPromptFailed
                    .error()
                    .with_context("field", "host")
                    .with_context("error", e.to_string())
            })?,
    };

    // Get input for user (with default)
    let user = match user {
        Some(u) if !u.is_empty() => u,
        _ => {
            let default_user = std::env::var("USER").unwrap_or_else(|_| "root".to_string());
            Input::new()
                .with_prompt("User")
                .default(default_user)
                .interact_text()
                .map_err(|e| {
                    crate::engine::error::ErrorCode::UiPromptFailed
                        .error()
                        .with_context("field", "user")
                        .with_context("error", e.to_string())
                })?
        }
    };

    // Get input for port (with default 22)
    let port = match port {
        Some(p) if p > 0 => Some(p),
        _ => {
            let port_str: String = Input::new()
                .with_prompt("Port")
                .default("22".to_string())
                .interact_text()
                .map_err(|e| {
                    crate::engine::error::ErrorCode::UiPromptFailed
                        .error()
                        .with_context("field", "port")
                        .with_context("error", e.to_string())
                })?;
            port_str.parse::<u16>().ok()
        }
    };

    // Create new VPS entry
    let entry = VpsEntry {
        id: None, // Will be generated
        name: Some(name),
        host,
        user: Some(user),
        port: port.map(|p| crate::engine::models::vps::FlexibleValue::Number(p)),
        private_key: _private_key.map(|p| p.to_string_lossy().to_string()),
        post_connect_script: _post_connect_script,
    };

    // Validate the entry
    entry.validate().map_err(|e| {
        crate::engine::error::ErrorCode::VpsConfigInvalid
            .error()
            .with_context("validation", e)
    })?;

    // Add to entries and save
    entries.push(entry);
    save_vps_config(&entries, vps_file_path)?;

    println!("✅ VPS profile added successfully!");
    Ok(())
}

/// Save VPS entries to file.
fn save_vps_config(entries: &[VpsEntry], vps_file_path: &PathBuf) -> crate::engine::Result<()> {
    let ext = vps_file_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let content = match ext.as_str() {
        "yaml" | "yml" => serde_yaml::to_string(&entries).map_err(|e| {
            crate::engine::error::ErrorCode::SerializationFailure
                .error()
                .with_context("format", "yaml")
                .with_context("error", e.to_string())
        })?,
        _ => serde_json::to_string_pretty(&entries).map_err(|e| {
            crate::engine::error::ErrorCode::SerializationFailure
                .error()
                .with_context("format", "json")
                .with_context("error", e.to_string())
        })?,
    };

    // Ensure directory exists
    if let Some(parent) = vps_file_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            crate::engine::error::ErrorCode::IoFailure
                .error()
                .with_context("path", parent.display().to_string())
                .with_context("error", e.to_string())
        })?;
    }

    fs::write(vps_file_path, content).map_err(|e| {
        crate::engine::error::ErrorCode::IoFailure
            .error()
            .with_context("path", vps_file_path.display().to_string())
            .with_context("error", e.to_string())
    })?;

    Ok(())
}

/// Connect to a VPS server via SSH.
pub async fn connect(id_or_name: Option<String>, vps_file_path: &PathBuf) -> Result<()> {
    use dialoguer::Select;
    use std::process::Command;

    let entries = load_vps_config(vps_file_path)?;

    if entries.is_empty() {
        return Err(crate::engine::error::ErrorCode::VpsNotFound
            .error()
            .with_context("reason", "no VPS entries found"));
    }

    // Find VPS by ID or prompt to select
    let vps = match id_or_name {
        Some(id) => entries
            .iter()
            .find(|v| v.effective_id() == id || v.name.as_deref() == Some(&id))
            .ok_or_else(|| {
                crate::engine::error::ErrorCode::VpsNotFound
                    .error()
                    .with_context("id_or_name", id)
            })?,
        None => {
            let options: Vec<String> = entries
                .iter()
                .map(|v| {
                    let user = v.effective_user();
                    let name = v.name.as_deref().unwrap_or("-");
                    format!("{}@{} ({})", user, v.host, name)
                })
                .collect();

            let selection = Select::new()
                .with_prompt("🔌 Select a VPS to connect to")
                .items(&options)
                .default(0)
                .interact()
                .map_err(|e| {
                    crate::engine::error::ErrorCode::UiPromptFailed
                        .error()
                        .with_context("prompt", "vps selection")
                        .with_context("error", e.to_string())
                })?;

            &entries[selection]
        }
    };

    // Build SSH command
    let user = vps.effective_user();
    let host = &vps.host;
    let port = vps.effective_port();
    let remote = format!("{}@{}", user, host);

    let mut cmd = Command::new("ssh");
    cmd.arg("-tt"); // Force pseudo-terminal allocation

    if let Some(ref key) = vps.private_key {
        cmd.arg("-i");
        cmd.arg(key);
    }

    if port != 22 {
        cmd.arg("-p");
        cmd.arg(port.to_string());
    }

    cmd.arg(remote);

    println!("🔌 Connecting to {}@{}:{}", user, host, port);

    // Execute SSH - let it inherit stdio so terminal interaction works
    let status = cmd.status().map_err(|e| {
        crate::engine::error::ErrorCode::ShellCommandFailed
            .error()
            .with_context("host", host.to_string())
            .with_context("error", e.to_string())
    })?;

    if !status.success() {
        return Err(crate::engine::error::ErrorCode::ProcessNonZeroExit
            .error()
            .with_context("command", "ssh")
            .with_context("exit_code", status.code().unwrap_or(-1).to_string()));
    }

    Ok(())
}

/// Update a VPS entry in the configuration file.
pub async fn update(
    id_or_name: String,
    name: Option<String>,
    host: Option<String>,
    user: Option<String>,
    port: Option<u16>,
    private_key: Option<String>,
    vps_file_path: &PathBuf,
) -> Result<()> {
    let mut entries = load_vps_config(vps_file_path)?;

    // Find the entry to update
    let entry = entries
        .iter_mut()
        .find(|v| v.effective_id() == id_or_name || v.name.as_deref() == Some(&id_or_name))
        .ok_or_else(|| {
            crate::engine::error::ErrorCode::VpsNotFound
                .error()
                .with_context("id_or_name", id_or_name)
        })?;

    println!(
        "✏️  Updating VPS entry: {}@{}",
        entry.effective_user(),
        entry.host
    );

    let mut changed = false;

    if let Some(n) = name {
        entry.name = Some(n);
        changed = true;
    }

    if let Some(h) = host {
        entry.host = h;
        changed = true;
    }

    if let Some(u) = user {
        entry.user = Some(u);
        changed = true;
    }

    if let Some(p) = port {
        if p > 0 {
            entry.port = Some(crate::engine::models::vps::FlexibleValue::Number(p));
            changed = true;
        }
    }

    if let Some(pk) = private_key {
        entry.private_key = Some(pk);
        changed = true;
    }

    if !changed {
        println!("⚠️  No changes provided.");
        return Ok(());
    }

    // Validate updated entry
    entry.validate().map_err(|e| {
        crate::engine::error::ErrorCode::VpsConfigInvalid
            .error()
            .with_context("validation", e)
    })?;

    save_vps_config(&entries, vps_file_path)?;
    println!("✅ VPS entry updated successfully!");
    Ok(())
}

/// Delete a VPS entry from the configuration file.
pub async fn delete(id_or_name: Option<String>, vps_file_path: &PathBuf) -> Result<()> {
    use dialoguer::{Confirm, Select};

    let mut entries = load_vps_config(vps_file_path)?;

    if entries.is_empty() {
        return Err(crate::engine::error::ErrorCode::VpsNotFound
            .error()
            .with_context("reason", "no VPS entries found"));
    }

    // Find the entry to delete
    let index = match id_or_name {
        Some(id_or_name) => entries
            .iter()
            .position(|v| v.effective_id() == id_or_name || v.name.as_deref() == Some(&id_or_name))
            .ok_or_else(|| {
                crate::engine::error::ErrorCode::VpsNotFound
                    .error()
                    .with_context("id_or_name", id_or_name)
            })?,
        None => {
            let options: Vec<String> = entries
                .iter()
                .map(|v| {
                    let user = v.effective_user();
                    let name = v.name.as_deref().unwrap_or("-");
                    format!("{}@{} ({})", user, v.host, name)
                })
                .collect();

            let selection = Select::new()
                .with_prompt("🗑️  Select a VPS to delete")
                .items(&options)
                .default(0)
                .interact()
                .map_err(|e| {
                    crate::engine::error::ErrorCode::UiPromptFailed
                        .error()
                        .with_context("prompt", "vps selection")
                        .with_context("error", e.to_string())
                })?;

            selection
        }
    };

    let entry = &entries[index];
    println!(
        "🗑️  Delete entry: {}@{} ({})",
        entry.effective_user(),
        entry.host,
        entry.name.as_deref().unwrap_or("-")
    );

    // Confirm deletion
    let confirmed = Confirm::new()
        .with_prompt("Are you sure?")
        .default(false)
        .interact()
        .map_err(|e| {
            crate::engine::error::ErrorCode::UiPromptFailed
                .error()
                .with_context("prompt", "delete confirmation")
                .with_context("error", e.to_string())
        })?;

    if !confirmed {
        println!("❌ Deletion cancelled.");
        return Ok(());
    }

    entries.remove(index);
    save_vps_config(&entries, vps_file_path)?;
    println!("✅ VPS entry deleted successfully!");
    Ok(())
}

/// Run a script on a VPS server via SSH.
pub async fn script(
    id_or_name: Option<String>,
    inline_script: Option<String>,
    script_file: Option<PathBuf>,
    vps_file_path: &PathBuf,
) -> Result<()> {
    use dialoguer::Select;
    use std::process::Command;

    let entries = load_vps_config(vps_file_path)?;

    if entries.is_empty() {
        return Err(crate::engine::error::ErrorCode::VpsNotFound
            .error()
            .with_context("reason", "no VPS entries found"));
    }

    // Find VPS by ID or prompt to select
    let vps = match id_or_name {
        Some(id) => entries
            .iter()
            .find(|v| v.effective_id() == id || v.name.as_deref() == Some(&id))
            .ok_or_else(|| {
                crate::engine::error::ErrorCode::VpsNotFound
                    .error()
                    .with_context("id_or_name", id)
            })?,
        None => {
            let options: Vec<String> = entries
                .iter()
                .map(|v| {
                    let user = v.effective_user();
                    let name = v.name.as_deref().unwrap_or("-");
                    format!("{}@{} ({})", user, v.host, name)
                })
                .collect();

            let selection = Select::new()
                .with_prompt("💻 Select a VPS to run the script on")
                .items(&options)
                .default(0)
                .interact()
                .map_err(|e| {
                    crate::engine::error::ErrorCode::UiPromptFailed
                        .error()
                        .with_context("prompt", "vps selection")
                        .with_context("error", e.to_string())
                })?;

            &entries[selection]
        }
    };

    // Resolve script content
    let script_content = if let Some(inline) = inline_script {
        if !inline.trim().is_empty() {
            inline
        } else if let Some(script_str) = &vps.post_connect_script {
            script_str.clone()
        } else {
            return Err(crate::engine::error::ErrorCode::ValidationFailed
                .error()
                .with_context("reason", "no script provided (--inline, --file, or config)"));
        }
    } else if let Some(file) = script_file {
        fs::read_to_string(&file).map_err(|e| {
            crate::engine::error::ErrorCode::IoFailure
                .error()
                .with_context("path", file.display().to_string())
                .with_context("error", e.to_string())
        })?
    } else if let Some(script_str) = &vps.post_connect_script {
        script_str.clone()
    } else {
        return Err(crate::engine::error::ErrorCode::ValidationFailed
            .error()
            .with_context("reason", "no script provided"));
    };

    // Build SSH command to run the script
    let user = vps.effective_user();
    let host = &vps.host;
    let port = vps.effective_port();
    let remote = format!("{}@{}", user, host);

    let mut cmd = Command::new("ssh");

    if let Some(ref key) = vps.private_key {
        cmd.arg("-i");
        cmd.arg(key);
    }

    if port != 22 {
        cmd.arg("-p");
        cmd.arg(port.to_string());
    }

    cmd.arg(remote);
    cmd.arg("bash"); // Run bash on remote
    cmd.arg("-s"); // Read script from stdin

    println!("🚀 Running script on {}@{}", user, host);

    // Execute command with script as stdin
    let mut child = cmd
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| {
            crate::engine::error::ErrorCode::ShellCommandFailed
                .error()
                .with_context("command", "ssh bash")
                .with_context("error", e.to_string())
        })?;

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        let _ = stdin.write_all(script_content.as_bytes()).map_err(|e| {
            eprintln!("⚠️  Warning: Failed to write script to stdin: {}", e);
        });
    }

    let status = child.wait().map_err(|e| {
        crate::engine::error::ErrorCode::ShellCommandFailed
            .error()
            .with_context("command", "ssh bash")
            .with_context("error", e.to_string())
    })?;

    if !status.success() {
        return Err(crate::engine::error::ErrorCode::ProcessNonZeroExit
            .error()
            .with_context("command", "ssh bash")
            .with_context("exit_code", status.code().unwrap_or(-1).to_string()));
    }

    println!("✅ Script executed successfully!");
    Ok(())
}

// ---------------------------------------------------------------------------
// Copy files to/from VPS
// ---------------------------------------------------------------------------

/// Copy files to/from a VPS using SCP.
pub async fn copy(
    id_or_name: Option<String>,
    source: String,
    destination: String,
    direction: &str,
    vps_file_path: &PathBuf,
) -> Result<()> {
    use dialoguer::Select;

    let entries = load_vps_config(vps_file_path)?;

    if entries.is_empty() {
        println!("⚠️  No VPS profiles found in configuration.");
        return Ok(());
    }

    // Find VPS by ID or prompt to select
    let vps = match id_or_name {
        Some(id) => entries
            .iter()
            .find(|v| v.effective_id() == id || v.name.as_deref() == Some(&id))
            .ok_or_else(|| {
                crate::engine::error::ErrorCode::VpsNotFound
                    .error()
                    .with_context("id_or_name", id)
            })?,
        None => {
            if entries.len() == 1 {
                &entries[0]
            } else {
                let options: Vec<String> = entries
                    .iter()
                    .map(|v| {
                        let user = v.effective_user();
                        let name = v.name.as_deref().unwrap_or("-");
                        format!("{}@{} ({})", user, v.host, name)
                    })
                    .collect();

                let selection = Select::new()
                    .with_prompt("📋 Select a VPS to copy from/to")
                    .items(&options)
                    .default(0)
                    .interact()
                    .map_err(|e| {
                        crate::engine::error::ErrorCode::UiPromptFailed
                            .error()
                            .with_context("prompt", "vps selection")
                            .with_context("error", e.to_string())
                    })?;

                &entries[selection]
            }
        }
    };

    let port = vps.effective_port().to_string();
    let user = vps.effective_user();
    let host = &vps.host;
    let remote = format!("{user}@{host}");

    let mut cmd_args = vec!["scp".to_string()];

    // Add port
    cmd_args.push("-P".to_string());
    cmd_args.push(port);

    // Add private key if specified
    if let Some(ref key) = vps.private_key {
        cmd_args.push("-i".to_string());
        cmd_args.push(key.clone());
    }

    // Handle directions: push (local→remote), pull (remote→local), remote (remote→remote)
    match direction {
        "pull" => {
            // Remote to local: remote_path → local_path
            cmd_args.push(format!("{remote}:{source}"));
            cmd_args.push(destination);
        }
        "remote" => {
            // Remote to remote (via scp -3)
            cmd_args.push("-3".to_string());
            cmd_args.push(source);
            cmd_args.push(destination);
        }
        _ => {
            // Default: push (local to remote)
            cmd_args.push(source);
            cmd_args.push(format!("{remote}:{destination}"));
        }
    }

    println!("📋 Executing: {}", cmd_args.join(" "));

    let status = std::process::Command::new("bash")
        .arg("-c")
        .arg(cmd_args.join(" "))
        .status()
        .map_err(|e| {
            crate::engine::error::ErrorCode::ShellCommandFailed
                .error()
                .with_context("command", "scp")
                .with_context("error", e.to_string())
        })?;

    if !status.success() {
        return Err(crate::engine::error::ErrorCode::ProcessNonZeroExit
            .error()
            .with_context("command", "scp")
            .with_context("exit_code", status.code().unwrap_or(-1).to_string()));
    }

    println!("✅ File transfer completed successfully!");
    Ok(())
}

// ---------------------------------------------------------------------------
// Schema
// ---------------------------------------------------------------------------

/// Print VPS config schema as JSON example.
pub async fn schema(_ctx: &crate::core::Context) -> Result<()> {
    let example_vps = VpsEntry {
        id: Some("example-vps-root".into()),
        name: Some("Example VPS".into()),
        user: Some("root".into()),
        host: "example.com".into(),
        port: Some(crate::engine::models::vps::FlexibleValue::Number(22)),
        private_key: Some("/home/user/.ssh/id_rsa".into()),
        post_connect_script: Some("uptime && echo $USER".into()),
    };

    let config = serde_json::json!({
        "vps": vec![example_vps]
    });

    let json_str = serde_json::to_string_pretty(&config).map_err(|e| {
        crate::engine::error::ErrorCode::SerializationFailure
            .error()
            .with_context("format", "json")
            .with_context("error", e.to_string())
    })?;

    println!("{}", json_str);
    Ok(())
}
