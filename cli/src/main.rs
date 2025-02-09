use std::env;
use std::process::Command;
use std::io::{self};

/// Check args
fn check_args() -> &'static str{
    // Fetch the command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if optional arguments were provided
    if args.len() > 1 {
        // Process the optional arguments
        for arg in &args[1..] {
            match arg.as_str() {
                "-a" => {
                    // Handle the "-a" flag
                    println!("Option -a is provided, installing everything!");
                    "all";
                    break;
                }
                "-b" => {
                    // Handle the "-b" flag
                    println!("Option -b is provided, for the brave souls, wanting only the core dev tools!");
                    "base";
                    break;
                }
                _ => {
                    // Handle any other unrecognized options
                    println!("Unrecognized option: {}, you thought we wouldn't know", arg);
                    std::process::exit(1);
                    break;
                }
            }
        }
    } else {
        // No optional arguments provided
        println!("No optional arguments provided... I see, you wish for something custom!");
        "custom"
    }
}

/// Checks if a given command exists in the system's PATH.
fn command_exists(cmd: &str) -> bool {
    // Use 'where' on Windows and 'which' on Unix-like systems.
    let output = if cfg!(target_os = "windows") {
        Command::new("where").arg(cmd).output()
    } else {
        Command::new("which").arg(cmd).output()
    };

    output.map(|o| o.status.success() && !o.stdout.is_empty()).unwrap_or(false)
}

// Get the OS using Rust's built-in constant.
fn detect_os() -> &'static str {
    std::env::consts::OS
}

// Determine the package manager based on the OS.
fn detect_package_manager(os: &str) -> &'static str {
    match os {
        "linux" => {
            if command_exists("apt") {
                "apt"
            } else if command_exists("yum") {
                "yum"
            } else if command_exists("dnf") {
                "dnf"
            } else if command_exists("pacman") {
                "pacman"
            } else {
                "unknown"
            }
        },
        "macos" => {
            if command_exists("brew") {
                "brew"
            } else {
                "unknown"
            }
        },
        "windows" => {
            if command_exists("choco") {
                "choco"
            } else if command_exists("winget") {
                "winget"
            } else {
                "unknown"
            }
        },
        _ => "unknown",
    }
}

// Reads input from the user and trims it.
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// Prompts the user to decide if they wish to install a package manager.
/// This function is only used for Windows and macOS.
fn prompt_select_package_manager() {
    println!("No known package manager detected. Would you like to install one? (y/N): ");
    let input = read_input();
    let answer = input.trim().to_lowercase();
    
    if answer == "y" || answer == "yes" {
        println!("Installation process initiated...");
        // Use the detected OS to present installation options.
        select_package_manager(detect_os());
    } else {
        println!("No package manager will be installed. Exiting.");
        std::process::exit(0);
    }
}

/// Prompts the user to select a package manager to install (Windows and macOS only).
fn select_package_manager(os: &str) {
    match os {
        "macos" => {
            println!("Select a package manager to install:");
            println!("1. brew");
            let input = read_input();
            let selection = input.trim().parse::<u8>().unwrap();
            match selection {
                1 => install_package_manager(os, "brew"),
                _ => {
                    eprintln!("Invalid selection.");
                    std::process::exit(1);
                },
            }
        },
        "windows" => {
            println!("Select a package manager to install:");
            println!("1. choco");
            println!("2. winget");
            let input = read_input();
            let selection = input.trim().parse::<u8>().unwrap();
            match selection {
                1 => install_package_manager(os, "choco"),
                2 => install_package_manager(os, "winget"),
                _ => {
                    eprintln!("Invalid selection.");
                    std::process::exit(1);
                },
            }
        },
        _ => {
            eprintln!("Automatic installation is only supported on Windows and macOS.");
            std::process::exit(1);
        }
    }
}

// Package manager installation functions.
fn install_package_manager(os: &str, package_manager: &str) {
    match os {
        "macos" => {
            match package_manager {
                "brew" => install_brew(),
                _ => {
                    eprintln!("Unsupported package manager for macOS.");
                    std::process::exit(1);
                },
            }
        },
        "windows" => {
            match package_manager {
                "choco" => install_choco(),
                "winget" => install_winget(),
                _ => {
                    eprintln!("Unsupported package manager for Windows.");
                    std::process::exit(1);
                },
            }
        },
        _ => {
            eprintln!("Automatic installation is not supported on this OS.");
            std::process::exit(1);
        },
    }
}

/// Installs Homebrew on macOS.
fn install_brew() {
    println!("Installing Homebrew...");
    let status = Command::new("bash")
        .arg("-c")
        .arg("curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh | /bin/bash")
        .status();
    match status {
        Ok(status) if status.success() => println!("Homebrew installed successfully."),
        _ => eprintln!("Failed to install Homebrew."),
    }
}

