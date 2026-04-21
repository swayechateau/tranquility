use crate::{
    core::{Context, CoreResult},
    engine::{constants::APP_NAME, models::system::SystemInfo},
};

pub async fn show(ctx: &Context) -> CoreResult<()> {
    let ui = ctx.ui();
    let sys = SystemInfo::new();

    let banner = ui.figlet(APP_NAME).unwrap_or_default();
    let colored = ui.use_color();

    let mut output = ctx
        .ui()
        .new_output_content()
        .line(banner)
        .heading(2, &blue_key("System Info", colored));

    output = output
        .key_value(&green_key("OS", colored), sys.os_type())
        .key_value(&green_key("Arch", colored), &sys.arch)
        .key_value(&green_key("Distro", colored), sys.distro())
        .key_value(&green_key("CPU Vendor", colored), sys.cpu_vendor())
        .key_value(&green_key("CPU Brand", colored), sys.cpu_brand())
        .key_value(&green_key("Default Package Manager", colored), sys.default_package_manager());

    // Show supported package managers for this OS
    let supported: Vec<String> = sys
        .available_package_managers
        .iter()
        .map(|pm| pm.name().to_string())
        .collect();
    if !supported.is_empty() {
        output = output.key_value(&green_key("Supported Package Managers", colored), supported.join(", "));
    }

    // Show which supported package managers are actually installed
    let installed: Vec<String> = sys
        .available_package_managers
        .iter()
        .filter(|pm| pm.is_available())
        .map(|pm| pm.name().to_string())
        .collect();
    if !installed.is_empty() {
        output = output.key_value(&green_key("Installed Package Managers", colored), installed.join(", "));
    } else {
        output = output.key_value(&green_key("Installed Package Managers", colored), "(none)");
    }

    if ctx.dry_run() {
        output = output.key_value(&green_key("Mode", colored), "Dry Run (no changes will be made)");
    }

    output = output.heading(2, &blue_key("Configuration", colored));
    output = match ctx.config() {
        Some(_) => output.status(scriba::StatusKind::Ok, &green_key("Config file is valid", colored)),
        None => output.status(
            scriba::StatusKind::Warning,
            &yellow_key("No config loaded — using defaults", colored),
        ),
    };

    ui.print(&output)
}

fn green_key(key: &str, colored: bool) -> String {
    // use ansi green if supported, otherwise fallback to plain text
    if colored {
        format!("\x1b[32m{}\x1b[0m", key)
    } else {
        key.to_string()
    }
}

fn blue_key(key: &str, colored: bool) -> String {
    if colored {
        format!("\x1b[34m{}\x1b[0m", key)
    } else {
        key.to_string()
    }
}

fn yellow_key(key: &str, colored: bool) -> String {
    if colored {
        format!("\x1b[33m{}\x1b[0m", key)
    } else {
        key.to_string()
    }
}