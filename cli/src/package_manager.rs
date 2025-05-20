
// src/packagemaners.rs

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
    Flatpak,
    Brew,
    Choco,
    Winget,
    Scoop,
}

impl PackageManager {
    pub fn name(&self) -> &'static str {
        match self {
            PackageManager::Apt => "apt",
            PackageManager::Dnf => "dnf",
            PackageManager::Yum => "yum",
            PackageManager::Zypper => "zypper",
            PackageManager::Portage => "portage",
            PackageManager::Apk => "apk",
            PackageManager::Pacman => "pacman",
            PackageManager::Nix => "nix",
            PackageManager::Flatpak => "flatpak",
            PackageManager::Snap => "snap",
            PackageManager::Brew => "brew",
            PackageManager::Choco => "choco",
            PackageManager::Winget => "winget",
            PackageManager::Scoop => "scoop",
        }
    }
    pub fn install(&self) -> bool {
        match self {
            PackageManager::Apt => default_pm_installed(PackageManager::Apt),
            PackageManager::Dnf => default_pm_installed(PackageManager::Dnf),
            PackageManager::Yum => default_pm_installed(PackageManager::Yum),
            PackageManager::Zypper => default_pm_installed(PackageManager::Zypper),
            PackageManager::Portage => default_pm_installed(PackageManager::Portage),
            PackageManager::Apk => default_pm_installed(PackageManager::Apk),
            PackageManager::Pacman => default_pm_installed(PackageManager::Pacman),
            PackageManager::Winget => default_pm_installed(PackageManager::Winget),
            PackageManager::Nix => nix_installed(),
            PackageManager::Flatpak => flatpak_installed(),
            PackageManager::Snap => snap_installed(),
            PackageManager::Brew => brew_installed(),
            PackageManager::Choco => chco_installed(),
            PackageManager::Scoop => scoop_installed(),
        }
    }
}

// check if packagemanager exist - if not tell user then ask if they want to install
fn check_for_package_manader(pm: PackageManager) {

}
// packagemanager installer
fn install_homebrew() {

}
fn install_scoop() {

}

fn install_chco() {

}
fn install_yay() {
    // check if yay is installed
    if !check_command("yay", "Yay", false) {
        println!("Installing yay...");
        run_shell_command("git clone https://aur.archlinux.org/yay.git");
        run_shell_command("cd yay && makepkg -si --noconfirm");
        run_shell_command("cd .. && rm -rf yay");
    }
}

fn install_nix() {

}

fn install_snap() {

}

fn install_flatpak() {

}

// install with package manager

// check pm is installed
fn default_pm_installed(pm: PackageManager) -> bool {
    // check command - if true return true 

    // else print error and return false
    print_error!("Please install {} manually as tranquility does not support this feature", pm.name);
    return false
}

fn nix_installed() -> bool {

}

fn brew_installed() -> bool {
    
}

fn snap_installed() -> bool {
    
}

fn flatpak_installed() -> bool {
    
}
fn scoop_installed() -> bool {

}

fn chco_installed() -> bool {

}



// src/features/package_managers.rs

use dialoguer::Confirm;

use crate::common::{run_shell_command, run_powershell_command, determine_os, determine_distro, check_command};

// get default package manager
pub fn get_default_pm() -> &'static str {
    let os = determine_os();
    let distro = determine_distro();
    // check for macos
    if os.contains("macOS") {
        return "brew";
    }

    // check for windows
    if os.contains("Windows") {
        return "winget";
    }

    if !os.contains("Linux") {
        return "unknown";
    }
    
    if distro.contains("Ubuntu") || distro.contains("Debian") {
        return "apt";
    }

    if distro.contains("Fedora") {
        return "dnf";
    }
    
    if distro.contains("Arch") {
        return "pacman";
    }

    return "unknown";    
}

// checks for default package managers
pub fn check_default_pm() {
    let package_manager = get_default_pm();
    // check if package manager is installed
    let mut installed = false;
    if package_manager != "unknown" {
        installed = check_command(package_manager, package_manager, true);
    }

    // if package manager is not installed, inform the user to install it
    if !installed && package_manager != "brew" {
        println!("❌ Please install the default package manager for your system.");
        std::process::exit(1);
    }

    if !installed && package_manager == "brew" {
        install_brew();
    }
}

