// src/system.rs
use crate::{common::command_exists, package_manager::PackageManager};
use bitflags::bitflags;
use colored::Colorize;
use os_info::{self, Type as OSType};
use serde::Deserialize;
use sysinfo::System;

bitflags! {
    #[derive(Clone, Copy)]
    pub struct OsSupport: u8 {
        const LINUX   = 0b001;
        const WINDOWS = 0b010;
        const MACOS   = 0b100;
    }
}

#[derive(Debug)]
pub struct SystemInfo {
    os: OSType,
    raw_os: os_info::Type,
    arch: String,
    distro: Option<String>,
    cpu_vendor: Option<String>,
    cpu_brand: Option<String>,
    default_package_manager: Option<PackageManager>,
    available_package_managers: Vec<PackageManager>,
}

impl SystemInfo {
    pub fn new() -> Self {
        let info = os_info::get();
        let raw_os = info.os_type();
        let os = normalized_os_type(&info);
        let arch = std::env::consts::ARCH.to_owned();
        let distro = Some(info.os_type().to_string());

        let sys = System::new_all();
        let cpu = sys.cpus().get(0);

        let cpu_brand = cpu.map(|c| c.brand().to_owned());
        let cpu_vendor = cpu.map(|c| c.vendor_id().to_owned());

        let default_package_manager = detect_default_package_manager(&info);
        let available_package_managers = detect_available_package_managers(&os);

        SystemInfo {
            os,
            raw_os,
            arch,
            distro,
            cpu_vendor,
            cpu_brand,
            default_package_manager,
            available_package_managers,
        }
    }

    pub fn os_type(&self) -> OSType {
        self.os
    }

    pub fn distro(&self) -> String {
        self.distro.as_deref().unwrap_or("Unknown").to_string()
    }

    pub fn cpu_brand(&self) -> String {
        self.cpu_brand.as_deref().unwrap_or("Unknown").to_string()
    }

    pub fn cpu_vendor(&self) -> String {
        self.cpu_vendor.as_deref().unwrap_or("Unknown").to_string()
    }

    pub fn default_package_manager(&self) -> String {
        self.default_package_manager
            .map(|pm| pm.name().to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    }

    pub fn available_package_manager(&self) -> String {
        if self.available_package_managers.is_empty() {
            "None".to_string()
        } else {
            self.available_package_managers
                .iter()
                .map(|pm| pm.name())
                .collect::<Vec<_>>()
                .join(", ")
        }
    }

    pub fn to_pretty_string(&self) -> String {
        format!(
            "\n🧠 {} {}\n\
             🖥  {} {}\n\
             🧱  {} {}\n\
             🐧  {} {}\n\
             🏷  {} {}\n\
             🧬  {} {}\n\
             📦  {} {}\n\
             📦  {} {}\n",
            "System Info".bold().underline().cyan(),
            "",
            "OS:".bold().green(),
            format!("{:?}", self.os).white(),
            "Arch:".bold().green(),
            self.arch,
            "Distro:".bold().green(),
            self.distro(),
            "CPU Vendor:".bold().green(),
            self.cpu_vendor(),
            "CPU Brand:".bold().green(),
            self.cpu_brand(),
            "Default Package Manager:".bold().green(),
            self.default_package_manager(),
            "Available Package Managers:".bold().green(),
            self.available_package_manager(),
        )
    }

    pub fn install_additional_pms(&self) {
        // Optional: Trigger installation logic on-demand (could be called manually)
        let _installed = PackageManager::supported_on_os(self.raw_os)
            .into_iter()
            .filter(|pm| pm.check_install())
            .collect::<Vec<_>>();
    }
}

#[derive(Copy, Clone, Debug, clap::ValueEnum, Deserialize)]
pub enum SystemSupport {
    Cross,
    MacLin,
    LinWin,
    WinMac,
    Linux,
    Windows,
    MacOS,
}

impl SystemSupport {
    pub fn flags(self) -> OsSupport {
        match self {
            SystemSupport::Cross => OsSupport::all(),
            SystemSupport::MacLin => OsSupport::MACOS | OsSupport::LINUX,
            SystemSupport::LinWin => OsSupport::LINUX | OsSupport::WINDOWS,
            SystemSupport::WinMac => OsSupport::WINDOWS | OsSupport::MACOS,
            SystemSupport::Linux => OsSupport::LINUX,
            SystemSupport::Windows => OsSupport::WINDOWS,
            SystemSupport::MacOS => OsSupport::MACOS,
        }
    }
}

#[derive(Copy, Clone, Debug, clap::ValueEnum, PartialEq, Eq, Deserialize)]
pub enum DistroSupport {
    Debian,
    Ubuntu,
    Fedora,
    Alpine,
    Redhat,
    Arch,
}

#[derive(Copy, Clone, Debug, clap::ValueEnum, Deserialize)]
pub enum ServerSupport {
    Fedora,
    Ubuntu,
    Debian,
}

fn normalized_os_type(info: &os_info::Info) -> OSType {
    match info.os_type() {
        OSType::Ubuntu
        | OSType::Debian
        | OSType::Fedora
        | OSType::Redhat
        | OSType::Alpine
        | OSType::Arch
        | OSType::Linux
        | OSType::Pop
        | OSType::EndeavourOS
        | OSType::Manjaro => OSType::Linux,
        other => other, // includes Windows, MacOS, BSD, etc.
    }
}

fn detect_default_package_manager(info: &os_info::Info) -> Option<PackageManager> {
    use PackageManager::*;

    match info.os_type() {
        OSType::Ubuntu | OSType::Debian => Some(Apt),
        OSType::Fedora => Some(Dnf),
        OSType::Redhat => Some(Yum),
        OSType::Alpine => Some(Apk),
        OSType::Arch | OSType::Manjaro | OSType::EndeavourOS => Some(Pacman),
        OSType::Macos => Some(Brew),
        OSType::Windows => Some(Winget),
        _ => {
            // Fallback: first one found
            [Apt, Dnf, Yum, Apk, Pacman, Brew, Winget, Scoop, Choco]
                .iter()
                .copied()
                .find(|pm| command_exists(pm.name()))
        }
    }
}

fn detect_available_package_managers(os: &OSType) -> Vec<PackageManager> {
    PackageManager::supported_on_os(*os)
        .into_iter()
        .filter(|pm| pm.check_installed()) // Non-interactive version (no prompting)
        .collect()
}
