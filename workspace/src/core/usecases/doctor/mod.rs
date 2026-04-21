use crate::{
    core::{Context, CoreResult},
    engine::models::{
        package_manager::{PackageManager, command_exists},
        system::SystemInfo,
    },
};

pub async fn run(ctx: &Context) -> CoreResult<()> {
    let ui = ctx.ui();
    let mut output = ui.new_output_content();

    // --- Config ---
    let config_status = match ctx.config() {
        Some(_) => "Config loaded",
        None => "No config loaded — using defaults",
    };
    
    output = output.heading(2, "Configuration");
    output = output.paragraph(config_status);
    
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
    let log_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("tranquility")
        .join("logs");
    
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
        .key_value("OS", &format!("{:?}", sys.os_type()))
        .key_value("Arch", &sys.arch)
        .key_value("Distro", &sys.distro())
        .key_value("CPU Vendor", &sys.cpu_vendor())
        .key_value("CPU Brand", &sys.cpu_brand());

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
    let log_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("tranquility")
        .join("logs");

    if log_dir.exists() {
        output = output.status(
            scriba::StatusKind::Ok,
            &format!("Log directory already exists: {}", log_dir.display()),
        );
    } else if ctx.dry_run() {
        output = output.status(
            scriba::StatusKind::Warning,
            &format!("[dry run] Would create log directory: {}", log_dir.display()),
        );
    } else {
        match std::fs::create_dir_all(&log_dir) {
            Ok(_) => {
                output = output.status(
                    scriba::StatusKind::Ok,
                    &format!("Created log directory: {}", log_dir.display()),
                );
            }
            Err(e) => {
                output = output.status(
                    scriba::StatusKind::Warning,
                    &format!("Failed to create log directory: {}", e),
                );
            }
        }
    }

    ui.print(&output)
}
