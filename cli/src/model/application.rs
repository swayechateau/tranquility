// src/models/application.rs
use crate::categories::Category;
use crate::common::command_exists;
use crate::config::TranquilityConfig;
use crate::package_manager::PackageManager;
use crate::system::{OsSupport, SystemInfo, SystemSupport};
use crate::{print_error, print_info};
use heck::{ToKebabCase};
use colored::Colorize;
use dialoguer::Confirm;
use os_info::Type as OSType;
use serde::Deserialize;

use tabled::settings::Style;
use tabled::{Table, Tabled};

/// Top-level application file
#[derive(Debug, Deserialize)]
pub struct ApplicationList {
    pub applications: Vec<Application>,
}

/// A single application entry
#[derive(Debug, Deserialize)]
pub struct Application {
    #[serde(default)]
    pub id: Option<String>, // Optional ID
    pub name: String,
    #[serde(default)]
    pub categories: Vec<Category>,
    #[serde(default)]
    pub server_compatible: bool,
    #[serde(default)]
    pub supported_systems: Vec<SystemSupport>,
    pub versions: Vec<ApplicationVersion>,
}

/// Top-level application file
#[derive(Debug, Deserialize)]
pub struct ApplicationVersion {
    pub name: String,

    #[serde(default)]
    pub check_command: Option<String>,

    #[serde(default)]
    pub dependencies: Vec<String>,

    pub install_methods: Vec<InstallMethod>,
}

/// An install method (OS specific PM, or CLI steps)
#[derive(Debug, Deserialize)]
pub struct InstallMethod {
    #[serde(deserialize_with = "deserialize_os")]
    pub os: Vec<String>,

    #[serde(default)]
    pub package_manager: Option<PackageManager>,

    #[serde(default)]
    pub package_name: Option<String>,

    #[serde(default)]
    pub is_cask: Option<bool>, // macOS-specific

    #[serde(default)]
    pub steps: Option<InstallSteps>,
}

/// CLI steps override block
#[derive(Debug, Deserialize)]
pub struct InstallSteps {
    #[serde(default)]
    pub preinstall_steps: Vec<String>,

    #[serde(default)]
    pub install: Vec<String>,

    #[serde(default)]
    pub postinstall_steps: Vec<String>,

    #[serde(default)]
    pub uninstall: Vec<String>,

    #[serde(default)]
    pub postuninstall_steps: Vec<String>,
}

// --------------------------
// CLI Prompt Helpers
// --------------------------

impl Application {
    /// Get or generate an ID
    pub fn get_id(&self) -> String {
        self.id.clone().unwrap_or_else(|| self.name.to_kebab_case())
    }
    pub fn prompt_install(&self) -> bool {
        let prompt = format!("Do you want to install: {}?", self.name)
            .purple()
            .to_string();
        Confirm::new()
            .with_prompt(prompt)
            .default(true)
            .interact()
            .unwrap_or(false)
    }

    pub fn prompt_uninstall(&self) -> bool {
        let prompt = format!("Are you sure you want to uninstall: {}?", self.name)
            .purple()
            .to_string();
        Confirm::new()
            .with_prompt(prompt)
            .default(true)
            .interact()
            .unwrap_or(false)
    }

    pub fn is_installed(&self) -> bool {
        if let Some(cmd) = self.versions.first().and_then(|v| v.check_command.as_ref()) {
            command_exists(cmd)
        } else if let Some(id) = &self.id {
            command_exists(id)
        } else {
            false
        }
    }
}

// --------------------------
// Flexible OS field
// --------------------------

use serde::{
    de::{Error, SeqAccess, Visitor},
    Deserializer,
};
use std::fmt;

/// Allows "os": "Ubuntu" or "os": ["Ubuntu", "Debian"]
fn deserialize_os<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct OsVisitor;

    impl<'de> Visitor<'de> for OsVisitor {
        type Value = Vec<String>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "a string or a list of strings")
        }

        fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
            Ok(vec![v.to_string()])
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut items = vec![];
            while let Some(item) = seq.next_element::<String>()? {
                items.push(item);
            }
            Ok(items)
        }
    }

    deserializer.deserialize_any(OsVisitor)
}

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
            versions: vec![],
        },
        Application {
            id: Some("zsh-shell".to_string()),
            name: "ZSH (shell)".to_string(),
            categories: vec![Category::Shells],
            supported_systems: vec![SystemSupport::MacLin],
            server_compatible: true,
            versions: vec![],
        },
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
            let os_match = app
                .supported_systems
                .iter()
                .any(|s| s.flags().contains(os_flag));
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

use crate::common::run_shell_command;

impl InstallMethod {
    pub fn install(&self, dry_run: bool) {
        if let Some(steps) = &self.steps {
            for cmd in &steps.preinstall_steps {
                run_shell_command(cmd);
            }

            for cmd in &steps.install {
                run_shell_command(cmd);
            }

            for cmd in &steps.postinstall_steps {
                run_shell_command(cmd);
            }
        } else if let (Some(pm), Some(pkg)) = (self.package_manager, self.package_name.as_deref()) {
            pm.install(pkg, self.is_cask, dry_run);
        } else {
            eprintln!("❌ No install steps or valid package manager fallback provided.");
        }
    }

    pub fn uninstall(&self, dry_run: bool) {
        if let Some(steps) = &self.steps {
            for cmd in &steps.uninstall {
                run_shell_command(cmd);
            }

            for cmd in &steps.postuninstall_steps {
                run_shell_command(cmd);
            }
        } else if let (Some(pm), Some(pkg)) = (self.package_manager, self.package_name.as_deref()) {
            pm.uninstall(pkg, dry_run);
        } else {
            eprintln!("❌ No uninstall steps or valid package manager fallback provided.");
        }
    }
}
