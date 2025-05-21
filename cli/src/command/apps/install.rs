// src/command/install.rs

use crate::{model::{application::{filter_apps, Application}, system::SystemInfo}, print_info, print_warn, shell::InstallRunner};


pub fn install_apps_command(all: bool, server: bool, dry_run: bool) {
    let apps = filter_apps(server, vec![]);
    install_apps(apps, Some(all), dry_run);
}

fn install_apps(apps: Vec<Application>, auto: Option<bool>, dry_run: bool) {
    let system = SystemInfo::new();
    let current_os = system.os_type().to_string();
    let current_distro = system.distro();

    system.install_additional_pms();

    for app in apps {
        if app.is_installed() {
            print_info!("Skipping {}: already installed", app.name);
            continue;
        }

        if auto.is_some() {
            print_info!("Auto mode enabled: Skipping user input");
        } else if !app.prompt_install() {
            print_info!("Skipping installation of {}", app.name);
            continue;
        }

        let mut installed = false;

        for method in &app.install_methods {
            if let Some(block) = method.get_validated_block(&current_os, &current_distro) {
                let runner = InstallRunner::new(&app, block, dry_run);
                runner.run_install();
                installed = true;
                break;
            }
        }

        if !installed {
            print_warn!("No valid install method worked for {}", app.name);
        }
    }
}
