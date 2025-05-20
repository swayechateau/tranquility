// src/models.rs
use serde::Deserialize;
use crate::categories::Category;
use crate::system::{DistroSupport, SystemSupport};

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