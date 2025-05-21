// src/installer.rs
use crate::{applications::{filter_apps}, common::run_shell_command, models::{Application, InstallMethod}, print_info, system::SystemInfo};

pub fn install_apps_command(all: bool, server: bool) {
    let apps = filter_apps(server, vec![]);
    install_apps(apps, Some(all));
}

pub fn uninstall_apps_command(all: bool, server: bool) {
    let apps = filter_apps(server, vec![]);

    uninstall_apps(apps, Some(all));
}

fn install_apps(apps: Vec<Application>, auto: Option<bool>) {
    let system = SystemInfo::new();
    let current_os = system.os_type().to_string();
    let current_distro = system.distro();

    system.install_additional_pms();

    for app in apps {
        // Check if the app is already installed
        if app.is_installed() {
            print_info!("Skipping {}: Already installed", app.name);
            continue;
        }
        // If auto is true, skip user input
        if auto.is_some() {
            print_info!("Auto mode enabled: Skipping user input");
        } else {
            // Prompt the user to install the app
            if !app.prompt_install() {
                print_info!("Skipping installation of {}", app.name);
                continue;
            }
        }
        print_info!("Installing {}", app.name);

        for method in &app.install_methods {
            match method {
                InstallMethod::PackageManager(block) | InstallMethod::ShellScript(block) => {
                    // Check conditions (optional filtering)
                    if let Some(conditions) = &block.conditions {
                        if !conditions.os.is_empty()
                            && !conditions.os.iter().any(|os| os == &current_os)
                        {
                            print_warn!("Skipping {}: OS condition mismatch", app.name);
                            continue;
                        }
                        if !conditions.distros.is_empty()
                            && !conditions.distros.iter().any(|d| current_distro.contains(d))
                        {
                            print_warn!("Skipping {}: Distro condition mismatch", app.name);
                            continue;
                        }
                    }

                    // Run single command or step list
                    if let Some(cmd) = &block.command {
                        run_shell_command(cmd);
                    }

                    for step in &block.steps {
                        run_shell_command(step);
                    }
                }
            }
        }
    }
}


fn uninstall_apps(apps: Vec<Application>, auto: Option<bool>) {
    for app in apps {
        // Check if the app is already installed
        if !app.is_installed() {
            print_info!("Skipping {}: is not installed.", app.name);
            continue;
        }
        // If auto is true, skip user input
        if auto.is_some() {
            print_info!("Auto mode enabled: Skipping user input");
        } else {
            // Prompt the user to install the app
            if !app.prompt_uninstall() {
                print_info!("Skipping installation of {}", app.name);
                continue;
            }
        }
        print_info!("Uninstalling {}", app.name);

        for method in &app.install_methods {
            match method {
                InstallMethod::PackageManager(block) | InstallMethod::ShellScript(block) => {
                    if let Some(uninstall) = &block.uninstall {
                        if let Some(cmd) = &uninstall.command {
                            run_shell_command(cmd);
                        }
                        for step in &uninstall.steps {
                            run_shell_command(step);
                        }
                    } else {
                        print_warn!("No uninstall method defined for {}", app.name);
                    }
                }
            }
        }
    }
}
