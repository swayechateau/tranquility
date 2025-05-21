// src/package_manager.rs
use dialoguer::Confirm;
use colored::Colorize;
use os_info::Type;
use serde::Deserialize;
use crate::common::{check_command, command_exists, run_shell_command};
use crate::{print_error, print_warn};
use crate::system::SystemInfo;

/// Represents supported package managers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
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
            let install = Confirm::new()
                .with_prompt(&prompt)
                .default(true)
                .interact()
                .unwrap();
            if install {
                $install_fn();
                print_warn!("Terminal session may need restarting for {} to be picked up", $name);
            }
            install
        }
    }};
}
impl PackageManager {
    pub fn all() -> &'static [PackageManager] {
        &[
            PackageManager::Apt,
            PackageManager::Snap,
            PackageManager::Yum,
            PackageManager::Dnf,
            PackageManager::Zypper,
            PackageManager::Portage,
            PackageManager::Nix,
            PackageManager::Apk,
            PackageManager::Pacman,
            PackageManager::Yay,
            PackageManager::Flatpak,
            PackageManager::Brew,
            PackageManager::Choco,
            PackageManager::Winget,
            PackageManager::Scoop,
        ]
    }
    pub fn name(&self) -> &'static str {
        match self {
            Self::Apt => "apt",
            Self::Dnf => "dnf",
            Self::Yum => "yum",
            Self::Zypper => "zypper",
            Self::Portage => "portage",
            Self::Apk => "apk",
            Self::Pacman => "pacman",
            Self::Yay => "yay",
            Self::Nix => "nix",
            Self::Flatpak => "flatpak",
            Self::Snap => "snap",
            Self::Brew => "brew",
            Self::Choco => "choco",
            Self::Winget => "winget",
            Self::Scoop => "scoop",
        }
    }

    pub fn check_install(&self) -> bool {
        match self {
            Self::Yay => pm_installer!("yay", install_yay),
            Self::Nix => pm_installer!("nix", install_nix),
            Self::Flatpak => pm_installer!("flatpak", install_flatpak),
            Self::Snap => pm_installer!("snap", install_snap),
            Self::Brew => pm_installer!("brew", install_homebrew),
            Self::Choco => pm_installer!("choco", install_choco),
            Self::Scoop => pm_installer!("scoop", install_scoop),
            _ => default_pm_installed(*self),
        }
    }

    pub fn install(&self, package: &str, cask: Option<bool>) {
        match self {
            Self::Apt => run_shell_command(&format!("sudo apt install {} -y", package)),
            Self::Dnf => run_shell_command(&format!("sudo dnf install {} -y", package)),
            Self::Yum => run_shell_command(&format!("sudo yum install {} -y", package)),
            Self::Zypper => run_shell_command(&format!("sudo zypper install -y {}", package)),
            Self::Portage => run_shell_command(&format!("sudo emerge {}", package)),
            Self::Apk => run_shell_command(&format!("sudo apk add {}", package)),
            Self::Pacman => run_shell_command(&format!("sudo pacman -S {} -y", package)),
            Self::Yay => run_shell_command(&format!("yay -S --noconfirm {}", package)),
            Self::Nix => run_shell_command(&format!("nix-env -iA nixpkgs.{} -y", package)),
            Self::Flatpak => run_shell_command(&format!("sudo flatpak install flathub {}", package)),
            Self::Snap => run_shell_command(&format!("sudo snap install {}", package)),
            Self::Brew => {
                if cask.unwrap_or(false) {
                    run_shell_command(&format!("brew install --cask {}", package));
                } else {
                    run_shell_command(&format!("brew install {}", package));
                }
            },
            Self::Winget => run_shell_command(&format!("winget install {}", package)),
            Self::Choco => run_shell_command(&format!("choco install {} -y", package)),
            Self::Scoop => run_shell_command(&format!("scoop install {}", package)),
        }
    }

    pub fn update(&self) {
        match self {
            Self::Apt => run_shell_command("sudo apt update -y"),
            Self::Dnf => run_shell_command("sudo dnf update -y"),
            Self::Yum => run_shell_command("sudo yum update -y"),
            Self::Zypper => run_shell_command("sudo zypper refresh && sudo zypper update -y"),
            Self::Portage => run_shell_command("sudo emerge --sync && sudo emerge -avuDN @world"),
            Self::Apk => run_shell_command("sudo apk update && sudo apk upgrade"),
            Self::Pacman => run_shell_command("sudo pacman -Syu --noconfirm"),
            Self::Yay => run_shell_command("yay -Syu --noconfirm"),
            Self::Nix => run_shell_command("nix-channel --update && nix-env -u"),
            Self::Flatpak => run_shell_command("sudo flatpak update -y"),
            Self::Snap => run_shell_command("sudo snap refresh"),
            Self::Brew => run_shell_command("brew update && brew upgrade"),
            Self::Choco => run_shell_command("choco upgrade all -y"),
            Self::Winget => run_shell_command("winget upgrade --all"),
            Self::Scoop => run_shell_command("scoop update *"),
        }
    }

    pub fn uninstall(&self, package: &str) {
        match self {
            Self::Apt => run_shell_command(&format!("sudo apt remove {} -y", package)),
            Self::Dnf => run_shell_command(&format!("sudo dnf remove {} -y", package)),
            Self::Yum => run_shell_command(&format!("sudo yum remove {} -y", package)),
            Self::Zypper => run_shell_command(&format!("sudo zypper remove -y {}", package)),
            Self::Portage => run_shell_command(&format!("sudo emerge -C {}", package)),
            Self::Apk => run_shell_command(&format!("sudo apk del {}", package)),
            Self::Pacman => run_shell_command(&format!("sudo pacman -R {} --noconfirm", package)),
            Self::Yay => run_shell_command(&format!("yay -R --noconfirm {}", package)),
            Self::Nix => run_shell_command(&format!("nix-env -e {}", package)),
            Self::Flatpak => run_shell_command(&format!("sudo flatpak uninstall -y {}", package)),
            Self::Snap => run_shell_command(&format!("sudo snap remove {}", package)),
            Self::Brew => run_shell_command(&format!("brew uninstall {}", package)),
            Self::Choco => run_shell_command(&format!("choco uninstall {} -y", package)),
            Self::Winget => run_shell_command(&format!("winget uninstall {}", package)),
            Self::Scoop => run_shell_command(&format!("scoop uninstall {}", package)),
        }
    }
}

