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
    let mut entries = parse_vps_config(vps_file_path)?;
    assign_missing_ids(&mut entries);
    Ok(entries)
}

/// Give every entry without an id a unique one derived from its name.
fn assign_missing_ids(entries: &mut [VpsEntry]) {
    use crate::engine::models::vps::{generate_vps_id, unique_id};
    let mut taken: Vec<String> = entries.iter().filter_map(|e| e.id.clone()).collect();
    for e in entries.iter_mut().filter(|e| e.id.is_none()) {
        let base = generate_vps_id(e.name.as_deref(), &e.host, None);
        let id = unique_id(&base, taken.iter().map(String::as_str));
        taken.push(id.clone());
        e.id = Some(id);
    }
}

fn parse_vps_config(vps_file_path: &PathBuf) -> Result<Vec<VpsEntry>> {
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

/// Validate a VPS file: it must exist, parse, and every entry must be valid.
/// Returns the number of entries.
pub fn validate_file(path: &PathBuf) -> Result<usize> {
    if !path.exists() {
        return Err(crate::engine::error::ErrorCode::IoFailure
            .error()
            .with_context("path", path.display().to_string())
            .with_context("error", "file not found"));
    }
    let entries = parse_vps_config(path)?;
    let mut seen = std::collections::HashSet::new();
    for entry in &entries {
        if let Some(id) = &entry.id
            && !seen.insert(id.as_str())
        {
            return Err(crate::engine::error::ErrorCode::VpsConfigInvalid
                .error()
                .with_context("validation", format!("duplicate id `{id}`")));
        }
    }
    for (i, entry) in entries.iter().enumerate() {
        entry.validate().map_err(|e| {
            crate::engine::error::ErrorCode::VpsConfigInvalid
                .error()
                .with_context("entry", (i + 1).to_string())
                .with_context("validation", e)
        })?;
    }
    Ok(entries.len())
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

/// Runtime options shared by the VPS operations.
#[derive(Debug, Clone, Copy, Default)]
pub struct Opts {
    /// Print what would happen without changing anything.
    pub dry_run: bool,
    /// Prompts are allowed (a terminal is attached and not in --ci/--non-interactive).
    pub interactive: bool,
    /// Confirmations are answered "yes" automatically (--yes or --ci).
    pub assume_yes: bool,
}

fn entry_label(v: &VpsEntry) -> String {
    format!(
        "{} - {}@{} [{}]",
        v.name.as_deref().unwrap_or("-"),
        v.effective_user(),
        v.host,
        v.effective_id()
    )
}

/// Resolve the target entry by id/name, or prompt when interactive.
fn select_entry(
    entries: &[VpsEntry],
    id_or_name: Option<&str>,
    prompt: &str,
    opts: &Opts,
) -> Result<usize> {
    use crate::engine::error::ErrorCode;

    if entries.is_empty() {
        return Err(ErrorCode::VpsNotFound
            .error()
            .with_context("reason", "no VPS entries found"));
    }

    if let Some(id) = id_or_name {
        return entries
            .iter()
            .position(|v| v.id.as_deref() == Some(id))
            .or_else(|| entries.iter().position(|v| v.name.as_deref() == Some(id)))
            .ok_or_else(|| {
                ErrorCode::VpsNotFound
                    .error()
                    .with_context("id_or_name", id)
            });
    }

    if !opts.interactive {
        if entries.len() == 1 {
            return Ok(0);
        }
        return Err(ErrorCode::ValidationFailed
            .error()
            .with_context("reason", "--id is required when not running interactively"));
    }

    let options: Vec<String> = entries.iter().map(entry_label).collect();
    dialoguer::Select::new()
        .with_prompt(prompt)
        .items(&options)
        .default(0)
        .interact()
        .map_err(|e| {
            ErrorCode::UiPromptFailed
                .error()
                .with_context("prompt", "vps selection")
                .with_context("error", e.to_string())
        })
}

/// Add a new VPS entry. Never prompts: the user defaults to the current OS user
/// and the port to 22.
#[allow(clippy::too_many_arguments)]
pub async fn add(
    id: Option<String>,
    name: String,
    host: String,
    user: Option<String>,
    port: Option<u16>,
    private_key: Option<PathBuf>,
    post_connect_script: Option<String>,
    vps_file_path: &PathBuf,
    opts: &Opts,
) -> crate::engine::Result<()> {
    use crate::engine::error::ErrorCode;

    let mut entries = load_vps_config(vps_file_path)?;

    if name.trim().is_empty() || host.trim().is_empty() {
        return Err(ErrorCode::ValidationFailed
            .error()
            .with_context("reason", "name and host must not be empty"));
    }

    if entries
        .iter()
        .any(|v| v.name.as_deref() == Some(name.as_str()))
    {
        return Err(ErrorCode::ValidationFailed
            .error()
            .with_context("reason", "a VPS profile with this name already exists")
            .with_context("name", name));
    }

    let user = match user {
        Some(u) if !u.is_empty() => u,
        _ => crate::engine::models::vps::get_current_user().unwrap_or_else(|| "root".to_string()),
    };

    let id = match id.filter(|i| !i.is_empty()).map(|i| i.to_lowercase()) {
        Some(id) => {
            crate::engine::models::vps::validate_id_format(&id).map_err(|e| {
                ErrorCode::ValidationFailed
                    .error()
                    .with_context("reason", e)
            })?;
            if entries.iter().any(|v| v.id.as_deref() == Some(id.as_str())) {
                return Err(ErrorCode::ValidationFailed
                    .error()
                    .with_context("reason", "a VPS profile with this id already exists")
                    .with_context("id", id));
            }
            id
        }
        None => {
            use crate::engine::models::vps::{generate_vps_id, unique_id};
            let base = generate_vps_id(Some(&name), &host, None);
            unique_id(&base, entries.iter().filter_map(|v| v.id.as_deref()))
        }
    };

    let entry = VpsEntry {
        id: Some(id),
        name: Some(name),
        host,
        user: Some(user),
        port: port
            .filter(|p| *p > 0)
            .map(crate::engine::models::vps::FlexibleValue::Number),
        private_key: private_key.map(|p| p.to_string_lossy().to_string()),
        post_connect_script,
    };

    entry.validate().map_err(|e| {
        ErrorCode::VpsConfigInvalid
            .error()
            .with_context("validation", e)
    })?;

    if opts.dry_run {
        println!(
            "[dry run] would add VPS profile {} to {}",
            entry_label(&entry),
            vps_file_path.display()
        );
        return Ok(());
    }

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
pub async fn connect(
    id_or_name: Option<String>,
    vps_file_path: &PathBuf,
    opts: &Opts,
) -> Result<()> {
    use std::process::Command;

    let entries = load_vps_config(vps_file_path)?;
    let idx = select_entry(
        &entries,
        id_or_name.as_deref(),
        "🔌 Select a VPS to connect to",
        opts,
    )?;
    let vps = &entries[idx];

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

    if opts.dry_run {
        println!("[dry run] would connect to {}@{}:{}", user, host, port);
        return Ok(());
    }

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
#[allow(clippy::too_many_arguments)]
pub async fn update(
    id_or_name: Option<String>,
    new_id: Option<String>,
    mut name: Option<String>,
    mut host: Option<String>,
    mut user: Option<String>,
    mut port: Option<u16>,
    mut private_key: Option<String>,
    vps_file_path: &PathBuf,
    opts: &Opts,
) -> Result<()> {
    use crate::engine::error::ErrorCode;

    let mut new_id = new_id.map(|i| i.to_lowercase());
    let mut entries = load_vps_config(vps_file_path)?;

    let idx = select_entry(
        &entries,
        id_or_name.as_deref(),
        "✏️  Select a VPS to update",
        opts,
    )?;

    // Interactive with no change flags: prompt for each field, pre-filled with the
    // current value. Unchanged answers are left as `None`.
    let no_flags = new_id.is_none()
        && name.is_none()
        && host.is_none()
        && user.is_none()
        && port.is_none()
        && private_key.is_none();
    if no_flags && opts.interactive {
        let cur = entries[idx].clone();
        let ask = |label: &str, current: String| -> Result<Option<String>> {
            let v: String = dialoguer::Input::new()
                .with_prompt(label)
                .with_initial_text(current.clone())
                .allow_empty(true)
                .interact_text()
                .map_err(|e| {
                    ErrorCode::UiPromptFailed
                        .error()
                        .with_context("field", label.to_string())
                        .with_context("error", e.to_string())
                })?;
            Ok((v != current).then_some(v))
        };
        new_id = ask("Id", cur.effective_id().to_string())?.map(|i| i.to_lowercase());
        name = ask("Name", cur.name.clone().unwrap_or_default())?;
        host = ask("Host", cur.host.clone())?;
        user = ask("User", cur.effective_user())?;
        let port_answer = ask("Port", cur.effective_port().to_string())?;
        port = match port_answer {
            None => None,
            Some(p) => Some(p.trim().parse::<u16>().map_err(|_| {
                ErrorCode::ValidationFailed
                    .error()
                    .with_context("reason", "port must be a number between 1 and 65535")
            })?),
        };
        private_key = ask("Private key", cur.private_key.clone().unwrap_or_default())?
            .filter(|k| !k.trim().is_empty());
    }

    if let Some(new) = &new_id {
        crate::engine::models::vps::validate_id_format(new).map_err(|e| {
            ErrorCode::ValidationFailed
                .error()
                .with_context("reason", e)
        })?;
        if entries
            .iter()
            .enumerate()
            .any(|(i, v)| i != idx && v.id.as_deref() == Some(new.as_str()))
        {
            return Err(ErrorCode::ValidationFailed
                .error()
                .with_context("reason", "a VPS profile with this id already exists")
                .with_context("id", new.clone()));
        }
    }
    if let Some(n) = &name {
        if n.trim().is_empty() {
            return Err(ErrorCode::ValidationFailed
                .error()
                .with_context("reason", "name must not be empty"));
        }
        if entries
            .iter()
            .enumerate()
            .any(|(i, v)| i != idx && v.name.as_deref() == Some(n.as_str()))
        {
            return Err(ErrorCode::ValidationFailed
                .error()
                .with_context("reason", "a VPS profile with this name already exists")
                .with_context("name", n.clone()));
        }
    }
    if host.as_deref().is_some_and(|h| h.trim().is_empty()) {
        return Err(ErrorCode::ValidationFailed
            .error()
            .with_context("reason", "host must not be empty"));
    }
    if port == Some(0) {
        return Err(ErrorCode::ValidationFailed
            .error()
            .with_context("reason", "port must be between 1 and 65535"));
    }
    if new_id.is_none()
        && name.is_none()
        && host.is_none()
        && user.is_none()
        && port.is_none()
        && private_key.is_none()
    {
        return Err(ErrorCode::ValidationFailed.error().with_context(
            "reason",
            "no changes provided (use --new-id, --name, --host, --user, --port or --private-key)",
        ));
    }

    let entry = &mut entries[idx];

    println!(
        "✏️  Updating VPS entry: {}@{}",
        entry.effective_user(),
        entry.host
    );

    if let Some(i) = new_id {
        entry.id = Some(i);
    }
    if let Some(n) = name {
        entry.name = Some(n);
    }
    if let Some(h) = host {
        entry.host = h;
    }
    if let Some(u) = user {
        entry.user = Some(u);
    }
    if let Some(p) = port {
        entry.port = Some(crate::engine::models::vps::FlexibleValue::Number(p));
    }
    if let Some(pk) = private_key {
        entry.private_key = Some(pk);
    }

    // Validate updated entry
    entry.validate().map_err(|e| {
        crate::engine::error::ErrorCode::VpsConfigInvalid
            .error()
            .with_context("validation", e)
    })?;

    if opts.dry_run {
        println!(
            "[dry run] would update VPS entry in {}",
            vps_file_path.display()
        );
        return Ok(());
    }

    save_vps_config(&entries, vps_file_path)?;
    println!("✅ VPS entry updated successfully!");
    Ok(())
}

/// Delete a VPS entry from the configuration file.
pub async fn delete(
    id_or_name: Option<String>,
    vps_file_path: &PathBuf,
    opts: &Opts,
) -> Result<()> {
    use crate::engine::error::ErrorCode;

    let mut entries = load_vps_config(vps_file_path)?;
    let index = select_entry(
        &entries,
        id_or_name.as_deref(),
        "🗑️  Select a VPS to delete",
        opts,
    )?;

    println!("🗑️  Delete entry: {}", entry_label(&entries[index]));

    if opts.dry_run {
        println!(
            "[dry run] would delete this entry from {}",
            vps_file_path.display()
        );
        return Ok(());
    }

    if !opts.assume_yes {
        if !opts.interactive {
            return Err(ErrorCode::ValidationFailed
                .error()
                .with_context("reason", "confirmation required: pass --yes (or --ci)"));
        }
        let confirmed = dialoguer::Confirm::new()
            .with_prompt("Are you sure?")
            .default(false)
            .interact()
            .map_err(|e| {
                ErrorCode::UiPromptFailed
                    .error()
                    .with_context("prompt", "delete confirmation")
                    .with_context("error", e.to_string())
            })?;
        if !confirmed {
            println!("❌ Deletion cancelled.");
            return Ok(());
        }
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
    opts: &Opts,
) -> Result<()> {
    use std::process::Command;

    let entries = load_vps_config(vps_file_path)?;
    let idx = select_entry(
        &entries,
        id_or_name.as_deref(),
        "💻 Select a VPS to run the script on",
        opts,
    )?;
    let vps = &entries[idx];

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

    if opts.dry_run {
        println!("[dry run] would run script on {}@{}", user, host);
        return Ok(());
    }

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
    opts: &Opts,
) -> Result<()> {
    let entries = load_vps_config(vps_file_path)?;
    let idx = select_entry(
        &entries,
        id_or_name.as_deref(),
        "📋 Select a VPS to copy from/to",
        opts,
    )?;
    let vps = &entries[idx];

    let port = vps.effective_port().to_string();
    let remote = format!("{}@{}", vps.effective_user(), vps.host);

    let mut args: Vec<String> = vec!["-P".into(), port];

    if let Some(ref key) = vps.private_key {
        args.push("-i".into());
        args.push(key.clone());
    }

    // push (local→remote), pull (remote→local), remote (remote→remote)
    match direction {
        "pull" => {
            args.push(format!("{remote}:{source}"));
            args.push(destination);
        }
        "remote" => {
            args.push("-3".into());
            args.push(source);
            args.push(destination);
        }
        _ => {
            args.push(source);
            args.push(format!("{remote}:{destination}"));
        }
    }

    if opts.dry_run {
        println!("[dry run] would execute: scp {}", args.join(" "));
        return Ok(());
    }

    println!("📋 Executing: scp {}", args.join(" "));

    let status = std::process::Command::new("scp")
        .args(&args)
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
