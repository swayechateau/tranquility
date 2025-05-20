use dialoguer::Confirm;
use colored::Colorize;
use os_info::Type;
use crate::common::{check_command, command_exists, run_powershell_command, run_shell_command};
use crate::{print_warn};
use crate::system::SystemInfo;

/// Represents supported package managers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            Self::Yay => yay_installed(),
            Self::Nix => nix_installed(),
            Self::Flatpak => flatpak_installed(),
            Self::Snap => snap_installed(),
            Self::Brew => brew_installed(),
            Self::Choco => choco_installed(),
            Self::Scoop => scoop_installed(),
            _ => default_pm_installed(*self),
        }
    }

    pub fn install(&self, package: &str, cask: Option<bool>) {
        match self {
            Self::Apt => install_with_apt(package),
            Self::Dnf => install_with_dnf(package),
            Self::Yum => install_with_yum(package),
            Self::Zypper => install_with_zypper(package),
            Self::Portage => install_with_portage(package),
            Self::Apk => install_with_apk(package),
            Self::Pacman => install_with_pacman(package),
            Self::Yay => install_with_yay(package),
            Self::Nix => install_with_nix(package),
            Self::Flatpak => install_with_flatpak(package),
            Self::Snap => install_with_snap(package),
            Self::Brew => install_with_brew(package, cask),
            Self::Choco => install_with_choco(package),
            Self::Winget => install_with_winget(package),
            Self::Scoop => install_with_scoop(package),
        }
    }

    pub fn update(&self) {
        match self {
            Self::Apt => run_shell_command("sudo apt update"),
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
            Self::Choco => run_powershell_command("choco upgrade all -y"),
            Self::Winget => run_powershell_command("winget upgrade --all"),
            Self::Scoop => run_powershell_command("scoop update *"),
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
            Self::Choco => run_powershell_command(&format!("choco uninstall {} -y", package)),
            Self::Winget => run_powershell_command(&format!("winget uninstall {}", package)),
            Self::Scoop => run_powershell_command(&format!("scoop uninstall {}", package)),
        }
    }
}

// check pm is installed
fn default_pm_installed(pm: PackageManager) -> bool {
    if command_exists(pm.name()) {
        true
    } else {
        eprintln!(
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

fn yay_installed() -> bool {
    pm_installer!("yay", install_yay)
}

fn nix_installed() -> bool {
    pm_installer!("nix", install_nix)
}

fn brew_installed() -> bool {
    pm_installer!("brew", install_homebrew)
}

fn snap_installed() -> bool {
    pm_installer!("snap", install_snap)
}

fn flatpak_installed() -> bool {
    pm_installer!("flatpak", install_flatpak)
}

fn scoop_installed() -> bool {
    pm_installer!("scoop", install_scoop)
}

fn choco_installed() -> bool {
    pm_installer!("choco", install_choco)
}
// installs additional package managers
pub fn install_package_manager() {
    // Check if Nix is installed
    if !check_command("nix","Nix") {
        install_nix();
    }

    // check if os is linux
    if cfg!(target_os = "linux") {
        // check if snap is installed
        if !check_command("snap", "Snap") {
            install_snap();
        }

        // check if flatpak is installed
        if !check_command("flatpak", "Flatpak") {
            install_flatpak();
        }
    }

    // check if os is windows
    if cfg!(target_os = "windows") {
        // check if choco is installed
        if !check_command("choco", "Chocolately") {
            install_choco();
        }

        // check if scoop is installed
        if !check_command("scoop", "Scoop") {
            install_scoop();
        }
    }
}

// packagemanager installer
fn install_choco() {
    let cmd = "Set-ExecutionPolicy Bypass -Scope Process -Force; \
               iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))";
    run_powershell_command(cmd);
}

fn install_scoop() {
    let cmd = "iwr -useb get.scoop.sh | iex";
    run_powershell_command(cmd);
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

// install with commands
fn install_with_winget(cmd: &str) {
    run_powershell_command(&format!("winget install {}", cmd));
}

fn install_with_choco(cmd: &str) {
    run_powershell_command(&format!("choco install {} -y", cmd));
}

fn install_with_scoop(cmd: &str) {
    run_powershell_command(&format!("scoop install {}", cmd));
}

fn install_with_brew(cmd: &str, cask: Option<bool>) {
    if cask.unwrap_or(false) {
        run_shell_command(&format!("brew install --cask {}", cmd));
    } else {
        run_shell_command(&format!("brew install {}", cmd));
    }
}

fn install_with_apt(cmd: &str) {
    run_shell_command(&format!("sudo apt update && sudo apt install {} -y", cmd));
}

fn install_with_dnf(cmd: &str) {
    run_shell_command(&format!("sudo dnf install {} -y", cmd));
}

fn install_with_yum(pkg: &str) {
    run_shell_command(&format!("sudo yum install {} -y", pkg));
}

fn install_with_zypper(pkg: &str) {
    run_shell_command(&format!("sudo zypper install -y {}", pkg));
}

fn install_with_portage(pkg: &str) {
    run_shell_command(&format!("sudo emerge {}", pkg));
}

fn install_with_apk(pkg: &str) {
    run_shell_command(&format!("sudo apk add {}", pkg));
}

fn install_with_pacman(cmd: &str) {
    run_shell_command(&format!("sudo pacman -S {} -y", cmd));
}

fn install_with_yay(cmd: &str) {
    run_shell_command(&format!("yay -S --noconfirm {}", cmd));
}

fn install_with_snap(cmd: &str) {
    run_shell_command(&format!("sudo snap install {}", cmd));
}

fn install_with_flatpak(cmd: &str) {
    run_shell_command(&format!("sudo flatpak install flathub {}", cmd));
}

fn install_with_nix(cmd: &str) {
    run_shell_command(&format!("nix-env -iA nixpkgs.{} -y", cmd));
}
