// src/command/uninstall.rs

use crate::{model::{application::{filter_apps, Application}, system::SystemInfo}, print_info, print_warn, shell::InstallRunner};

pub fn uninstall_apps_command(all: bool, server: bool, dry_run: bool) {
    let apps = filter_apps(server, vec![]);
    uninstall_apps(apps, Some(all), dry_run);
}


fn uninstall_apps(apps: Vec<Application>, auto: Option<bool>, dry_run: bool) {
    let system = SystemInfo::new();
    let current_os = system.os_type().to_string();
    let current_distro = system.distro();

    for app in apps {
        if !app.is_installed() {
            print_info!("Skipping {}: is not installed", app.name);
            continue;
        }

        if auto.is_some() {
            print_info!("Auto mode enabled: Skipping user input");
        } else if !app.prompt_uninstall() {
            print_info!("Skipping uninstall of {}", app.name);
            continue;
        }

        let mut uninstalled = false;

        for method in &app.install_methods {
            if let Some((block, uninstall)) = method.get_validated_uninstall_block(&current_os, &current_distro) {
                let runner = InstallRunner::new(&app, block, dry_run);
                runner.run_uninstall(uninstall);
                uninstalled = true;
                break;
            }
        }

        if !uninstalled {
            print_warn!("No uninstall method worked for {}", app.name);
        }
    }
}