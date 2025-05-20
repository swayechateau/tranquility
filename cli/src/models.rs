// src/models.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApplicationConfig {
    pub applications: Vec<Application>,
}

#[derive(Debug, Deserialize)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub categories: Vec<String>,
    pub supported_operating_systems: Vec<String>,
    pub supported_distributions: Vec<String>,
    pub server_compatible: bool,
    #[serde(default)]
    pub dependencies: Vec<String>,
    pub install_methods: Vec<InstallMethod>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallMethodType {
    ShellScript,
    PackageManager,
}

#[derive(Debug, Deserialize)]
pub struct InstallMethod {
    #[serde(rename = "type")]
    pub method_type: InstallMethodType,
    pub package_manager: String,
    pub command: Option<String>,
    pub steps: Option<Vec<String>>,
    pub conditions: Option<Conditions>,
    pub notes: Option<String>,
    pub uninstall: Option<UninstallBlock>,
}

#[derive(Debug, Deserialize)]
pub struct Conditions {
    pub operating_systems: Option<Vec<String>>,
    pub distributions: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UninstallBlock {
    pub command: Option<String>,
    pub steps: Option<Vec<String>>,
}
