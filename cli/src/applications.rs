// src/applications.rs
use crate::categories::{Category};
use crate::config::TranquilityConfig;
use crate::{print_error, print_info};
use crate::system::{OsSupport, SystemInfo, SystemSupport};
use crate::models::{Application,ApplicationList};
use os_info::Type as OSType;

use tabled::settings::Style;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct DisplayApp<'a> {
    name: &'a str,
    categories: String,
    server: bool,
}

/// Gets a list of predefined applications and checks application.json if exists adds to the list and returns
pub fn get_apps() -> ApplicationList {
     // Static built-in apps
    let mut apps: Vec<Application> = vec![
        Application {
            id: "nerd-fonts".into(),
            name: "Nerd Fonts".into(),
            categories: vec![Category::Fonts, Category::Customization],
            supported_os: vec![SystemSupport::Cross],
            supported_distros: vec![],
            server_compatible: true,
            dependencies: vec![],
            install_methods: vec![]
        }
    ];

    if let Ok(config) = TranquilityConfig::load_or_init() {
        if config.applications_file.exists() {
            if let Ok(data) = std::fs::read_to_string(&config.applications_file) {
                if let Ok(user_apps) = serde_json::from_str::<ApplicationList>(&data) {
                    apps.extend(user_apps.applications);
                }
            }
        }
    }

    ApplicationList { applications: apps }
}

pub fn filter_apps(server_only: bool, categories: Vec<Category>) -> Vec<Application> {
    let apps = get_apps();
    let system = SystemInfo::new();
    let os_flag = match system.os_type() {
        OSType::Linux => OsSupport::LINUX,
        OSType::Windows => OsSupport::WINDOWS,
        OSType::Macos => OsSupport::MACOS,
        _ => {
            print_error!("Unsupported OS: {:?}", system.os_type());
            return vec![];
        }
    };

    apps.applications
        .into_iter()
        .filter(|app| {
            let os_match = app.supported_os.iter().any(|s| s.flags().contains(os_flag));
            let server_match = !server_only || app.server_compatible;
            let matches_category = if categories.is_empty() {
                true
            } else {
                app.categories.iter().any(|c| categories.contains(c))
            };
            os_match && server_match && matches_category
        })
        .collect()
}

/// Dynamically loaded list, not static!
pub fn list_supported_applications(
    server_only: bool,
    category_filter: Vec<Category>,
) {
    let filtered_apps = filter_apps(server_only, category_filter.clone());
    let system = SystemInfo::new();

    if !category_filter.is_empty() {
        let joined = category_filter
            .iter()
            .map(|c| format!("{:?}", c))
            .collect::<Vec<_>>()
            .join(", ");
        print_info!(
            "{} applications supported on {:?} in categories: {}",
            if server_only { "Server" } else { "All" },
            system.os_type(),
            joined
        );
    } else {
        print_info!(
            "{} applications supported on {:?}",
            if server_only { "Server" } else { "All" },
            system.os_type()
        );
    }

    let rows: Vec<DisplayApp> = filtered_apps
        .iter()
        .map(|app| DisplayApp {
            name: &app.name,
            categories: app
                .categories
                .iter()
                .map(|c| format!("{:?}", c))
                .collect::<Vec<_>>()
                .join(", "),
            server: app.server_compatible,
        })
        .collect();

    let mut table = Table::new(rows);
    table.with(Style::modern_rounded());
    println!("{}", table);
}