/// Installs Chocolatey on Windows.
fn install_choco() {
    println!("Installing Chocolatey...");
    let status = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-InputFormat", "None",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            "Set-ExecutionPolicy Bypass -Scope Process -Force; \
             [System.Net.ServicePointManager]::SecurityProtocol = \
             [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; \
             iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))",
        ])
        .status();
    match status {
        Ok(status) if status.success() => println!("Chocolatey installed successfully."),
        _ => eprintln!("Failed to install Chocolatey."),
    }
}

/// Initiates Winget installation on Windows.
fn install_winget() {
    println!("Installing Winget...");
    let status = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-InputFormat", "None",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            "Start-Process ms-appinstaller:?source=https://winget.azureedge.net/cache",
        ])
        .status();
    match status {
        Ok(status) if status.success() => println!("Winget installation initiated successfully."),
        _ => eprintln!("Failed to initiate Winget installation."),
    }
}
// update command functions for each package manager
fn update_apt() {
    println!("Updating package list...");
    let status = Command::new("bash")
        .arg("-c")
        .arg("sudo apt update")
        .status();
    match status {
        Ok(status) if status.success() => println!("Package list updated successfully."),
        _ => eprintln!("Failed to update package list."),
    }
}

fn update_yum() {
    println!("Updating package list...");
    let status = Command::new("bash")
        .arg("-c")
        .arg("sudo yum check-update")
        .status();
    match status {
        Ok(status) if status.success() => println!("Package list updated successfully."),
        _ => eprintln!("Failed to update package list."),
    }
}

fn update_dnf() {
    println!("Updating package list...");
    let status = Command::new("bash")
        .arg("-c")
        .arg("sudo dnf check-update")
        .status();
    match status {
        Ok(status) if status.success() => println!("Package list updated successfully."),
        _ => eprintln!("Failed to update package list."),
    }
}

fn update_pacman() {
    println!("Updating package list...");
    let status = Command::new("bash")
        .arg("-c")
        .arg("sudo pacman -Sy")
        .status();
    match status {
        Ok(status) if status.success() => println!("Package list updated successfully."),
        _ => eprintln!("Failed to update package list."),
    }
}

fn update_brew() {
    println!("Updating Homebrew...");
    let status = Command::new("brew")
        .arg("update")
        .status();
    match status {
        Ok(status) if status.success() => println!("Homebrew updated successfully."),
        _ => eprintln!("Failed to update Homebrew."),
    }
}

fn update_choco() {
    println!("Updating package list...");
    let status = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-InputFormat", "None",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            "choco upgrade all -y",
        ])
        .status();
    match status {
        Ok(status) if status.success() => println!("Package list updated successfully."),
        _ => eprintln!("Failed to update package list."),
    }
}

fn update_winget() {
    println!("Updating package list...");
    let status = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-InputFormat", "None",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            "winget upgrade --all",
        ])
        .status();
    match status {
        Ok(status) if status.success() => println!("Package list updated successfully."),
        _ => eprintln!("Failed to update package list."),
    }
}

// Update the package manager.
fn update_package_manager(pm: &str) {
    println!("Updating the package manager...");
    match pm {
        "apt" => update_apt(),
        "yum" => update_yum(),
        "dnf" => update_dnf(),
        "pacman" => update_pacman(),
        "brew" => update_brew(),
        "choco" => update_choco(),
        "winget" => update_winget(),
        _ => {
            eprintln!("Unsupported package manager...");
            std::process::exit(1);
        },
    }
}

mac_required_packages = [
    "git",
    "curl",
    "zsh",
    "wget",
}

linux_required_packages = [
    "git",
    "curl",
    "zsh",
    "wget",
]

windows_required_packages = [
    "git",
    "curl",
    "zsh",
    "wget",
]

// Install the required packages.
fn install_required_packages(os: &str, pm: &str) {

}
// Return list of browsers and the supported os

// Additional functions can be added here.
fn install_apps(os: &str, pm: &str, args: &str) {
    update_package_manager(pm);
    // required packages
    install_required_packages(os, pm);
    // browsers
    // dev tools
    // editors
    // fonts
    // creative
    // productivity
    // communication
    // entertainment
    // security
    // other

}
// Main function to run the code.
fn main() {
    let detected_os = detect_os();
    
    // Check if the OS is supported.
    if detected_os != "linux" && detected_os != "macos" && detected_os != "windows" {
        eprintln!("Your operating system is unsupported.");
        eprintln!("Supported operating systems are Linux, macOS, and Windows.");
        std::process::exit(1);
    }
    
    println!("Operating System: {}", detected_os);
    
    let detected_pm = detect_package_manager(detected_os);
    
    if detected_os == "linux" && detected_pm == "unknown" {
        eprintln!("No recognized package manager found on Linux. Your system is either unsupported for automatic installation or doesn't have a recognized package manager. Please install one manually.");
        std::process::exit(1);
    }
    if detected_pm == "unknown" {
        prompt_select_package_manager();
        detected_os = detect_os();
    }
    // The package manager is known at this point.
    println!("Using the {} package manager...", detected_pm);
    // Check if any args were provided and process them.
    let args = check_args();
    install_apps(detected_os, detected_pm, args);
    
}
