use colored::Colorize;
// src/system.rs
use os_info::Type as OSType;
use sysinfo::{System};
use bitflags::bitflags;
use serde::Deserialize;

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

        SystemInfo {
            os,
            arch,
            distro,
            cpu_vendor,
            cpu_brand,
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
    pub fn to_pretty_string(&self) -> String {
        format!(
            "\n🧠 {} {}\n\
            🖥  {} {}\n\
            🧱  {} {}\n\
            🐧  {} {}\n\
            🏷  {} {}\n\
            🧬  {} {}\n",
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
        )
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
            SystemSupport::Cross    => OsSupport::all(),
            SystemSupport::MacLin  => OsSupport::MACOS | OsSupport::LINUX,
            SystemSupport::LinWin   => OsSupport::LINUX | OsSupport::WINDOWS,
            SystemSupport::WinMac   => OsSupport::WINDOWS | OsSupport::MACOS,
            SystemSupport::Linux    => OsSupport::LINUX,
            SystemSupport::Windows  => OsSupport::WINDOWS,
            SystemSupport::MacOS    => OsSupport::MACOS,
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
    Arch
}

#[derive(Copy, Clone, Debug, clap::ValueEnum, Deserialize)]
pub enum ServerSupport {
    Fedora,
    Ubuntu,
    Debian
}

// TODO: add Supported Servers (linux distros) Ubuntu, Debian, Fedora


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
