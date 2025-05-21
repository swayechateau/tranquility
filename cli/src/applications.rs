// src/applications.rs
use crate::categories::Category;
use crate::config::TranquilityConfig;
use crate::model::application::{Application, ApplicationList};
use crate::system::{OsSupport, SystemInfo, SystemSupport};
use crate::{print_error, print_info};
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
            id: Some("nerd-fonts".to_string()),
            name: "Nerd Fonts".to_string(),
            categories: vec![Category::Fonts, Category::Customization],
            supported_systems: vec![SystemSupport::Cross],
            server_compatible: true,
            versions: vec![]
        },
        Application {
            id: Some("zsh-shell".to_string()),
            name: "ZSH (shell)".to_string(),
            categories: vec![Category::Shells],
            supported_systems: vec![SystemSupport::MacLin],
            server_compatible: true,
            versions: vec![]
        }
    ];

    if let Ok(config) = TranquilityConfig::load_or_init() {
        println!(
            "📄 Applications file path: {}",
            config.applications_file.display()
        );
        if config.applications_file.exists() {
            print_info!(
                "📄 Loading applications from {}",
                config.applications_file.display()
            );
            if let Ok(data) = std::fs::read_to_string(&config.applications_file) {
                print_info!(
                    "📄 Parsing applications from {}",
                    config.applications_file.display()
                );
                match serde_json::from_str::<ApplicationList>(&data) {
                    Ok(user_apps) => {
                        print_info!(
                            "📄 Loaded {} applications from {}",
                            user_apps.applications.len(),
                            config.applications_file.display()
                        );
                        apps.extend(user_apps.applications);
                    }
                    Err(e) => {
                        print_error!("❌ Failed to parse applications.json: {e}");
                    }
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
            let os_match = app.supported_systems.iter().any(|s| s.flags().contains(os_flag));
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
pub fn list_supported_applications(server_only: bool, category_filter: Vec<Category>) {
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
