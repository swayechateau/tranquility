use os_info::Type as OSType;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};

/// Supported package managers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub enum PackageManager {
    Apt,
    Snap,
    Yum,
    Dnf,
    Zypper,
    Portage,
    Nix,
    Apk,
    Pacman,
    Yay,
    Flatpak,
    Brew,
    Choco,
    Winget,
    Scoop,
}

impl PackageManager {
    /// Get supported package managers for a given OS.
    pub fn supported_on_os(os: OSType) -> Vec<Self> {
        use PackageManager::*;
        match os {
            OSType::Ubuntu | OSType::Debian | OSType::Pop | OSType::Linux => {
                vec![Apt, Snap, Flatpak, Nix]
            }
            OSType::Fedora => vec![Dnf, Flatpak, Nix],
            OSType::Redhat => vec![Yum, Nix],
            OSType::Alpine => vec![Apk, Nix],
            OSType::Arch | OSType::Manjaro | OSType::EndeavourOS => {
                vec![Pacman, Yay, Flatpak, Nix]
            }
            OSType::SUSE | OSType::openSUSE => vec![Zypper, Flatpak, Nix],
            OSType::Gentoo => vec![Portage, Nix],
            OSType::Macos => vec![Brew, Nix],
            OSType::Windows => vec![Winget, Choco, Scoop, Nix],
            _ => vec![],
        }
    }

    /// Get the CLI name of the package manager.
    pub fn name(&self) -> &'static str {
        use PackageManager::*;
        match self {
            Apt => "apt",
            Dnf => "dnf",
            Yum => "yum",
            Zypper => "zypper",
            Portage => "portage",
            Apk => "apk",
            Pacman => "pacman",
            Yay => "yay",
            Nix => "nix",
            Flatpak => "flatpak",
            Snap => "snap",
            Brew => "brew",
            Choco => "choco",
            Winget => "winget",
            Scoop => "scoop",
        }
    }

    /// Whether this package manager requires sudo on Linux.
    pub fn requires_sudo(&self) -> bool {
        matches!(
            self,
            Self::Apt
                | Self::Apk
                | Self::Dnf
                | Self::Yum
                | Self::Flatpak
                | Self::Pacman
                | Self::Portage
                | Self::Snap
                | Self::Yay
                | Self::Zypper
        )
    }

    /// Check if the package manager binary is in PATH.
    pub fn is_available(&self) -> bool {
        command_exists(self.name())
    }

    /// Install a package using this package manager.
    pub fn install(&self, package: &str, is_cask: Option<bool>, dry_run: bool) {
        if matches!(self, Self::Nix) {
            println!(
                "⚠️ Install '{}' via Nix manually:\n    nix-env -iA nixpkgs.{}",
                package, package
            );
            return;
        }

        let (cmd, args): (&str, Vec<&str>) = match self {
            Self::Apt | Self::Dnf | Self::Yum => (self.name(), vec!["install", package, "-y"]),
            Self::Zypper => (self.name(), vec!["install", "-y", package]),
            Self::Pacman | Self::Yay => (self.name(), vec!["-S", package, "--noconfirm"]),
            Self::Portage => ("emerge", vec![package]),
            Self::Apk => ("apk", vec!["add", package]),
            Self::Flatpak => ("flatpak", vec!["install", "flathub", package, "-y"]),
            Self::Snap => ("snap", vec!["install", package]),
            Self::Brew => {
                if is_cask.unwrap_or(false) {
                    ("brew", vec!["install", "--cask", package])
                } else {
                    ("brew", vec!["install", package])
                }
            }
            Self::Winget => ("winget", vec!["install", package]),
            Self::Choco => ("choco", vec!["install", package, "-y"]),
            Self::Scoop => ("scoop", vec!["install", package]),
            _ => return,
        };

        run_package_cmd(cmd, &args, self.requires_sudo(), dry_run);
    }

    /// Uninstall a package using this package manager.
    pub fn uninstall(&self, package: &str, dry_run: bool) {
        if matches!(self, Self::Nix) {
            println!("⚠️ Uninstall via Nix manually:\n    nix-env -e {}", package);
            return;
        }

        let (cmd, args): (&str, Vec<&str>) = match self {
            Self::Apt | Self::Dnf | Self::Yum => (self.name(), vec!["remove", package, "-y"]),
            Self::Zypper => ("zypper", vec!["remove", "-y", package]),
            Self::Pacman | Self::Yay => (self.name(), vec!["-R", package, "--noconfirm"]),
            Self::Portage => ("emerge", vec!["-C", package]),
            Self::Apk => ("apk", vec!["del", package]),
            Self::Flatpak => ("flatpak", vec!["uninstall", "-y", package]),
            Self::Snap => ("snap", vec!["remove", package]),
            Self::Brew => ("brew", vec!["uninstall", package]),
            Self::Choco => ("choco", vec!["uninstall", package, "-y"]),
            Self::Winget => ("winget", vec!["uninstall", package]),
            Self::Scoop => ("scoop", vec!["uninstall", package]),
            _ => return,
        };

        run_package_cmd(cmd, &args, self.requires_sudo(), dry_run);
    }
}

/// Check whether a command exists in PATH.
pub fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Run a shell command via the given executable, optionally with sudo.
fn run_package_cmd(cmd: &str, args: &[&str], sudo: bool, dry_run: bool) {
    let full_cmd = if sudo && !cfg!(windows) {
        let mut parts = vec!["sudo", cmd];
        parts.extend_from_slice(args);
        parts.join(" ")
    } else {
        let mut parts = vec![cmd];
        parts.extend_from_slice(args);
        parts.join(" ")
    };

    if dry_run {
        println!("💡 [dry run] {}", full_cmd);
        return;
    }

    let status = if cfg!(windows) {
        Command::new("cmd").args(["/C", &full_cmd]).status()
    } else if sudo {
        Command::new("sudo").arg(cmd).args(args).status()
    } else {
        Command::new(cmd).args(args).status()
    };

    match status {
        Ok(s) if s.success() => {}
        Ok(s) => eprintln!("⚠️ Command exited with status: {}", s),
        Err(e) => eprintln!("❌ Failed to run '{}': {}", full_cmd, e),
    }
}

/// Run an arbitrary shell command string.
pub fn run_shell_command(cmd: &str) {
    if cfg!(windows) {
        let _ = Command::new("cmd").args(["/C", cmd]).status();
    } else {
        let _ = Command::new("sh").args(["-c", cmd]).status();
    }
}
