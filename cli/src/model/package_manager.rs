use crate::shell::command::{check_command, command_exists, run_shell_command};
// src/models/package_manager.rs
use crate::shell::ShellCommand;
use crate::system::SystemInfo;
use crate::{print_error, print_warn};
use colored::Colorize;
use dialoguer::Confirm;
use os_info::Type as OSType;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents supported package managers.
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
            let prompt = format!("{} is not installed. Install it?", $name)
                .purple()
                .to_string();
            let install = Confirm::new()
                .with_prompt(&prompt)
                .default(true)
                .interact()
                .unwrap();
            if install {
                $install_fn();
                print_warn!(
                    "Terminal session may need restarting for {} to be picked up",
                    $name
                );
            }
            install
        }
    }};
}
impl PackageManager {
    pub fn supported_on_os(os: OSType) -> Vec<PackageManager> {
        match os {
            OSType::Ubuntu | OSType::Debian | OSType::Pop | OSType::Linux => vec![
                PackageManager::Apt,
                PackageManager::Snap,
                PackageManager::Flatpak,
                PackageManager::Nix,
            ],
            OSType::Fedora => vec![
                PackageManager::Dnf,
                PackageManager::Snap,
                PackageManager::Flatpak,
                PackageManager::Nix,
            ],
            OSType::Redhat => vec![PackageManager::Yum, PackageManager::Nix],
            OSType::Alpine => vec![PackageManager::Apk, PackageManager::Nix],
            OSType::Arch | OSType::Manjaro | OSType::EndeavourOS => vec![
                PackageManager::Pacman,
                PackageManager::Yay,
                PackageManager::Flatpak,
                PackageManager::Snap,
                PackageManager::Nix,
            ],
            OSType::SUSE => vec![PackageManager::Zypper, PackageManager::Nix],
            OSType::Gentoo => vec![PackageManager::Portage, PackageManager::Nix],
            OSType::Macos => vec![PackageManager::Brew, PackageManager::Nix],
            OSType::Windows => vec![
                PackageManager::Winget,
                PackageManager::Choco,
                PackageManager::Scoop,
                PackageManager::Nix,
            ],
            _ => vec![],
        }
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

    pub fn check_installed(&self) -> bool {
        match self {
            _ => command_exists(self.name()),
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

    pub fn install(&self, package: &str, cask: Option<bool>, dry_run: bool) {
        let (cmd, args): (&str, Vec<String>) = match self {
            Self::Apt => (
                "apt",
                vec!["install".to_string(), package.to_string(), "-y".to_string()],
            ),
            Self::Dnf => (
                "dnf",
                vec!["install".to_string(), package.to_string(), "-y".to_string()],
            ),
            Self::Yum => (
                "yum",
                vec!["install".to_string(), package.to_string(), "-y".to_string()],
            ),
            Self::Zypper => (
                "zypper",
                vec!["install".to_string(), "-y".to_string(), package.to_string()],
            ),
            Self::Portage => ("emerge", vec![package.to_string()]),
            Self::Apk => ("apk", vec!["add".to_string(), package.to_string()]),
            Self::Pacman => (
                "pacman",
                vec![
                    "-S".to_string(),
                    package.to_string(),
                    "--noconfirm".to_string(),
                ],
            ),
            Self::Yay => (
                "yay",
                vec![
                    "-S".to_string(),
                    package.to_string(),
                    "--noconfirm".to_string(),
                ],
            ),
            Self::Nix => (
                "nix-env",
                vec!["-iA".to_string(), format!("nixpkgs.{}", package)],
            ),
            Self::Flatpak => (
                "flatpak",
                vec![
                    "install".to_string(),
                    "flathub".to_string(),
                    package.to_string(),
                ],
            ),
            Self::Snap => ("snap", vec!["install".to_string(), package.to_string()]),
            Self::Brew => {
                if cask.unwrap_or(false) {
                    (
                        "brew",
                        vec![
                            "install".to_string(),
                            "--cask".to_string(),
                            package.to_string(),
                        ],
                    )
                } else {
                    ("brew", vec!["install".to_string(), package.to_string()])
                }
            }
            Self::Winget => ("winget", vec!["install".to_string(), package.to_string()]),
            Self::Choco => (
                "choco",
                vec!["install".to_string(), package.to_string(), "-y".to_string()],
            ),
            Self::Scoop => ("scoop", vec!["install".to_string(), package.to_string()]),
        };

        let args_ref: Vec<&str> = args.iter().map(String::as_str).collect();

        ShellCommand::new(cmd)
            .with_args(&args_ref)
            .with_sudo(!matches!(
                self,
                Self::Brew | Self::Yay | Self::Nix | Self::Winget | Self::Choco | Self::Scoop
            ))
            .run_verbose(dry_run);
    }

    pub fn update(&self, dry_run: bool) {
        let cmd = match self {
            Self::Apt => ("apt", vec!["update", "-y"]),
            Self::Dnf => ("dnf", vec!["update", "-y"]),
            Self::Yum => ("yum", vec!["update", "-y"]),
            Self::Zypper => ("sh", vec!["-c", "zypper refresh && zypper update -y"]),
            Self::Portage => ("sh", vec!["-c", "emerge --sync && emerge -avuDN @world"]),
            Self::Apk => ("sh", vec!["-c", "apk update && apk upgrade"]),
            Self::Pacman => ("pacman", vec!["-Syu", "--noconfirm"]),
            Self::Yay => ("yay", vec!["-Syu", "--noconfirm"]),
            Self::Nix => ("sh", vec!["-c", "nix-channel --update && nix-env -u"]),
            Self::Flatpak => ("flatpak", vec!["update", "-y"]),
            Self::Snap => ("snap", vec!["refresh"]),
            Self::Brew => ("sh", vec!["-c", "brew update && brew upgrade"]),
            Self::Choco => ("choco", vec!["upgrade", "all", "-y"]),
            Self::Winget => ("winget", vec!["upgrade", "--all"]),
            Self::Scoop => ("scoop", vec!["update", "*"]),
        };

        ShellCommand::new(cmd.0)
            .with_args(&cmd.1)
            .with_sudo(!matches!(
                self,
                Self::Brew | Self::Yay | Self::Nix | Self::Winget | Self::Choco | Self::Scoop
            ))
            .run_verbose(dry_run);
    }

    pub fn uninstall(&self, package: &str, dry_run: bool) {
        let (cmd, args) = match self {
            Self::Apt => ("apt", vec!["remove", package, "-y"]),
            Self::Dnf => ("dnf", vec!["remove", package, "-y"]),
            Self::Yum => ("yum", vec!["remove", package, "-y"]),
            Self::Zypper => ("zypper", vec!["remove", "-y", package]),
            Self::Portage => ("emerge", vec!["-C", package]),
            Self::Apk => ("apk", vec!["del", package]),
            Self::Pacman => ("pacman", vec!["-R", package, "--noconfirm"]),
            Self::Yay => ("yay", vec!["-R", package, "--noconfirm"]),
            Self::Nix => ("nix-env", vec!["-e", package]),
            Self::Flatpak => ("flatpak", vec!["uninstall", "-y", package]),
            Self::Snap => ("snap", vec!["remove", package]),
            Self::Brew => ("brew", vec!["uninstall", package]),
            Self::Choco => ("choco", vec!["uninstall", package, "-y"]),
            Self::Winget => ("winget", vec!["uninstall", package]),
            Self::Scoop => ("scoop", vec!["uninstall", package]),
        };

        ShellCommand::new(cmd)
            .with_args(&args)
            .with_sudo(!matches!(
                self,
                Self::Brew | Self::Yay | Self::Nix | Self::Winget | Self::Choco | Self::Scoop
            ))
            .run_verbose(dry_run);
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
    if system.os_type() == OSType::Linux {
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
