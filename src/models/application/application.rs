// Module: Model/Application
// Location: cli/src/model/application.rs
use crate::{
    config::TranquilityConfig,
    core::shell::command::{command_exists, run_shell_command},
    log_error,
    models::{
        category::Category,
        package_manager::PackageManager,
        system::{OsSupport, SystemInfo, SystemSupport},
    },
    print_error, print_info,
};
use colored::Colorize;
use dialoguer::Confirm;
use heck::ToKebabCase;
use os_info::Type as OSType;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tabled::settings::Style;
use tabled::{Table, Tabled};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ApplicationList {
    pub applications: Vec<Application>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Application {
    #[serde(default)]
    pub id: Option<String>,
    pub name: String,
    #[serde(default)]
    pub server_compatible: bool,
    #[serde(default)]
    pub categories: Vec<Category>,
    #[serde(default)]
    pub supported_systems: Vec<SystemSupport>,
    pub versions: Vec<ApplicationVersion>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ApplicationVersion {
    pub name: String,
    #[serde(default)]
    pub check_command: Option<String>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    pub install_methods: Vec<InstallMethod>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct InstallMethod {
    #[serde(default)]
    pub fallback: bool,
    #[serde(default)]
    pub os: Vec<crate::models::system::OsTypeWrapper>,
    #[serde(default)]
    pub package_manager: Option<PackageManager>,
    #[serde(default)]
    pub package_name: Option<String>,
    #[serde(default)]
    pub is_cask: Option<bool>,
    #[serde(default)]
    pub steps: Option<InstallSteps>,
}

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

#[derive(Debug, Tabled)]
struct DisplayApp<'a> {
    #[tabled(rename = "Name")]
    name: &'a str,
    #[tabled(rename = "Categories")]
    categories: String,
    #[tabled(rename = "Server")]
    server: bool,
}

impl Application {
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
        Confirm::new()
            .with_prompt(
                format!("Do you want to install: {}?", self.name)
                    .purple()
                    .to_string(),
            )
            .default(true)
            .interact()
            .unwrap_or(false)
    }

    pub fn prompt_uninstall(&self) -> bool {
        Confirm::new()
            .with_prompt(
                format!("Are you sure you want to uninstall: {}?", self.name)
                    .purple()
                    .to_string(),
            )
            .default(true)
            .interact()
            .unwrap_or(false)
    }

    pub fn is_installed(&self) -> bool {
        self.versions
            .first()
            .and_then(|v| v.check_command.as_deref())
            .map_or(false, command_exists)
    }
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
            pm.install(None, pkg, self.is_cask, dry_run);
        } else {
            print_error!("❌ No install steps or valid package manager fallback provided.");
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
            pm.uninstall(None, pkg, dry_run);
        } else {
            print_error!("❌ No uninstall steps or valid package manager fallback provided.");
            log_error!(
                "uninstall",
                "app",
                "No uninstall steps or valid package manager fallback provided."
            );
        }
    }
}

pub fn get_apps() -> ApplicationList {
    let mut apps = default_apps();
    let config = TranquilityConfig::load_once();
    print_info!(
        "📄 Applications file path: {}",
        config.applications_file.display()
    );
    if config.applications_file.exists() {
        if let Ok(data) = std::fs::read_to_string(&config.applications_file) {
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

pub fn list_supported_applications(server_only: bool, category_filter: Vec<Category>) {
    let apps = filter_apps(server_only, category_filter);
    let rows: Vec<DisplayApp> = apps
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
                    fallback: false,
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
                    fallback: false,
                    os: vec![OSType::Linux.into(), OSType::Macos.into()],
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
                    fallback: false,
                    os: vec![OSType::Linux.into(), OSType::Macos.into()],
                    package_manager: Some(PackageManager::Apt),
                    package_name: Some("zsh".to_string()),
                    is_cask: None,
                    steps: None,
                }],
            }],
        ),
    ]
}
