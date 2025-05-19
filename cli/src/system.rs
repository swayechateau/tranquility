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
        // For distro we can pull from os_info’s version type if Linux
        let distro = match info.os_type() {
            OSType::Linux => Some(format!("{}", info.version())),
            _ => None,
        };
        // For CPU you could shell out to /proc/cpuinfo on Linux, or
        // use a crate like raw_cpuid. Here we leave as None.
        let (cpu_vendor, cpu_brand) = (None, None);

        SystemInfo {
            os: info.os_type(),
            arch,
            distro,
            cpu_vendor,
            cpu_brand,
        }
    }
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