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

    parse_vps_str(&content, vps_file_path)
}

fn ext_of(path: &std::path::Path) -> String {
    path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase()
}

fn parse_vps_str(content: &str, path: &std::path::Path) -> Result<Vec<VpsEntry>> {
    match ext_of(path).as_str() {
        "yaml" | "yml" => {
            // Try wrapped format first, then bare array
            if let Ok(wrapper) = serde_yaml::from_str::<VpsConfigWrapper>(content) {
                Ok(wrapper.vps)
            } else {
                serde_yaml::from_str::<Vec<VpsEntry>>(content).map_err(|e| {
                    crate::engine::error::ErrorCode::ConfigInvalid
                        .error()
                        .with_context("format", "yaml")
                        .with_context("error", e.to_string())
                })
            }
        }
        _ => {
            // JSON: try bare array first, then wrapped
            if let Ok(entries) = serde_json::from_str::<Vec<VpsEntry>>(content) {
                Ok(entries)
            } else if let Ok(wrapper) = serde_json::from_str::<VpsConfigWrapper>(content) {
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
    check_entries(&entries)?;
    Ok(entries.len())
}

/// Check a set of entries for duplicate ids and per-entry validity.
fn check_entries(entries: &[VpsEntry]) -> Result<()> {
    let mut seen = std::collections::HashSet::new();
    for entry in entries {
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
    Ok(())
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
    let entries = filter_entries(load_vps_config(vps_file_path)?, user_filter, host_filter);

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

fn filter_entries(
    mut entries: Vec<VpsEntry>,
    user_filter: Option<&str>,
    host_filter: Option<&str>,
) -> Vec<VpsEntry> {
    if let Some(user) = user_filter {
        entries.retain(|v| v.user.as_deref().unwrap_or("") == user);
    }
    if let Some(host) = host_filter {
        entries.retain(|v| v.host == host);
    }
    entries
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
    /// `ConnectTimeout` (seconds) passed to ssh/scp, when set.
    pub timeout: Option<u32>,
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

/// SSH/SCP options shared by every remote command:
/// - `StrictHostKeyChecking=accept-new` trusts a host the first time but still
///   refuses a host whose key later changes (unlike `no`, which trusts blindly).
/// - Non-interactive runs add `BatchMode=yes` so a password/host-key prompt
///   fails fast instead of hanging a script or CI job.
/// - `--timeout` (if set) becomes `ConnectTimeout`, bounding the connection phase.
fn ssh_common_args(opts: &Opts) -> Vec<String> {
    let mut args = vec![
        "-o".to_string(),
        "StrictHostKeyChecking=accept-new".to_string(),
    ];
    if !opts.interactive {
        args.push("-o".to_string());
        args.push("BatchMode=yes".to_string());
    }
    if let Some(t) = opts.timeout {
        args.push("-o".to_string());
        args.push(format!("ConnectTimeout={t}"));
    }
    args
}

/// Append the entry's private key, jump host and non-default port to `args`.
fn push_connection_args(args: &mut Vec<String>, vps: &VpsEntry, port_flag: &str) {
    if let Some(ref key) = vps.private_key {
        args.push("-i".to_string());
        args.push(key.clone());
    }
    if let Some(ref jump) = vps.jump_host {
        args.push("-J".to_string());
        args.push(jump.clone());
    }
    let port = vps.effective_port();
    if port != 22 {
        args.push(port_flag.to_string());
        args.push(port.to_string());
    }
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

/// Resolve the entries a bulk operation (`--all`) should target.
fn select_targets(entries: &[VpsEntry], all: bool) -> Result<Vec<&VpsEntry>> {
    if !all {
        return Err(crate::engine::error::ErrorCode::ValidationFailed
            .error()
            .with_context("reason", "internal: select_targets called without --all"));
    }
    if entries.is_empty() {
        return Err(crate::engine::error::ErrorCode::VpsNotFound
            .error()
            .with_context("reason", "no VPS entries found"));
    }
    Ok(entries.iter().collect())
}

/// Add a new VPS entry. Never prompts for connection details: the user defaults
/// to the current OS user and the port to 22. May prompt for the id alone.
#[allow(clippy::too_many_arguments)]
pub async fn add(
    id: Option<String>,
    name: String,
    host: String,
    user: Option<String>,
    port: Option<u16>,
    private_key: Option<PathBuf>,
    post_connect_script: Option<String>,
    jump_host: Option<String>,
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
            let generated = unique_id(&base, entries.iter().filter_map(|v| v.id.as_deref()));
            if opts.interactive && !opts.dry_run {
                // Show the generated id so the user can accept it or type their own.
                let taken: Vec<String> = entries.iter().filter_map(|v| v.id.clone()).collect();
                let answer: String = dialoguer::Input::new()
                    .with_prompt("Id")
                    .with_initial_text(generated)
                    .validate_with(move |input: &String| -> std::result::Result<(), String> {
                        let id = input.to_lowercase();
                        crate::engine::models::vps::validate_id_format(&id)?;
                        if taken.contains(&id) {
                            return Err(format!("id `{id}` is already in use"));
                        }
                        Ok(())
                    })
                    .interact_text()
                    .map_err(|e| {
                        ErrorCode::UiPromptFailed
                            .error()
                            .with_context("field", "id")
                            .with_context("error", e.to_string())
                    })?;
                answer.to_lowercase()
            } else {
                generated
            }
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
        jump_host,
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
    let content = serialize_entries(entries, vps_file_path)?;

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

fn serialize_entries(entries: &[VpsEntry], vps_file_path: &std::path::Path) -> Result<String> {
    match ext_of(vps_file_path).as_str() {
        "yaml" | "yml" => serde_yaml::to_string(&entries).map_err(|e| {
            crate::engine::error::ErrorCode::SerializationFailure
                .error()
                .with_context("format", "yaml")
                .with_context("error", e.to_string())
        }),
        _ => serde_json::to_string_pretty(&entries).map_err(|e| {
            crate::engine::error::ErrorCode::SerializationFailure
                .error()
                .with_context("format", "json")
                .with_context("error", e.to_string())
        }),
    }
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
    cmd.args(ssh_common_args(opts));

    let mut conn_args = Vec::new();
    push_connection_args(&mut conn_args, vps, "-p");
    cmd.args(&conn_args);

    cmd.arg(remote);

    // Run the profile's post-connect script, then drop into a login shell.
    let post = vps
        .post_connect_script
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());
    if let Some(script) = post {
        cmd.arg(format!("{script}\nexec \"${{SHELL:-/bin/sh}}\" -l"));
    }

    if opts.dry_run {
        println!("[dry run] would connect to {}@{}:{}", user, host, port);
        if post.is_some() {
            println!("[dry run] would run the post-connect script first");
        }
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
    post_connect_script: Option<String>,
    jump_host: Option<String>,
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
        && post_connect_script.is_none()
        && jump_host.is_none()
    {
        return Err(ErrorCode::ValidationFailed.error().with_context(
            "reason",
            "no changes provided (use --new-id, --name, --host, --user, --port, --private-key, --post-connect-script or --jump-host)",
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
    if let Some(script) = post_connect_script {
        entry.post_connect_script = Some(script).filter(|s| !s.trim().is_empty());
    }
    if let Some(jump) = jump_host {
        entry.jump_host = Some(jump).filter(|s| !s.trim().is_empty());
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

/// Build the ssh argument list to run `script_content` (via stdin) on `vps`.
fn script_ssh_command(vps: &VpsEntry, opts: &Opts) -> std::process::Command {
    use std::process::Command;
    let user = vps.effective_user();
    let remote = format!("{}@{}", user, vps.host);

    let mut cmd = Command::new("ssh");
    cmd.args(ssh_common_args(opts));

    let mut conn_args = Vec::new();
    push_connection_args(&mut conn_args, vps, "-p");
    cmd.args(&conn_args);

    cmd.arg(remote);
    cmd.arg("bash"); // Run bash on remote
    cmd.arg("-s"); // Read script from stdin
    cmd
}

/// Run `script_content` on a single entry, returning whether it succeeded.
fn run_script_on(vps: &VpsEntry, script_content: &str, opts: &Opts) -> Result<()> {
    use crate::engine::error::ErrorCode;

    let mut cmd = script_ssh_command(vps, opts);

    let mut child = cmd
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| {
            ErrorCode::ShellCommandFailed
                .error()
                .with_context("command", "ssh bash")
                .with_context("error", e.to_string())
        })?;

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        if let Err(e) = stdin.write_all(script_content.as_bytes()) {
            eprintln!("⚠️  Warning: Failed to write script to stdin: {e}");
        }
    }

    let status = child.wait().map_err(|e| {
        ErrorCode::ShellCommandFailed
            .error()
            .with_context("command", "ssh bash")
            .with_context("error", e.to_string())
    })?;

    if !status.success() {
        return Err(ErrorCode::ProcessNonZeroExit
            .error()
            .with_context("command", "ssh bash")
            .with_context("exit_code", status.code().unwrap_or(-1).to_string()));
    }
    Ok(())
}

/// Resolve the script content to run: `--inline`, `--file`, or the entry's
/// `post_connect_script` as a fallback.
fn resolve_script_content(
    inline_script: &Option<String>,
    script_file: &Option<PathBuf>,
    vps: &VpsEntry,
) -> Result<String> {
    use crate::engine::error::ErrorCode;

    if let Some(inline) = inline_script {
        if !inline.trim().is_empty() {
            return Ok(inline.clone());
        }
        return vps.post_connect_script.clone().ok_or_else(|| {
            ErrorCode::ValidationFailed
                .error()
                .with_context("reason", "no script provided (--inline, --file, or config)")
        });
    }
    if let Some(file) = script_file {
        return fs::read_to_string(file).map_err(|e| {
            ErrorCode::IoFailure
                .error()
                .with_context("path", file.display().to_string())
                .with_context("error", e.to_string())
        });
    }
    vps.post_connect_script.clone().ok_or_else(|| {
        ErrorCode::ValidationFailed
            .error()
            .with_context("reason", "no script provided")
    })
}

/// Run a script on a VPS server via SSH, or on every profile with `all`.
pub async fn script(
    id_or_name: Option<String>,
    inline_script: Option<String>,
    script_file: Option<PathBuf>,
    all: bool,
    vps_file_path: &PathBuf,
    opts: &Opts,
) -> Result<()> {
    let entries = load_vps_config(vps_file_path)?;

    if all {
        let targets = select_targets(&entries, true)?;
        if opts.dry_run {
            for vps in &targets {
                println!("[dry run] would run script on {}", entry_label(vps));
            }
            return Ok(());
        }
        let mut failures = Vec::new();
        for vps in &targets {
            let script_content = resolve_script_content(&inline_script, &script_file, vps)?;
            println!("🚀 Running script on {}", entry_label(vps));
            if let Err(e) = run_script_on(vps, &script_content, opts) {
                eprintln!("  ❌ {}: {}", entry_label(vps), e.message);
                failures.push(vps.effective_id().to_string());
            } else {
                println!("  ✅ {}", entry_label(vps));
            }
        }
        return report_bulk_result("script", targets.len(), failures);
    }

    let idx = select_entry(
        &entries,
        id_or_name.as_deref(),
        "💻 Select a VPS to run the script on",
        opts,
    )?;
    let vps = &entries[idx];
    let script_content = resolve_script_content(&inline_script, &script_file, vps)?;

    if opts.dry_run {
        println!("[dry run] would run script on {}", entry_label(vps));
        return Ok(());
    }

    println!("🚀 Running script on {}@{}", vps.effective_user(), vps.host);
    run_script_on(vps, &script_content, opts)?;
    println!("✅ Script executed successfully!");
    Ok(())
}

/// Summarize a bulk (`--all`) operation and turn any failures into an error.
fn report_bulk_result(op: &str, total: usize, failures: Vec<String>) -> Result<()> {
    if failures.is_empty() {
        println!("✅ {op} succeeded on all {total} profile(s).");
        return Ok(());
    }
    Err(crate::engine::error::ErrorCode::ProcessNonZeroExit
        .error()
        .with_context(
            "reason",
            format!(
                "{op} failed on {}/{} profile(s): {}",
                failures.len(),
                total,
                failures.join(", ")
            ),
        ))
}

// ---------------------------------------------------------------------------
// Copy files to/from VPS
// ---------------------------------------------------------------------------

fn copy_args(
    vps: &VpsEntry,
    source: &str,
    destination: &str,
    direction: &str,
    opts: &Opts,
) -> Vec<String> {
    let remote = format!("{}@{}", vps.effective_user(), vps.host);
    let mut args = ssh_common_args(opts);
    push_connection_args(&mut args, vps, "-P");

    // push (local→remote), pull (remote→local), remote (remote→remote, same host)
    match direction {
        "pull" => {
            args.push(format!("{remote}:{source}"));
            args.push(destination.to_string());
        }
        "remote" => {
            args.push("-3".to_string());
            args.push(source.to_string());
            args.push(destination.to_string());
        }
        _ => {
            args.push(source.to_string());
            args.push(format!("{remote}:{destination}"));
        }
    }
    args
}

/// Build `scp` args for a remote-to-remote copy between two different profiles.
fn copy_args_between(
    from: &VpsEntry,
    to: &VpsEntry,
    source: &str,
    destination: &str,
    opts: &Opts,
) -> Vec<String> {
    let mut args = ssh_common_args(opts);
    // scp -3 only takes one -i/-J pair; prefer the source host's.
    push_connection_args(&mut args, from, "-P");
    args.push("-3".to_string());
    args.push(format!(
        "{}@{}:{}",
        from.effective_user(),
        from.host,
        source
    ));
    args.push(format!(
        "{}@{}:{}",
        to.effective_user(),
        to.host,
        destination
    ));
    args
}

fn run_scp(args: &[String], opts: &Opts) -> Result<()> {
    use crate::engine::error::ErrorCode;

    if opts.dry_run {
        println!("[dry run] would execute: scp {}", args.join(" "));
        return Ok(());
    }

    println!("📋 Executing: scp {}", args.join(" "));

    let status = std::process::Command::new("scp")
        .args(args)
        .status()
        .map_err(|e| {
            ErrorCode::ShellCommandFailed
                .error()
                .with_context("command", "scp")
                .with_context("error", e.to_string())
        })?;

    if !status.success() {
        return Err(ErrorCode::ProcessNonZeroExit
            .error()
            .with_context("command", "scp")
            .with_context("exit_code", status.code().unwrap_or(-1).to_string()));
    }
    Ok(())
}

/// Copy files to/from a VPS using SCP. With `all`, copy to/from every profile.
/// With `to_id` set, copy directly between two different profiles (host-to-host).
#[allow(clippy::too_many_arguments)]
pub async fn copy(
    id_or_name: Option<String>,
    to_id: Option<String>,
    source: String,
    destination: String,
    direction: &str,
    all: bool,
    vps_file_path: &PathBuf,
    opts: &Opts,
) -> Result<()> {
    use crate::engine::error::ErrorCode;

    let entries = load_vps_config(vps_file_path)?;

    if let Some(to_id) = to_id {
        let from_idx = select_entry(
            &entries,
            id_or_name.as_deref(),
            "📋 Select the source VPS",
            opts,
        )?;
        let to_idx = entries
            .iter()
            .position(|v| {
                v.id.as_deref() == Some(to_id.as_str()) || v.name.as_deref() == Some(to_id.as_str())
            })
            .ok_or_else(|| {
                ErrorCode::VpsNotFound
                    .error()
                    .with_context("to_id", to_id.clone())
            })?;
        let args = copy_args_between(
            &entries[from_idx],
            &entries[to_idx],
            &source,
            &destination,
            opts,
        );
        run_scp(&args, opts)?;
        if !opts.dry_run {
            println!("✅ File transfer completed successfully!");
        }
        return Ok(());
    }

    if all {
        let targets = select_targets(&entries, true)?;
        let mut failures = Vec::new();
        for vps in &targets {
            let args = copy_args(vps, &source, &destination, direction, opts);
            println!("--- {}", entry_label(vps));
            if let Err(e) = run_scp(&args, opts) {
                eprintln!("  ❌ {}: {}", entry_label(vps), e.message);
                failures.push(vps.effective_id().to_string());
            }
        }
        if opts.dry_run {
            return Ok(());
        }
        return report_bulk_result("copy", targets.len(), failures);
    }

    let idx = select_entry(
        &entries,
        id_or_name.as_deref(),
        "📋 Select a VPS to copy from/to",
        opts,
    )?;
    let vps = &entries[idx];
    let args = copy_args(vps, &source, &destination, direction, opts);
    run_scp(&args, opts)?;
    if !opts.dry_run {
        println!("✅ File transfer completed successfully!");
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Connectivity check
// ---------------------------------------------------------------------------

/// Try to open (and immediately close) an SSH connection to check reachability.
/// Always non-interactive: it never prompts, regardless of `opts.interactive`.
fn probe(vps: &VpsEntry, opts: &Opts) -> bool {
    use std::process::Command;

    let mut probe_opts = *opts;
    probe_opts.interactive = false; // never prompt during a reachability check

    let remote = format!("{}@{}", vps.effective_user(), vps.host);
    let mut cmd = Command::new("ssh");
    cmd.args(ssh_common_args(&probe_opts));
    let mut conn_args = Vec::new();
    push_connection_args(&mut conn_args, vps, "-p");
    cmd.args(&conn_args);
    cmd.arg(remote);
    cmd.arg("true");

    cmd.stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Check reachability of one profile, or every profile with `all`.
/// Returns `(id, reachable)` for each profile checked.
pub async fn ping(
    id_or_name: Option<String>,
    all: bool,
    vps_file_path: &PathBuf,
    opts: &Opts,
) -> Result<Vec<(String, bool)>> {
    let entries = load_vps_config(vps_file_path)?;

    let targets: Vec<&VpsEntry> = if all {
        select_targets(&entries, true)?
    } else {
        let idx = select_entry(
            &entries,
            id_or_name.as_deref(),
            "📡 Select a VPS to test",
            opts,
        )?;
        vec![&entries[idx]]
    };

    let mut results = Vec::with_capacity(targets.len());
    for vps in targets {
        let label = entry_label(vps);
        if opts.dry_run {
            println!("[dry run] would test connectivity to {label}");
            continue;
        }
        let reachable = probe(vps, opts);
        println!("{} {}", if reachable { "✅" } else { "❌" }, label);
        results.push((vps.effective_id().to_string(), reachable));
    }
    Ok(results)
}

// ---------------------------------------------------------------------------
// Export / import
// ---------------------------------------------------------------------------

/// Serialize every profile for backup/sharing. Returns the serialized text.
pub fn export_entries(vps_file_path: &PathBuf) -> Result<String> {
    let entries = load_vps_config(vps_file_path)?;
    serialize_entries(&entries, vps_file_path)
}

/// Import profiles from `source` into `vps_file_path`.
/// With `merge`, entries whose id already exists are skipped; profiles are
/// otherwise appended. Without `merge`, the existing file is replaced outright.
/// Returns `(imported, skipped)`.
pub fn import_entries(
    source: &std::path::Path,
    vps_file_path: &PathBuf,
    merge: bool,
    opts: &Opts,
) -> Result<(usize, usize)> {
    use crate::engine::error::ErrorCode;

    if !source.exists() {
        return Err(ErrorCode::IoFailure
            .error()
            .with_context("path", source.display().to_string())
            .with_context("error", "file not found"));
    }
    let content = fs::read_to_string(source).map_err(|e| {
        ErrorCode::IoFailure
            .error()
            .with_context("path", source.display().to_string())
            .with_context("error", e.to_string())
    })?;
    let mut incoming = parse_vps_str(&content, source)?;
    assign_missing_ids(&mut incoming);
    check_entries(&incoming)?;

    let (final_entries, imported, skipped) = if merge {
        let mut existing = load_vps_config(vps_file_path)?;
        let existing_ids: std::collections::HashSet<String> =
            existing.iter().filter_map(|e| e.id.clone()).collect();
        let mut imported = 0;
        let mut skipped = 0;
        for entry in incoming {
            match &entry.id {
                Some(id) if existing_ids.contains(id) => skipped += 1,
                _ => {
                    existing.push(entry);
                    imported += 1;
                }
            }
        }
        (existing, imported, skipped)
    } else {
        let count = incoming.len();
        (incoming, count, 0)
    };

    if opts.dry_run {
        println!(
            "[dry run] would import {imported} profile(s) ({skipped} skipped as duplicates) into {}",
            vps_file_path.display()
        );
        return Ok((imported, skipped));
    }

    save_vps_config(&final_entries, vps_file_path)?;
    Ok((imported, skipped))
}

// ---------------------------------------------------------------------------
// Schema
// ---------------------------------------------------------------------------

/// Print the VPS config JSON Schema, generated from `VpsConfig`, with an example.
pub async fn schema(_ctx: &crate::core::Context) -> Result<()> {
    use crate::engine::models::vps::VpsConfig;
    use schemars::schema_for;

    let example_vps = VpsEntry {
        id: Some("example-vps-root".into()),
        name: Some("Example VPS".into()),
        user: Some("root".into()),
        host: "example.com".into(),
        port: Some(crate::engine::models::vps::FlexibleValue::Number(22)),
        private_key: Some("/home/user/.ssh/id_rsa".into()),
        post_connect_script: Some("uptime && echo $USER".into()),
        jump_host: None,
    };

    let schema = schema_for!(VpsConfig);
    let mut schema_value = serde_json::to_value(&schema).map_err(|e| {
        crate::engine::error::ErrorCode::SerializationFailure
            .error()
            .with_context("format", "json")
            .with_context("error", e.to_string())
    })?;

    if let Some(obj) = schema_value.as_object_mut() {
        obj.insert(
            "example".to_string(),
            serde_json::json!({ "vps": vec![example_vps] }),
        );
    }

    let json_str = serde_json::to_string_pretty(&schema_value).map_err(|e| {
        crate::engine::error::ErrorCode::SerializationFailure
            .error()
            .with_context("format", "json")
            .with_context("error", e.to_string())
    })?;

    println!("{}", json_str);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(id: &str, name: &str, host: &str) -> VpsEntry {
        VpsEntry {
            id: Some(id.to_string()),
            name: Some(name.to_string()),
            host: host.to_string(),
            user: None,
            port: None,
            private_key: None,
            post_connect_script: None,
            jump_host: None,
        }
    }

    fn opts(interactive: bool) -> Opts {
        Opts {
            interactive,
            ..Default::default()
        }
    }

    #[test]
    fn select_entry_matches_id_then_name() {
        let entries = vec![entry("a1", "Alpha", "a.com"), entry("b1", "Beta", "b.com")];
        assert_eq!(
            select_entry(&entries, Some("b1"), "", &opts(false)).unwrap(),
            1
        );
        assert_eq!(
            select_entry(&entries, Some("Alpha"), "", &opts(false)).unwrap(),
            0
        );
        assert!(select_entry(&entries, Some("nope"), "", &opts(false)).is_err());
    }

    #[test]
    fn select_entry_without_id_requires_interactive_unless_single() {
        let one = vec![entry("a1", "Alpha", "a.com")];
        assert_eq!(select_entry(&one, None, "", &opts(false)).unwrap(), 0);

        let two = vec![entry("a1", "Alpha", "a.com"), entry("b1", "Beta", "b.com")];
        assert!(select_entry(&two, None, "", &opts(false)).is_err());
    }

    #[test]
    fn select_entry_errors_on_empty_list() {
        assert!(select_entry(&[], Some("a1"), "", &opts(false)).is_err());
        assert!(select_entry(&[], None, "", &opts(true)).is_err());
    }

    #[test]
    fn filter_entries_filters_by_user_and_host() {
        let mut a = entry("a1", "Alpha", "a.com");
        a.user = Some("bob".into());
        let b = entry("b1", "Beta", "b.com");
        let entries = vec![a, b];

        assert_eq!(filter_entries(entries.clone(), Some("bob"), None).len(), 1);
        assert_eq!(
            filter_entries(entries.clone(), None, Some("b.com")).len(),
            1
        );
        assert_eq!(filter_entries(entries, Some("nobody"), None).len(), 0);
    }

    #[test]
    fn ssh_common_args_adds_batchmode_only_when_not_interactive() {
        let interactive = ssh_common_args(&opts(true));
        assert!(!interactive.contains(&"BatchMode=yes".to_string()));

        let non_interactive = ssh_common_args(&opts(false));
        assert!(non_interactive.contains(&"BatchMode=yes".to_string()));
        assert!(non_interactive.contains(&"StrictHostKeyChecking=accept-new".to_string()));
    }

    #[test]
    fn ssh_common_args_includes_connect_timeout_when_set() {
        let mut o = opts(false);
        o.timeout = Some(5);
        let args = ssh_common_args(&o);
        assert!(args.contains(&"ConnectTimeout=5".to_string()));
    }

    #[test]
    fn push_connection_args_includes_key_jump_and_nonstandard_port() {
        let mut e = entry("a1", "Alpha", "a.com");
        e.private_key = Some("/k".into());
        e.jump_host = Some("bastion@j.com".into());
        e.port = Some(crate::engine::models::vps::FlexibleValue::Number(2222));

        let mut args = Vec::new();
        push_connection_args(&mut args, &e, "-p");
        assert_eq!(
            args,
            vec!["-i", "/k", "-J", "bastion@j.com", "-p", "2222"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn push_connection_args_omits_default_port() {
        let e = entry("a1", "Alpha", "a.com");
        let mut args = Vec::new();
        push_connection_args(&mut args, &e, "-p");
        assert!(args.is_empty());
    }

    #[test]
    fn copy_args_builds_push_pull_and_remote() {
        let mut e = entry("a1", "Alpha", "a.com");
        e.user = Some("bob".into());
        let o = opts(false);

        let push = copy_args(&e, "local.txt", "remote.txt", "push", &o);
        assert!(push.ends_with(&["local.txt".to_string(), "bob@a.com:remote.txt".to_string()]));

        let pull = copy_args(&e, "remote.txt", "local.txt", "pull", &o);
        assert!(pull.ends_with(&["bob@a.com:remote.txt".to_string(), "local.txt".to_string()]));
    }

    #[test]
    fn copy_args_between_uses_both_hosts() {
        let mut from = entry("a1", "Alpha", "a.com");
        from.user = Some("bob".into());
        let mut to = entry("b1", "Beta", "b.com");
        to.user = Some("ana".into());

        let args = copy_args_between(&from, &to, "src.txt", "dst.txt", &opts(false));
        assert!(args.contains(&"-3".to_string()));
        assert!(args.contains(&"bob@a.com:src.txt".to_string()));
        assert!(args.contains(&"ana@b.com:dst.txt".to_string()));
    }

    #[test]
    fn report_bulk_result_errors_only_when_something_failed() {
        assert!(report_bulk_result("op", 2, vec![]).is_ok());
        assert!(report_bulk_result("op", 2, vec!["a1".into()]).is_err());
    }

    #[test]
    fn resolve_script_content_prefers_inline_then_file_then_config() {
        let mut e = entry("a1", "Alpha", "a.com");
        e.post_connect_script = Some("fallback".into());

        let inline = Some("echo hi".to_string());
        assert_eq!(
            resolve_script_content(&inline, &None, &e).unwrap(),
            "echo hi"
        );

        assert_eq!(
            resolve_script_content(&None, &None, &e).unwrap(),
            "fallback"
        );

        let mut no_fallback = entry("b1", "Beta", "b.com");
        no_fallback.post_connect_script = None;
        assert!(resolve_script_content(&None, &None, &no_fallback).is_err());
    }

    #[test]
    fn parse_and_roundtrip_yaml_and_json() {
        let entries = vec![entry("a1", "Alpha", "a.com")];
        let yaml_path = std::path::PathBuf::from("vps.yaml");
        let yaml = serialize_entries(&entries, &yaml_path).unwrap();
        let back = parse_vps_str(&yaml, &yaml_path).unwrap();
        assert_eq!(back.len(), 1);
        assert_eq!(back[0].id.as_deref(), Some("a1"));

        let json_path = std::path::PathBuf::from("vps.json");
        let json = serialize_entries(&entries, &json_path).unwrap();
        let back = parse_vps_str(&json, &json_path).unwrap();
        assert_eq!(back.len(), 1);
    }

    #[test]
    fn parse_vps_str_accepts_wrapped_and_bare_yaml() {
        let path = std::path::PathBuf::from("vps.yaml");
        let bare = "- id: a1\n  name: Alpha\n  host: a.com\n";
        assert_eq!(parse_vps_str(bare, &path).unwrap().len(), 1);

        let wrapped = "vps:\n  - id: a1\n    name: Alpha\n    host: a.com\n";
        assert_eq!(parse_vps_str(wrapped, &path).unwrap().len(), 1);
    }

    #[test]
    fn check_entries_rejects_duplicate_ids() {
        let entries = vec![entry("a1", "Alpha", "a.com"), entry("a1", "Beta", "b.com")];
        assert!(check_entries(&entries).is_err());
    }

    #[test]
    fn assign_missing_ids_is_idempotent_and_unique() {
        let mut entries = vec![
            VpsEntry {
                id: None,
                name: Some("Temp Server".into()),
                host: "a.com".into(),
                user: None,
                port: None,
                private_key: None,
                post_connect_script: None,
                jump_host: None,
            },
            VpsEntry {
                id: None,
                name: Some("Temp  Server!".into()),
                host: "b.com".into(),
                user: None,
                port: None,
                private_key: None,
                post_connect_script: None,
                jump_host: None,
            },
        ];
        assign_missing_ids(&mut entries);
        assert_eq!(entries[0].id.as_deref(), Some("tempserver"));
        assert_eq!(entries[1].id.as_deref(), Some("tempserver2"));

        // Running it again must not change already-assigned ids.
        let before: Vec<_> = entries.iter().map(|e| e.id.clone()).collect();
        assign_missing_ids(&mut entries);
        let after: Vec<_> = entries.iter().map(|e| e.id.clone()).collect();
        assert_eq!(before, after);
    }
}
