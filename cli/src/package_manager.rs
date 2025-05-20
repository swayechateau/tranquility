
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