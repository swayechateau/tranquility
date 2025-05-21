use crate::{
    applications::filter_apps,
    common::run_shell_command,
    models::{Application},
    print_info,
    print_warn,
    system::SystemInfo,
};

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
        if app.is_installed() {
            print_info!("Skipping {}: Already installed", app.name);
            continue;
        }

        if auto.is_some() {
            print_info!("Auto mode enabled: Skipping user input");
        } else if !app.prompt_install() {
            print_info!("Skipping installation of {}", app.name);
            continue;
        }

        print_info!("Installing {}", app.name);
        let mut installed = false;

        for method in &app.install_methods {
            if let Some(block) = method.get_validated_block(&current_os, &current_distro) {
                // Pre-install
                for step in &block.preinstall_steps {
                    run_shell_command(step);
                }

                // If package manager + command = install via PM
                if let Some(pm) = block.package_manager {
                    if let Some(cmd) = &block.command {
                        if pm.check_install() {
                            print_info!(
                                "Installing {} using package manager: {}",
                                app.name,
                                pm.name()
                            );
                            pm.install(cmd, None);
                            installed = true;
                        } else {
                            print_warn!(
                                "Skipping {}: Required package manager '{}' is not available",
                                app.name,
                                pm.name()
                            );
                        }
                    }
                }
                // Else fallback to shell command
                else if let Some(cmd) = &block.command {
                    print_info!("Installing {} via custom shell command", app.name);
                    run_shell_command(cmd);
                    installed = true;
                }

                // Run additional steps
                for step in &block.steps {
                    run_shell_command(step);
                    installed = true;
                }

                for step in &block.postinstall_steps {
                    run_shell_command(step);
                }

                if installed {
                    break;
                }
            }
        }

        if !installed {
            print_warn!("No valid install method worked for {}", app.name);
        }
    }
}

fn uninstall_apps(apps: Vec<Application>, auto: Option<bool>) {
    let system = SystemInfo::new();
    let current_os = system.os_type().to_string();
    let current_distro = system.distro();

    for app in apps {
        if !app.is_installed() {
            print_info!("Skipping {}: is not installed.", app.name);
            continue;
        }

        if auto.is_some() {
            print_info!("Auto mode enabled: Skipping user input");
        } else if !app.prompt_uninstall() {
            print_info!("Skipping uninstall of {}", app.name);
            continue;
        }

        print_info!("Uninstalling {}", app.name);
        let mut uninstalled = false;

        for method in &app.install_methods {
            if let Some((block, uninstall)) =
                method.get_validated_uninstall_block(&current_os, &current_distro)
            {
                if let Some(pm) = block.package_manager {
                    if pm.check_install() {
                        pm.uninstall(&app.id);
                        uninstalled = true;
                        break;
                    } else {
                        print_warn!(
                            "Package manager {} not available for uninstalling {}",
                            pm.name(),
                            app.name
                        );
                        continue;
                    }
                }

                if let Some(cmd) = &uninstall.command {
                    run_shell_command(cmd);
                    uninstalled = true;
                }

                for step in &uninstall.steps {
                    run_shell_command(step);
                    uninstalled = true;
                }

                if uninstalled {
                    break;
                }
            }
        }

        if !uninstalled {
            print_warn!("No uninstall method worked for {}", app.name);
        }
    }
}
