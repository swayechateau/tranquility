use crate::{
    core::{Context, CoreResult},
    engine::{
        config::{BaseConfig, deserialize_config},
        models::{
            package_manager::{PackageManager, command_exists},
            system::SystemInfo,
        },
    },
};

pub async fn run(ctx: &Context) -> CoreResult<()> {
    let ui = ctx.ui();
    let mut output = ui.new_output_content();

    // --- Config ---
    let config_status = match ctx.config_load_error() {
        None => "Config loaded".to_string(),
        Some(err) => format!("Config invalid, using in-memory defaults: {err}"),
    };

    output = output.heading(2, "Configuration");
    output = output.paragraph(&config_status);

    if let Some(path) = ctx.project_config_path() {
        let status = if path.exists() {
            format!("Project config exists: {}", path.display())
        } else {
            format!("Project config not found: {}", path.display())
        };
        output = output.paragraph(&status);
    } else {
        output = output.paragraph("No project config found in current directory");
    }

    // --- Data dirs ---
    output = output.heading(2, "Data Directories");
    let log_dir = ctx.base_config().log_directory.clone();

    let dir_status = if log_dir.exists() {
        format!("Log directory exists: {}", log_dir.display())
    } else {
        format!("Log directory would be: {}", log_dir.display())
    };
    output = output.paragraph(&dir_status);

    // --- System info ---
    output = output.heading(2, "System");
    let sys = SystemInfo::new();
    output = output
        .key_value("OS", format!("{:?}", sys.os_type()))
        .key_value("Arch", &sys.arch)
        .key_value("Distro", sys.distro())
        .key_value("CPU Vendor", sys.cpu_vendor())
        .key_value("CPU Brand", sys.cpu_brand());

    // --- Package managers ---
    output = output.heading(2, "Package Managers");
    let pms = PackageManager::supported_on_os(sys.os_type_raw());
    if pms.is_empty() {
        output = output.paragraph("No known package managers for this OS");
    } else {
        let mut pm_list = Vec::new();
        for pm in pms {
            if pm.is_available() {
                pm_list.push(format!("{} available", pm.name()));
            } else {
                pm_list.push(format!("{} (not in PATH)", pm.name()));
            }
        }
        output = output.list(false, pm_list);
    }

    // --- Common tools ---
    output = output.heading(2, "Common Tools");
    let tools = ["git", "curl", "ssh", "unzip", "tar"];
    let mut tool_list = Vec::new();
    for tool in tools {
        if command_exists(tool) {
            tool_list.push(format!("{} available", tool));
        } else {
            tool_list.push(format!("{} (not found)", tool));
        }
    }
    output = output.list(false, tool_list);

    ui.print(&output)
}

pub async fn fix(ctx: &Context) -> CoreResult<()> {
    let ui = ctx.ui();
    let mut output = ui.new_output_content();

    if ctx.dry_run() {
        output = output.paragraph("Dry run — no changes will be made.");
    }

    output = output.heading(2, "Fixing Issues");

    // Ensure log directory exists
    let log_dir = ctx.base_config().log_directory.clone();

    if log_dir.exists() {
        output = output.status(
            scriba::StatusKind::Ok,
            format!("Log directory already exists: {}", log_dir.display()),
        );
    } else if ctx.dry_run() {
        output = output.status(
            scriba::StatusKind::Warning,
            format!(
                "[dry run] Would create log directory: {}",
                log_dir.display()
            ),
        );
    } else {
        match std::fs::create_dir_all(&log_dir) {
            Ok(_) => {
                output = output.status(
                    scriba::StatusKind::Ok,
                    format!("Created log directory: {}", log_dir.display()),
                );
            }
            Err(e) => {
                output = output.status(
                    scriba::StatusKind::Warning,
                    format!("Failed to create log directory: {}", e),
                );
            }
        }
    }

    // Ensure the project config exists, is parseable, and has every field.
    let config_path = match ctx.project_config_path() {
        Some(path) => path,
        None => BaseConfig::config_path().map_err(|e| {
            crate::engine::Error::from_code(crate::engine::ErrorCode::IoFailure)
                .with_context("error", e.to_string())
        })?,
    };
    let (kind, message) = fix_config_status(&config_path, ctx.dry_run())?;
    output = output.status(kind, message);

    ui.print(&output)
}

/// Diagnose/fix the config file at `path`, returning the status to report.
/// In dry-run mode this never writes, only reports what would happen.
fn fix_config_status(
    path: &std::path::Path,
    dry_run: bool,
) -> CoreResult<(scriba::StatusKind, String)> {
    if !path.exists() {
        return Ok(if dry_run {
            (
                scriba::StatusKind::Warning,
                format!("[dry run] Would create default config: {}", path.display()),
            )
        } else {
            BaseConfig::fix_at(path).map_err(|e| {
                crate::engine::Error::from_code(crate::engine::ErrorCode::IoFailure)
                    .with_context("error", e.to_string())
            })?;
            (
                scriba::StatusKind::Ok,
                format!("Created default config: {}", path.display()),
            )
        });
    }

    let content = std::fs::read_to_string(path).map_err(|e| {
        crate::engine::Error::from_code(crate::engine::ErrorCode::IoFailure)
            .with_context("path", path.display().to_string())
            .with_context("error", e.to_string())
    })?;

    match deserialize_config(path, &content) {
        Err(e) => Ok(if dry_run {
            (
                scriba::StatusKind::Warning,
                format!(
                    "[dry run] Would recreate invalid config ({e}): {}",
                    path.display()
                ),
            )
        } else {
            BaseConfig::fix_at(path).map_err(|e| {
                crate::engine::Error::from_code(crate::engine::ErrorCode::IoFailure)
                    .with_context("error", e.to_string())
            })?;
            (
                scriba::StatusKind::Ok,
                format!("Recreated invalid config: {}", path.display()),
            )
        }),
        Ok(cfg) => {
            let incomplete = cfg.log_directory.as_os_str().is_empty()
                || cfg.applications_file.as_os_str().is_empty()
                || cfg.vps_file.as_os_str().is_empty();

            if !incomplete {
                return Ok((
                    scriba::StatusKind::Ok,
                    format!("Config is complete: {}", path.display()),
                ));
            }

            Ok(if dry_run {
                (
                    scriba::StatusKind::Warning,
                    format!(
                        "[dry run] Would patch missing config fields: {}",
                        path.display()
                    ),
                )
            } else {
                BaseConfig::fix_at(path).map_err(|e| {
                    crate::engine::Error::from_code(crate::engine::ErrorCode::IoFailure)
                        .with_context("error", e.to_string())
                })?;
                (
                    scriba::StatusKind::Ok,
                    format!("Patched missing config fields: {}", path.display()),
                )
            })
        }
    }
}
