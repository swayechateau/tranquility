// src/models/application.rs
use crate::categories::Category;
use crate::config::TranquilityConfig;
use crate::package_manager::PackageManager;
use crate::shell::command::{command_exists, run_shell_command};
use crate::system::{OsSupport, SystemInfo, SystemSupport};
use crate::{print_error, print_info};
use heck::{ToKebabCase};
use colored::Colorize;
use dialoguer::Confirm;
use os_info::Type as OSType;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use tabled::settings::Style;
use tabled::{Table, Tabled};

/// Top-level application file
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ApplicationList {
    pub applications: Vec<Application>,
}

/// A single application entry
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Application {
    #[serde(default)]
    pub id: Option<String>, // Optional ID
    pub name: String,
    #[serde(default)]
    pub server_compatible: bool,
    #[serde(default)]
    pub categories: Vec<Category>,
    #[serde(default)]
    pub supported_systems: Vec<SystemSupport>,
    pub versions: Vec<ApplicationVersion>,
}

/// Top-level application file
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ApplicationVersion {
    pub name: String,

    #[serde(default)]
    pub check_command: Option<String>,

    #[serde(default)]
    pub dependencies: Vec<String>,

    pub install_methods: Vec<InstallMethod>,
}

/// An install method (OS specific PM, or CLI steps)
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct InstallMethod {
    #[serde(default)]
    pub os: Vec<OsTypeWrapper>,

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
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
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
    /// Constructs a new Application and generates an ID if not provided
    pub fn new(
        id: Option<String>,
        name: String,
        server_compatible: bool,
        categories: Vec<Category>,
        supported_systems: Vec<SystemSupport>,
        versions: Vec<ApplicationVersion>,
    ) -> Self {
        let generated_id = id.or_else(|| Some(name.to_kebab_case()));

        Application {
            id: generated_id,
            name,
            server_compatible,
            categories,
            supported_systems,
            versions,
        }
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
use super::system::OsTypeWrapper;

#[derive(Tabled)]
struct DisplayApp<'a> {
    name: &'a str,
    categories: String,
    server: bool,
}

/// Gets a list of predefined applications and checks application.json if exists adds to the list and returns
pub fn get_apps() -> ApplicationList {
    // Static built-in apps
    let mut apps= default_apps();

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


fn default_apps() -> Vec<Application> {
    vec![
        Application::new(
            None, // id
            "Alacritty".to_string(),
            false, // server_compatible
            vec![Category::TerminalEmulators],
            vec![SystemSupport::MacLin],
            vec![ApplicationVersion {
                name: "Latest".to_string(),
                check_command: Some("alacritty --version".to_string()),
                dependencies: vec!["cmake".to_string()],
                install_methods: vec![InstallMethod {
                    os: vec![OSType::Ubuntu.into(), OSType::Debian.into()],
                    package_manager: Some(PackageManager::Apt),
                    package_name: Some("alacritty".to_string()),
                    is_cask: None,
                    steps: None,
                }],
            }],
        ),
        Application::new(
            Some("fish-shell".to_string()),
            "Fish Shell".to_string(),
            true,
            vec![Category::Shells],
            vec![SystemSupport::MacLin],
            vec![ApplicationVersion {
                name: "Default".to_string(),
                check_command: Some("fish --version".to_string()),
                dependencies: vec![],
                install_methods: vec![InstallMethod {
                    os: vec![
                        OSType::Linux.into(), 
                        OSType::Macos.into()
                    ],
                    package_manager: Some(PackageManager::Apt),
                    package_name: Some("fish".to_string()),
                    is_cask: None,
                    steps: None,
                }],
            }],
        ),
        Application::new(
            Some("zsh-shell".to_string()),
            "ZSH Shell".to_string(),
            true,
            vec![Category::Shells],
            vec![SystemSupport::MacLin],
            vec![ApplicationVersion {
                name: "Default".to_string(),
                check_command: Some("zsh --version".to_string()),
                dependencies: vec![],
                install_methods: vec![InstallMethod {
                    os: vec![
                        OSType::Linux.into(), 
                        OSType::Macos.into()
                    ],
                    package_manager: Some(PackageManager::Apt),
                    package_name: Some("zsh".to_string()),
                    is_cask: None,
                    steps: None,
                }],
            }],
        ),
    ]
}