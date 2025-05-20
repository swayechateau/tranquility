// src/system.rs
use os_info::Type as OSType;
use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy)]
    pub struct OsSupport: u8 {
        const LINUX   = 0b001;
        const WINDOWS = 0b010;
        const MACOS   = 0b100;
    }
}
// #[derive(Debug)]
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
        // For CPU you could shell out to /proc/cpuinfo on Linux, or
        // use a crate like raw_cpuid. Here we leave as None.
        let (cpu_vendor, cpu_brand) = (None, None);

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
    pub fn distro(&self) -> Option<&str> {
        self.distro.as_deref()
    }
    // pub fn arch(&self) -> String {
    //     self.arch
    // }
}

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
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
