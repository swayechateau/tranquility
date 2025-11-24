use crate::{
    core::shell::InstallRunner,
    models::{
        application::{Application, filter_apps},
        system::SystemInfo,
    },
    print_info, print_warn,
};

pub fn uninstall_apps_command(all: bool, server: bool, dry_run: bool) {
    let apps = filter_apps(server, vec![]);
    uninstall_apps(apps, all, dry_run);
}

fn uninstall_apps(apps: Vec<Application>, auto: bool, dry_run: bool) {
    let system = SystemInfo::new();
    let current_os = system.os_type_raw();

    for app in apps {
        if !app.is_installed() {
            print_info!("Skipping {}: not installed", app.name);
            continue;
        }

        if !auto && !app.prompt_uninstall() {
            print_info!("Skipping uninstall of {}", app.name);
            continue;
        }

        let mut uninstalled = false;

        for version in &app.versions {
            for method in &version.install_methods {
                if method.os.iter().any(|os| os.equals_ostype(&current_os)) {
                    let runner = InstallRunner::new(&app, method, dry_run);
                    runner.run_uninstall();
                    uninstalled = true;
                    break;
                }
            }
            if uninstalled {
                break;
            }
        }

        if !uninstalled {
            print_warn!("No valid uninstall method found for {}", app.name);
        }
    }
}
