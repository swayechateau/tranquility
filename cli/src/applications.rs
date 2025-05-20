use std::fs;
use std::path::Path;

use crate::categories::Category;
use crate::print::{print_error, print_info};
use crate::system::{DistroSupport, OsSupport, SystemInfo, SystemSupport};
use os_info::Type as OSType;
use serde::Deserialize;
use tabled::grid::records::vec_records::VecRecords;
use tabled::settings::Style;
use tabled::{Table, Tabled};

#[derive(Debug, Deserialize)]
pub struct ApplicationList {
    pub applications: Vec<Application>,
}

#[derive(Debug, Deserialize)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub categories: Vec<Category>,
    #[serde(rename = "supported_os")]
    pub supported_os: Vec<SystemSupport>,
    #[serde(rename = "supported_distros")]
    pub supported_distros: Vec<DistroSupport>,
    pub server_compatible: bool,
    #[serde(default)]
    pub dependencies: Vec<String>,
    pub install_methods: Vec<InstallMethod>,
}

#[derive(Tabled)]
pub struct DisplayApp<'a> {
    pub Name: &'a str,
    pub Categories: String,
    pub Server: bool,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum InstallMethod {
    #[serde(rename = "shell_script")]
    ShellScript(InstallBlock),
    #[serde(rename = "package_manager")]
    PackageManager(InstallBlock),
}

#[derive(Debug, Deserialize)]
pub struct InstallBlock {
    pub package_manager: String,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub steps: Vec<String>,
    #[serde(default)]
    pub conditions: Option<InstallConditions>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub uninstall: Option<Uninstall>,
}

#[derive(Debug, Deserialize)]
pub struct InstallConditions {
    #[serde(default, rename = "os")]
    pub os: Vec<String>,
    #[serde(default, rename = "distros")]
    pub distros: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Uninstall {
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub steps: Vec<String>,
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

    let json_path = Path::new("application.json");
    if json_path.exists() {
        match fs::read_to_string(json_path) {
            Ok(data) => match serde_json::from_str::<ApplicationList>(&data) {
                Ok(external) => {
                    apps.extend(external.applications);
                }
                Err(e) => {
                    print_error(format!("❌ Failed to parse application.json: {e}"));
                }
            },
            Err(e) => {
                print_error(format!("❌ Could not read application.json: {e}"));
            }
        }
    }

    ApplicationList { applications: apps }
}

/// Dynamically loaded list, not static!
pub fn list_supported_applications(
    server_only: bool,
    category_filter: Vec<Category>,
) {
    let apps = get_apps();
    let system = SystemInfo::new();
    let os_flag = match system.os_type() {
        OSType::Linux => OsSupport::LINUX,
        OSType::Windows => OsSupport::WINDOWS,
        OSType::Macos => OsSupport::MACOS,
        _ => {
            print_error(format!("Unsupported OS: {:?}", system.os_type()));
            return;
        }
    };

    if !category_filter.is_empty() {
        let joined = category_filter
            .iter()
            .map(|c| format!("{:?}", c))
            .collect::<Vec<_>>()
            .join(", ");
        print_info(format!(
            "{} applications supported on {:?} in categories: {}",
            if server_only { "Server" } else { "All" },
            system.os_type(),
            joined
        ));
    } else {
        print_info(format!(
            "{} applications supported on {:?}",
            if server_only { "Server" } else { "All" },
            system.os_type()
        ));
    }

    let mut rows = vec![];

    for app in apps.applications.iter() {
        let os_match = app.supported_os.iter().any(|s| s.flags().contains(os_flag));

        let is_server = app.server_compatible;

        let matches_category = if category_filter.is_empty() {
            true
        } else {
            app.categories.iter().any(|c| category_filter.contains(c))
        };

        if os_match && (!server_only || is_server) && matches_category {
            rows.push(DisplayApp {
                Name: &app.name,
                Categories: app
                    .categories
                    .iter()
                    .map(|c| format!("{:?}", c))
                    .collect::<Vec<_>>()
                    .join(", "),
                Server: is_server,
            });
        }
    }

    let mut table = Table::new(rows);
    table.with(Style::modern_rounded());
    println!("{}", table);
}
