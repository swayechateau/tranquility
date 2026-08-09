use bitflags::bitflags;
use os_info::{self, Type as OSType};
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::engine::models::package_manager::PackageManager;

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct OsSupport: u8 {
        const LINUX   = 0b001;
        const WINDOWS = 0b010;
        const MACOS   = 0b100;
    }
}

/// Wrapper around OSType for serialization.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OsTypeWrapper {
    pub os_type: String,
}

impl OsTypeWrapper {
    pub fn equals_ostype(&self, other: &OSType) -> bool {
        self.os_type.eq_ignore_ascii_case(&other.to_string())
    }
}

impl From<OSType> for OsTypeWrapper {
    fn from(ty: OSType) -> Self {
        OsTypeWrapper {
            os_type: ty.to_string(),
        }
    }
}

impl JsonSchema for OsTypeWrapper {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("OsTypeWrapper")
    }

    fn json_schema(r#gen: &mut SchemaGenerator) -> Schema {
        <String as JsonSchema>::json_schema(r#gen)
    }
}

impl std::fmt::Display for OsTypeWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.os_type)
    }
}

impl std::str::FromStr for OsTypeWrapper {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            os_type: s.to_string(),
        })
    }
}

/// Platform support indicator.
#[derive(Copy, Clone, Debug, clap::ValueEnum, Deserialize, Serialize, JsonSchema)]
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

/// System information collected at runtime.
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os: OSType,
    pub raw_os: OSType,
    pub arch: String,
    pub distro: Option<String>,
    pub cpu_vendor: Option<String>,
    pub cpu_brand: Option<String>,
    pub default_package_manager: Option<PackageManager>,
    pub available_package_managers: Vec<PackageManager>,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemInfo {
    /// Collect system information.
    pub fn new() -> Self {
        let info = os_info::get();
        let raw_os = info.os_type();
        let os = normalize_os_type(&info);
        let arch = std::env::consts::ARCH.to_owned();
        let distro = Some(info.os_type().to_string());

        let (cpu_brand, cpu_vendor) = detect_cpu_info();

        let default_package_manager = detect_default_package_manager(raw_os);
        let available_package_managers = detect_available_package_managers(raw_os);

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

    pub fn os_type_raw(&self) -> OSType {
        self.raw_os
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

    pub fn available_package_managers(&self) -> String {
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
}

fn normalize_os_type(info: &os_info::Info) -> OSType {
    let os = info.os_type();
    match os {
        OSType::Linux => OSType::Linux,
        OSType::Macos => OSType::Macos,
        OSType::Windows => OSType::Windows,
        OSType::Ubuntu => OSType::Linux,
        OSType::Debian => OSType::Linux,
        OSType::Fedora => OSType::Linux,
        OSType::Redhat => OSType::Linux,
        OSType::Alpine => OSType::Linux,
        OSType::Arch => OSType::Linux,
        OSType::SUSE => OSType::Linux,
        OSType::Gentoo => OSType::Linux,
        OSType::Manjaro => OSType::Linux,
        OSType::EndeavourOS => OSType::Linux,
        _ => os,
    }
}

fn detect_default_package_manager(os: OSType) -> Option<PackageManager> {
    match os {
        // Debian-based
        OSType::Ubuntu | OSType::Debian | OSType::Pop => Some(PackageManager::Apt),
        
        // Red Hat-based
        OSType::Fedora => Some(PackageManager::Dnf),
        OSType::Redhat => Some(PackageManager::Yum),
        
        // Independent Linux distros
        OSType::Alpine => Some(PackageManager::Apk),
        OSType::Arch | OSType::Manjaro | OSType::EndeavourOS => Some(PackageManager::Pacman),
        OSType::SUSE | OSType::openSUSE => Some(PackageManager::Zypper),
        OSType::Gentoo => Some(PackageManager::Portage),
        
        // Generic Linux fallback to Apt (most common)
        OSType::Linux => Some(PackageManager::Apt),
        
        // macOS
        OSType::Macos => Some(PackageManager::Brew),
        
        // Windows - prefer winget as it's the Microsoft standard
        OSType::Windows => Some(PackageManager::Winget),
        
        // Unknown OS types
        _ => None,
    }
}

fn detect_available_package_managers(os: OSType) -> Vec<PackageManager> {
    PackageManager::supported_on_os(os)
}

/// Detect CPU brand and vendor information from the system.
fn detect_cpu_info() -> (Option<String>, Option<String>) {
    #[cfg(target_os = "linux")]
    {
        detect_cpu_info_linux()
    }

    #[cfg(target_os = "macos")]
    {
        detect_cpu_info_macos()
    }

    #[cfg(target_os = "windows")]
    {
        detect_cpu_info_windows()
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        (None, None)
    }
}

#[cfg(target_os = "linux")]
fn detect_cpu_info_linux() -> (Option<String>, Option<String>) {
    use std::fs;

    let mut brand = None;
    let mut vendor = None;

    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        for line in cpuinfo.lines() {
            if let Some(value) = line.strip_prefix("model name\t: ") {
                brand = Some(value.trim().to_string());
            } else if let Some(value) = line.strip_prefix("vendor_id\t: ") {
                vendor = Some(value.trim().to_string());
            }
            if brand.is_some() && vendor.is_some() {
                break;
            }
        }
    }

    (brand, vendor)
}

#[cfg(target_os = "macos")]
fn detect_cpu_info_macos() -> (Option<String>, Option<String>) {
    use std::process::Command;

    let mut brand = None;
    let mut vendor = None;

    // Get CPU brand string
    if let Ok(output) = Command::new("sysctl")
        .arg("-n")
        .arg("machdep.cpu.brand_string")
        .output()
    {
        if output.status.success() {
            if let Ok(s) = String::from_utf8(output.stdout) {
                brand = Some(s.trim().to_string());
            }
        }
    }

    // Get CPU vendor
    if let Ok(output) = Command::new("sysctl")
        .arg("-n")
        .arg("machdep.cpu.vendor")
        .output()
    {
        if output.status.success() {
            if let Ok(s) = String::from_utf8(output.stdout) {
                vendor = Some(s.trim().to_string());
            }
        }
    }

    (brand, vendor)
}

#[cfg(target_os = "windows")]
fn detect_cpu_info_windows() -> (Option<String>, Option<String>) {
    use std::process::Command;

    let mut brand = None;
    let mut vendor = None;

    // Use wmic to get processor info
    if let Ok(output) = Command::new("wmic")
        .args(&["cpu", "get", "name"])
        .output()
    {
        if output.status.success() {
            if let Ok(s) = String::from_utf8(output.stdout) {
                // Extract name from output (skip header and take first line)
                if let Some(line) = s.lines().nth(1) {
                    brand = Some(line.trim().to_string());
                }
            }
        }
    }

    // Vendor is usually part of the brand string on Windows
    if let Some(ref b) = brand {
        if b.contains("Intel") {
            vendor = Some("GenuineIntel".to_string());
        } else if b.contains("AMD") {
            vendor = Some("AuthenticAMD".to_string());
        }
    }

    (brand, vendor)
}