// check pm is installed
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

// packagemanager installer
fn install_choco() {
    let cmd = "Set-ExecutionPolicy Bypass -Scope Process -Force; \
               iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))";
    run_shell_command(cmd);
}

fn install_scoop() {
    let cmd = "iwr -useb get.scoop.sh | iex";
    run_shell_command(cmd);
}

fn install_homebrew() {
    let cmd = "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"";
    run_shell_command(cmd);
}

fn install_yay() {
    // check if yay is installed
    if !check_command("yay", "Yay") {
        println!("Installing yay...");
        run_shell_command("git clone https://aur.archlinux.org/yay.git");
        run_shell_command("cd yay && makepkg -si --noconfirm");
        run_shell_command("cd .. && rm -rf yay");
    }
}

fn install_snap() {
    let system = SystemInfo::new();
    let disto = system.distro();
    // check package manager and install snap
    if disto.contains("Ubuntu") || disto.contains("Debian") {
        run_shell_command("sudo apt update && sudo apt install snapd -y");
    } else if disto.contains("Fedora") {
        run_shell_command("sudo dnf install snapd -y");
    } else if disto.contains("Arch") {
        run_shell_command("sudo pacman -S snapd -y");
    } else {
        println!("❌ Unsupported distribution: {}", disto);
        std::process::exit(1);
    }
}

fn install_flatpak() {
    let system = SystemInfo::new();
    let disto = system.distro();
    // check package manager and install flatpak
    if disto.contains("Ubuntu") || disto.contains("Debian") {
        run_shell_command("sudo apt update && sudo apt install flatpak -y");
    } else if disto.contains("Fedora") {
        run_shell_command("sudo dnf install flatpak -y");
    } else if disto.contains("Arch") {
        run_shell_command("sudo pacman -S flatpak -y");
    } else {
        println!("❌ Unsupported distribution: {}", disto);
        std::process::exit(1);
    }
}

fn install_nix() {
    let mut cmd =
        "sh <(curl --proto '=https' --tlsv1.2 -L https://nixos.org/nix/install)".to_string();
    let system = SystemInfo::new();
    
    // if the target is linux
    if system.os_type() == Type::Linux {
        let daemon = Confirm::new()
            .with_prompt("Do you want to install the Nix daemon?")
            .default(true)
            .interact()
            .unwrap();

        if daemon {
            cmd.push_str(" --daemon");
        } else {
            cmd.push_str(" --no-daemon");
        }
    }

    run_shell_command(&cmd);
}
