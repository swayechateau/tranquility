// src/models.rs
use std::path::PathBuf;
use colored::Colorize;
use dialoguer::Confirm;
use serde::Deserialize;
use crate::categories::Category;
use crate::common::command_exists;
use crate::package_manager::PackageManager;
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

impl Application {
    pub fn is_installed(&self) -> bool {
        command_exists(&self.id)
    }

    pub fn prompt_install(&self) -> bool {
        if self.is_installed() {
            return false;
        }
        
        let prompt = format!("Do you want to install: {}?", self.name).purple().to_string();
        Confirm::new()
        .with_prompt(&prompt)
        .default(true)
        .interact()
        .unwrap()
    }

    pub fn prompt_uninstall(&self) -> bool {
        if !self.is_installed() {
            return false;
        }
        
        let prompt = format!("Are you sure you want to uninstall: {}?", self.name).purple().to_string();
        Confirm::new()
        .with_prompt(&prompt)
        .default(true)
        .interact()
        .unwrap()
    }

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
    pub package_manager: Option<PackageManager>,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub steps: Vec<String>,
    #[serde(default)]
    pub conditions: Option<InstallConditions>,
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

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct VPSConfig {
    pub name: Option<String>,
    pub username: Option<String>,
    pub host: String,
    pub port: Option<String>,
    pub private_key: Option<PathBuf>,
}
