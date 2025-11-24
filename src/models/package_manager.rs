// Module: Model/PackageManager
// Location: cli/src/model/package_manager.rs
use crate::{
    print_error, print_warn,
    models::system::SystemInfo,
    core::shell::command::{check_command, command_exists, execute_package_cmd, run_shell_command}
};

use colored::Colorize;
use dialoguer::Confirm;
use os_info::Type as OSType;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

macro_rules! pm_installer {
    ($name:expr, $install_fn:ident) => {{
        if command_exists($name) {
            true
        } else {
            let prompt = format!("{} is not installed. Install it?", $name).purple().to_string();
            if Confirm::new().with_prompt(&prompt).default(true).interact().unwrap() {
                $install_fn();
                print_warn!("Terminal session may need restarting for {} to be picked up", $name);
                true
            } else {
                false
            }
        }
    }};
}

impl PackageManager {
    pub fn supported_on_os(os: OSType) -> Vec<Self> {
        use PackageManager::*;
        match os {
            OSType::Ubuntu | OSType::Debian | OSType::Pop | OSType::Linux => vec![Apt, Snap, Flatpak, Nix],
            OSType::Fedora => vec![Dnf, Snap, Flatpak, Nix],
            OSType::Redhat => vec![Yum, Nix],
            OSType::Alpine => vec![Apk, Nix],
            OSType::Arch | OSType::Manjaro | OSType::EndeavourOS => vec![Pacman, Yay, Flatpak, Snap, Nix],
            OSType::SUSE => vec![Zypper, Nix],
            OSType::Gentoo => vec![Portage, Nix],
            OSType::Macos => vec![Brew, Nix],
            OSType::Windows => vec![Winget, Choco, Scoop, Nix],
            _ => vec![],
        }
    }

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

    pub fn check_installed(&self) -> bool {
        command_exists(self.name())
    }

    pub fn check_install(&self) -> bool {
        use PackageManager::*;
        match self {
            Yay => pm_installer!("yay", install_yay),
            Nix => pm_installer!("nix", install_nix),
            Flatpak => pm_installer!("flatpak", install_flatpak),
            Snap => pm_installer!("snap", install_snap),
            Brew => pm_installer!("brew", install_homebrew),
            Choco => pm_installer!("choco", install_choco),
            Scoop => pm_installer!("scoop", install_scoop),
            _ => default_pm_installed(*self),
        }
    }

    pub fn install(&self, use_sudo: Option<bool>, package: &str, cask: Option<bool>, dry_run: bool) {
        if matches!(self, Self::Nix) {
            print_warn!(
                "⚠️ It's recommended to install '{}' using Nix directly:\n    nix-env -iA nixpkgs.{}",
                package,
                package
            );
            return;
        }

        let (cmd, args) = match self {
            Self::Apt | Self::Dnf | Self::Yum => (self.name(), vec!["install", package, "-y"]),
            Self::Zypper => (self.name(), vec!["install", "-y", package]),
            Self::Pacman | Self::Yay => (self.name(), vec!["-S", package, "--noconfirm"]),
            Self::Portage => ("emerge", vec![package]),
            Self::Apk => ("apk", vec!["add", package]),
            Self::Flatpak => ("flatpak", vec!["install", "flathub", package]),
            Self::Snap => ("snap", vec!["install", package]),
            Self::Brew => {
                let base = vec!["install"];
                if cask.unwrap_or(false) {
                    ("brew", [base.as_slice(), &["--cask", package]].concat())
                } else {
                    ("brew", [base.as_slice(), &[package]].concat())
                }
            }
            Self::Winget => ("winget", vec!["install", package]),
            Self::Choco => ("choco", vec!["install", package, "-y"]),
            Self::Scoop => ("scoop", vec!["install", package]),
            _ => return,
        };

        execute_package_cmd(cmd, &args, use_sudo.unwrap_or(self.requires_sudo()), dry_run);
    }


    // pub fn update(&self, use_sudo: Option<bool>, dry_run: bool) {
    //     if matches!(self, Self::Nix) {
    //         print_warn!("⚠️ To update Nix packages, run:\n    nix-channel --update && nix-env -u");
    //         return;
    //     }