// installs additional package managers
pub fn install_package_manager() {
    // Check if Nix is installed
    if !check_command("nix","Nix", true) {
        install_nix();
    }

    // check if os is linux
    if cfg!(target_os = "linux") {
        // check if snap is installed
        if !check_command("snap", "Snap", true) {
            install_snap();
        }

        // check if flatpak is installed
        if !check_command("flatpak", "Flatpak", true) {
            install_flatpak();
        }
    }

    // check if os is windows
    if cfg!(target_os = "windows") {
        // check if choco is installed
        if !check_command("choco", "Chocolately", true) {
            install_choco();
        }

        // check if scoop is installed
        if !check_command("scoop", "Scoop", true) {
            install_scoop();
        }
    }
}

pub fn install_brew() {
    let cmd = "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"";
    run_shell_command(cmd);
}

pub fn install_choco() {
    let cmd = "Set-ExecutionPolicy Bypass -Scope Process -Force; \
               iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))";
    run_powershell_command(cmd);
}

pub fn install_scoop() {
    let cmd = "iwr -useb get.scoop.sh | iex";
    run_powershell_command(cmd);
}

pub fn install_nix() {
    let mut cmd =
        "sh <(curl --proto '=https' --tlsv1.2 -L https://nixos.org/nix/install)".to_string();

    // if the target is linux
    if cfg!(target_os = "linux") {
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

pub fn install_snap() {
    let disto = determine_distro();
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

pub fn install_flatpak() {
    let disto = determine_distro();
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

pub fn install_aur_helper() {
    // check if yay is installed
    if !check_command("yay", "Yay", true) {
        println!("Installing yay...");
        run_shell_command("git clone https://aur.archlinux.org/yay.git");
        run_shell_command("cd yay && makepkg -si --noconfirm");
        run_shell_command("cd .. && rm -rf yay");
    }
}

// install with commands
pub fn install_with(pm: &str, cmd: &str, cask: bool) {
    match pm {
        "apt" => install_with_apt(cmd),
        "dnf" => install_with_dnf(cmd),
        "pacman" => install_with_pacman(cmd),
        "yay" => install_with_yay(cmd),
        "choco" => install_with_choco(cmd),
        "scoop" => install_with_scoop(cmd),
        "nix" => install_with_nix(cmd),
        "snap" => install_with_snap(cmd),
        "flatpak" => install_with_flatpak(cmd),
        "brew" => install_with_brew(cmd, cask),
        "winget" => install_with_winget(cmd),
        _ => {
            eprintln!("❌ Unsupported package manager: {}", pm);
            std::process::exit(1);
        }
    }
}
pub fn install_with_apt(cmd: &str) {
    run_shell_command(&format!("sudo apt update && sudo apt install {} -y", cmd));
}

pub fn install_with_dnf(cmd: &str) {
    run_shell_command(&format!("sudo dnf install {} -y", cmd));
}

pub fn install_with_pacman(cmd: &str) {
    run_shell_command(&format!("sudo pacman -S {} -y", cmd));
}

fn install_with_choco(cmd: &str) {
    run_powershell_command(&format!("choco install {} -y", cmd));
}

fn install_with_scoop(cmd: &str) {
    run_powershell_command(&format!("scoop install {}", cmd));
}

fn install_with_nix(cmd: &str) {
    run_shell_command(&format!("nix-env -iA nixpkgs.{} -y", cmd));
}

fn install_with_snap(cmd: &str) {
    run_shell_command(&format!("sudo snap install {}", cmd));
}

fn install_with_flatpak(cmd: &str) {
    run_shell_command(&format!("sudo flatpak install flathub {}", cmd));
}

pub fn install_with_brew(cmd: &str, cask: bool) {
    if cask {
        run_shell_command(&format!("brew install --cask {}", cmd));
        return;
    }

    run_shell_command(&format!("brew install {}", cmd));
}

pub fn install_with_winget(cmd: &str) {
    run_powershell_command(&format!("winget install {}", cmd));
}

pub fn install_with_yay(cmd: &str) {
    run_shell_command(&format!("yay -S --noconfirm {}", cmd));
}