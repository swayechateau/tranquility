use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::engine::models::category::Category;
use crate::engine::models::package_manager::{PackageManager, command_exists, run_shell_command};
use crate::engine::models::system::{OsTypeWrapper, SystemSupport};

/// A collection of applications.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename = "applications")]
pub struct ApplicationList {
    pub applications: Vec<Application>,
}

/// An application that can be installed.
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

impl Application {
    /// Create a new application with auto-generated ID if not provided.
    pub fn new(
        id: Option<String>,
        name: String,
        server_compatible: bool,
        categories: Vec<Category>,
        supported_systems: Vec<SystemSupport>,
        versions: Vec<ApplicationVersion>,
    ) -> Self {
        let generated_id = id.or_else(|| Some(heck::ToKebabCase::to_kebab_case(name.as_str())));
        Application {
            id: generated_id,
            name,
            server_compatible,
            categories,
            supported_systems,
            versions,
        }
    }

    /// Check if the application is installed by running its check_command.
    pub fn is_installed(&self) -> bool {
        self.versions
            .first()
            .and_then(|v| v.check_command.as_deref())
            .is_some_and(|cmd| {
                let first_word = cmd.split_whitespace().next().unwrap_or(cmd);
                command_exists(first_word)
            })
    }
}

/// A specific version of an application.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ApplicationVersion {
    pub name: String,
    #[serde(default)]
    pub check_command: Option<String>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    pub install_methods: Vec<InstallMethod>,
}

/// A method to install an application (via package manager, cask, or custom steps).
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct InstallMethod {
    #[serde(default)]
    pub fallback: bool,
    #[serde(default)]
    pub os: Vec<OsTypeWrapper>,
    #[serde(default)]
    pub package_manager: Option<PackageManager>,
    #[serde(default)]
    pub package_name: Option<String>,
    #[serde(default)]
    pub is_cask: Option<bool>,
    #[serde(default)]
    pub steps: Option<InstallSteps>,
}

impl InstallMethod {
    /// Run the install steps for this method.
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

    /// Run the uninstall steps for this method.
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

/// Custom installation steps (pre, install, post, uninstall).
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