    //     let (cmd, args): (&str, Vec<&str>) = match self {
    //         Self::Apt => ("apt", vec!["update", "-y"]),
    //         Self::Dnf => ("dnf", vec!["update", "-y"]),
    //         Self::Yum => ("yum", vec!["update", "-y"]),
    //         Self::Zypper => ("sh", vec!["-c", "zypper refresh && zypper update -y"]),
    //         Self::Portage => ("sh", vec!["-c", "emerge --sync && emerge -avuDN @world"]),
    //         Self::Apk => ("sh", vec!["-c", "apk update && apk upgrade"]),
    //         Self::Pacman | Self::Yay => (self.name(), vec!["-Syu", "--noconfirm"]),
    //         Self::Flatpak => ("flatpak", vec!["update", "-y"]),
    //         Self::Snap => ("snap", vec!["refresh"]),
    //         Self::Brew => ("sh", vec!["-c", "brew update && brew upgrade"]),
    //         Self::Choco => ("choco", vec!["upgrade", "all", "-y"]),
    //         Self::Winget => ("winget", vec!["upgrade", "--all"]),
    //         Self::Scoop => ("scoop", vec!["update", "*"]),
    //         _ => return,
    //     };

    //     execute_package_cmd(cmd, &args, use_sudo.unwrap_or(self.requires_sudo()), dry_run);
    // }


    pub fn uninstall(&self, use_sudo: Option<bool>, package: &str, dry_run: bool) {
        if matches!(self, Self::Nix) {
            print_warn!(
                "⚠️ To uninstall Nix packages, run:\n    nix-env -e {}",
                package
            );
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

        execute_package_cmd(cmd, &args, use_sudo.unwrap_or(self.requires_sudo()), dry_run);
    }


    fn requires_sudo(&self) -> bool {
        matches!(
            self,
            Self::Apt
                | Self::Apk
                | Self::Dnf
                | Self::Flatpak
                | Self::Pacman
                | Self::Portage
                | Self::Snap
                | Self::Yay
                | Self::Zypper
        )
    }
}

fn default_pm_installed(pm: PackageManager) -> bool {
    if command_exists(pm.name()) {
        true
    } else {
        print_error!(
            "❌ {}",
            format!(
                "Package manager '{}' is not installed. Please install it manually.",
                pm.name()
            )
            .red()
        );
        false
    }
}

// Installer functions
fn install_choco() {
    run_shell_command("Set-ExecutionPolicy Bypass -Scope Process -Force; iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))");
}

fn install_scoop() {
    run_shell_command("iwr -useb get.scoop.sh | iex");
}

fn install_homebrew() {
    run_shell_command("/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"");
}

fn install_yay() {
    if !check_command("yay", "Yay") {
        println!("Installing yay...");
        run_shell_command("git clone https://aur.archlinux.org/yay.git");
        run_shell_command("cd yay && makepkg -si --noconfirm");
        run_shell_command("cd .. && rm -rf yay");
    }
}

fn install_snap() {
    let distro = SystemInfo::new().distro();
    match distro.as_str() {
        d if d.contains("Ubuntu") || d.contains("Debian") => {
            run_shell_command("sudo apt update && sudo apt install snapd -y")
        }
        d if d.contains("Fedora") => run_shell_command("sudo dnf install snapd -y"),
        d if d.contains("Arch") => run_shell_command("sudo pacman -S snapd -y"),
        _ => print_error!("❌ Unsupported distribution: {}", distro),
    }
}

fn install_flatpak() {
    let distro = SystemInfo::new().distro();
    match distro.as_str() {
        d if d.contains("Ubuntu") || d.contains("Debian") => {
            run_shell_command("sudo apt update && sudo apt install flatpak -y")
        }
        d if d.contains("Fedora") => run_shell_command("sudo dnf install flatpak -y"),
        d if d.contains("Arch") => run_shell_command("sudo pacman -S flatpak -y"),
        _ => print_error!("❌ Unsupported distribution: {}", distro),
    }
}

fn install_nix() {
    let mut cmd = "sh <(curl --proto '=https' --tlsv1.2 -L https://nixos.org/nix/install)".to_string();
    if SystemInfo::new().os_type() == OSType::Linux {
        let daemon = Confirm::new()
            .with_prompt("Do you want to install the Nix daemon?")
            .default(true)
            .interact()
            .unwrap();
        cmd.push_str(if daemon { " --daemon" } else { " --no-daemon" });
    }
    run_shell_command(&cmd);
}
