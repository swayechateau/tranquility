// src/common.rs
use dialoguer::Confirm;
use std::{env, process::Command};

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

// checks for default package managers
pub fn check_default_pm() -> &'static str {
    let mut package_manager = "unknown";
    if cfg!(target_os = "linux") {
        let distro = determine_distro();

        if distro.contains("Ubuntu") || distro.contains("Debian") {
            package_manager = "apt";
        }

        if distro.contains("Fedora") {
            package_manager = "dnf";
        }
        if distro.contains("Arch") {
            package_manager = "pacman";
        }
    }

    // check for macos
    if cfg!(target_os = "macos") {
        package_manager = "brew";
    }

    // check for windows
    if cfg!(target_os = "windows") {
        package_manager = "winget";
    }

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

    return package_manager;
}

pub fn check_command(cmd: &str, friendly_name: &str, check_version: bool) -> bool {
    let mut arg = "";
    if check_version {
        arg = "--version";
    }

    if check_version && cmd == "go" {
        arg = "version";
    }

    if Command::new(cmd).arg(arg).status().is_ok() {
        println!("✅ {} is installed.", friendly_name);
        return true;
    }
    
    println!("❌ {} is not installed.", friendly_name);

    return false;
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

// installs additional shells
pub fn install_shell() {
    // check if zsh is installed
    if !check_command("zsh", "zsh", true) {
        install_zsh();
    }

    // check if fish is installed
    if !check_command("fish", "fish",true) {
        install_fish();
    }
}

pub fn install_zsh() {
    // ask user if they want to install zsh
    let install_zsh = Confirm::new()
        .with_prompt("Do you want to install zsh?")
        .default(true)
        .interact()
        .unwrap();

    if install_zsh {
        let distro = determine_distro();
        // check package manager and install zsh
        if distro.contains("Ubuntu") || distro.contains("Debian") {
            run_shell_command("sudo apt update && sudo apt install zsh -y");
        } else if distro.contains("Fedora") {
            run_shell_command("sudo dnf install zsh -y");
        } else if distro.contains("Arch") {
            run_shell_command("sudo pacman -S zsh -y");
        } else {
            println!("❌ Unsupported distribution: {}", distro);
            std::process::exit(1);
        }
    }
}

pub fn install_fish() {
    // ask user if they want to install fish
    let install_fish = Confirm::new()
        .with_prompt("Do you want to install fish?")
        .default(true)
        .interact()
        .unwrap();

    if install_fish {
        let distro = determine_distro();
        // check package manager and install fish
        if distro.contains("Ubuntu") || distro.contains("Debian") {
            run_shell_command("sudo apt update && sudo apt install fish -y");
        } else if distro.contains("Fedora") {
            run_shell_command("sudo dnf install fish -y");
        } else if distro.contains("Arch") {
            run_shell_command("sudo pacman -S fish -y");
        } else {
            println!("❌ Unsupported distribution: {}", distro);
            std::process::exit(1);
        }
    }
}

// install additional languages
pub fn install_programming_languages() {
    // check if goland is installed
    if !check_command("go", "Go", true) {
        install_go();
    }

    // check if rust is installed
    if !check_command("cargo", "Rust and Cargo", true) {
        install_rust();
    }

    // check if node is installed
    if !check_command("node", "Node", true) {
        install_node();
    }

    // check if python is installed
    if !check_command("python3", "Python", true) {
        install_python();
    }

    // check if php is installed
    if !check_command("php", "PHP", true) {
        install_php();
    }

    // check if ruby is installed
    if !check_command("ruby", "Ruby", true) {
        install_ruby();
    }

    // check if java is installed
    if !check_command("java", "Java", true) {
        install_java();
    }

    // check if .net is installed
    if !check_command("dotnet", ".Net", true) {
        install_dotnet();
    }

    // check if elixir is installed
    if !check_command("elixir", "Elixir", true) {
        install_elixir();
    }

    // check if C is installed
    if !check_command("gcc", "C", true) {
        install_c();
    }

    // check if C++ is installed
    if !check_command("g++", "C++", true) {
        install_cpp();
    }
}

pub fn install_go() {
    let install_go = Confirm::new()
        .with_prompt("Do you want to install Go?")
        .default(true)
        .interact()
        .unwrap();

    if install_go {
        install_with(check_default_pm(), "go", false);
    }
}

pub fn install_rust() {
    let install_rust = Confirm::new()
        .with_prompt("Do you want to install Rust?")
        .default(true)
        .interact()
        .unwrap();

    if install_rust {
        run_shell_command("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh");
    }
}

pub fn install_node() {
    let install_node = Confirm::new()
        .with_prompt("Do you want to install Node?")
        .default(true)
        .interact()
        .unwrap();

    if install_node {
        install_with(check_default_pm(), "nodejs", false);
    }
}

pub fn install_python() {
    let install_python = Confirm::new()
        .with_prompt("Do you want to install Python?")
        .default(true)
        .interact()
        .unwrap();

    if install_python {
        install_with(check_default_pm(), "python3", false);
    }
}

pub fn install_php() {
    let install_php = Confirm::new()
        .with_prompt("Do you want to install PHP?")
        .default(true)
        .interact()
        .unwrap();

    if install_php {
        install_with(check_default_pm(), "php", false);
    }
}

pub fn install_ruby() {
    let install_ruby = Confirm::new()
        .with_prompt("Do you want to install Ruby?")
        .default(true)
        .interact()
        .unwrap();

    if install_ruby {
        install_with(check_default_pm(), "ruby", false);
    }
}

pub fn install_java() {
    let install_java = Confirm::new()
        .with_prompt("Do you want to install Java?")
        .default(true)
        .interact()
        .unwrap();

    if install_java {
        install_with(check_default_pm(), "openjdk-17-jdk", false);
    }
}

pub fn install_dotnet() {
    let install_dotnet = Confirm::new()
        .with_prompt("Do you want to install .NET?")
        .default(true)
        .interact()
        .unwrap();

    if install_dotnet {
        install_with(check_default_pm(), "dotnet-core", false);
    }
}

pub fn install_elixir() {
    let install_elixir = Confirm::new()
        .with_prompt("Do you want to install Elixir?")
        .default(true)
        .interact()
        .unwrap();

    if install_elixir {
        install_with(check_default_pm(), "elixir", false);
    }
}

pub fn install_c() {
    let install_c = Confirm::new()
        .with_prompt("Do you want to install C?")
        .default(true)
        .interact()
        .unwrap();

    if install_c {
        install_with(check_default_pm(), "gcc", false);
    }
}

pub fn install_cpp() {
    let install_cpp = Confirm::new()
        .with_prompt("Do you want to install C++?")
        .default(true)
        .interact()
        .unwrap();

    if install_cpp {
        install_with(check_default_pm(), "g++", false);
    }
}

// run shell command
pub fn run_shell_command(command: &str) {
    println!("🚀 Running shell command: {}", command);
    if let Err(err) = Command::new("sh").arg("-c").arg(command).status() {
        eprintln!("❌ Failed to execute command: {}\n{}", command, err);
        std::process::exit(1);
    }
}

pub fn run_powershell_command(command: &str) {
    if let Err(err) = Command::new("powershell")
        .arg("-Command")
        .arg(command)
        .status()
    {
        eprintln!("❌ PowerShell command failed: {}\n{}", command, err);
        std::process::exit(1);
    }
}


pub fn determine_os() -> String {
    if cfg!(target_os = "windows") {
        "Windows".to_string()
    } else if cfg!(target_os = "macos") {
        "macOS".to_string()
    } else if cfg!(target_os = "linux") {
        "Linux".to_string()
    } else {
        "Unknown".to_string()
    }
}

pub fn determine_arch() -> String {
    env::consts::ARCH.to_string()
}

pub fn determine_distro() -> String {
    if cfg!(target_os = "linux") {
        if let Ok(output) = Command::new("lsb_release").arg("-d").output() {
            if let Ok(desc) = String::from_utf8(output.stdout) {
                if desc.contains("Fedora") {
                    return "Fedora".to_string();
                }
                if desc.contains("Ubuntu") {
                    return "Ubuntu".to_string();
                }
                if desc.contains("Debian") {
                    return "Debian".to_string();
                }
                if desc.contains("Arch") {
                    return "Arch".to_string();
                }
                return desc.trim().to_string(); // fallback: return full description
            }
        }
    }
    "Unknown".to_string()
}


// install with commands
fn install_with(pm: &str, cmd: &str, cask: bool) {
    match pm {
        "apt" => install_with_apt(cmd),
        "dnf" => install_with_dnf(cmd),
        "pacman" => install_with_pacman(cmd),
        "choco" => install_with_choco(cmd),
        "scoop" => install_with_scoop(cmd),
        "nix" => install_with_nix(cmd),
        "snap" => install_with_snap(cmd),
        "flatpak" => install_with_flatpak(cmd),
        "brew" => install_with_brew(cmd, cask),
        _ => {
            eprintln!("❌ Unsupported package manager: {}", pm);
            std::process::exit(1);
        }
    }
}
fn install_with_apt(cmd: &str) {
    run_shell_command(&format!("sudo apt update && sudo apt install {} -y", cmd));
}

fn install_with_dnf(cmd: &str) {
    run_shell_command(&format!("sudo dnf install {} -y", cmd));
}

fn install_with_pacman(cmd: &str) {
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

fn install_with_brew(cmd: &str, cask: bool) {
    if cask {
        run_shell_command(&format!("brew install --cask {}", cmd));
        return;
    }

    run_shell_command(&format!("brew install {}", cmd));
}