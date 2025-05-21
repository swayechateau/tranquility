// src/system.rs
use crate::{common::command_exists, package_manager::PackageManager};
use bitflags::bitflags;
use colored::Colorize;
use os_info::Type as OSType;
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
        let arch = std::env::consts::ARCH.to_string();
        let os = normalized_os_type(&info);
        // For distro we can pull from os_info’s version type if Linux
        let distro = Some(info.os_type().to_string());

        let sys = System::new_all();

        let cpu_brand = sys.cpus().get(0).map(|cpu| cpu.brand().to_string());
        let cpu_vendor = sys.cpus().get(0).map(|cpu| cpu.vendor_id().to_string());
        let default_package_manager = detect_default_package_manager(&info);
        let available_package_managers = detect_available_package_managers();

        SystemInfo {
            os,
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
        self.distro.as_deref().unwrap_or("Unknown").into()
    }
    pub fn arch_type(&self) -> String {
        self.arch.clone()
    }
    pub fn cpu_brand(&self) -> String {
        self.cpu_brand.as_deref().unwrap_or("Unknown").into()
    }
    pub fn cpu_vendor(&self) -> String {
        self.cpu_vendor.as_deref().unwrap_or("Unknown").into()
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
                .collect::<Vec<&str>>()
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
            format!("{:?}", self.os_type()).white(),
            "Arch:".bold().green(),
            self.arch_type().white(),
            "Distro:".bold().green(),
            self.distro().white(),
            "CPU Vendor:".bold().green(),
            self.cpu_vendor().white(),
            "CPU Brand:".bold().green(),
            self.cpu_brand().white(),
            "Default Package Manager:".bold().green(),
            self.default_package_manager().white(),
            "Available Package Managers:".bold().green(),
            self.available_package_manager().white(),
        )
    }
    pub fn install_additional_pms(&self) {
        // Everyone gets Nix
        PackageManager::Nix.check_install();
        match self.os_type() {
            OSType::Linux => {
                PackageManager::Snap.check_install();
                PackageManager::Flatpak.check_install();
                // If arch based, install yay
                if self.distro().contains("Arch") {
                    PackageManager::Yay.check_install();
                }
            }
            OSType::Macos => {
                PackageManager::Brew.check_install();
            }
            OSType::Windows => {
                PackageManager::Scoop.check_install();
                PackageManager::Choco.check_install();
            }
            _ => {}
        }
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
    /// map each supported system to the bitflags it implies:
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

        OSType::Windows => OSType::Windows,
        OSType::Macos => OSType::Macos,
        other => other, // Fallback (BSD, Solaris, Unknown, etc.)
    }
}

fn detect_default_package_manager(info: &os_info::Info) -> Option<PackageManager> {
    let os_type = info.os_type();

    match os_type {
        OSType::Ubuntu | OSType::Debian => Some(PackageManager::Apt),
        OSType::Fedora => Some(PackageManager::Dnf),
        OSType::Redhat => Some(PackageManager::Yum),
        OSType::Alpine => Some(PackageManager::Apk),
        OSType::Arch | OSType::Manjaro | OSType::EndeavourOS => Some(PackageManager::Pacman),
        OSType::Macos => Some(PackageManager::Brew),
        OSType::Windows => Some(PackageManager::Winget),
        _ => {
            // Fallback: probe what's available
            if command_exists("apt") {
                Some(PackageManager::Apt)
            } else if command_exists("dnf") {
                Some(PackageManager::Dnf)
            } else if command_exists("yum") {
                Some(PackageManager::Yum)
            } else if command_exists("apk") {
                Some(PackageManager::Apk)
            } else if command_exists("pacman") {
                Some(PackageManager::Pacman)
            } else if command_exists("brew") {
                Some(PackageManager::Brew)
            } else if command_exists("winget") {
                Some(PackageManager::Winget)
            } else if command_exists("scoop") {
                Some(PackageManager::Scoop)
            } else if command_exists("choco") {
                Some(PackageManager::Choco)
            } else {
                None
            }
        }
    }
}

fn detect_available_package_managers() -> Vec<PackageManager> {
    use crate::package_manager::PackageManager;

    PackageManager::all()
        .iter()
        .copied()
        .filter(|pm| pm.check_install())
        .collect()
}
