use crate::{
    print_info, print_warn,
    models::application::{filter_apps, Application},
    models::system::SystemInfo,
    core::shell::InstallRunner,
};

pub fn install_apps_command(all: bool, server: bool, dry_run: bool) {
    let apps = filter_apps(server, vec![]);
    install_apps(apps, all, dry_run);
}

fn install_apps(apps: Vec<Application>, auto: bool, dry_run: bool) {
    let system = SystemInfo::new();
    let current_os = system.os_type_raw();

    system.install_additional_pms();

    for app in apps {
        if app.is_installed() {
            print_info!("Skipping {}: already installed", app.name);
            continue;
        }

        if !auto && !app.prompt_install() {
            print_info!("Skipping installation of {}", app.name);
            continue;
        }

        let mut installed = false;

        for version in &app.versions {
            for method in &version.install_methods {
                if method
                    .os
                    .iter()
                    .any(|os| os.equals_ostype(&current_os))
                {
                    let runner = InstallRunner::new(&app, method, dry_run);
                    runner.run_install();
                    installed = true;
                    break;
                }
            }
            if installed {
                break;
            }
        }

        if !installed {
            print_warn!("No valid install method found for {}", app.name);
        }
    }
}

