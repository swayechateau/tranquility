use std::process::Command;
use std::io::{self};

/// Checks if a given command exists in the system's PATH.
fn command_exists(cmd: &str) -> bool {
    // Use 'where' on Windows and 'which' on Unix-like systems.
    let output = if cfg!(target_os = "windows") {
        Command::new("where").arg(cmd).output()
    } else {
        Command::new("which").arg(cmd).output()
    };

    // Consider the command found if the output is successful and non-empty.
    output.map(|o| o.status.success() && !o.stdout.is_empty()).unwrap_or(false)
}

// Get the OS using Rust's built-in constant.
fn detect_os() -> &'static str {
    std::env::consts::OS
}

// Determine the package manager based on the OS.
fn detect_package_manager(os: &str) -> &'static str {
    let package_manager = match os {
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
    };

    package_manager
}

// Functions to install package manager if unknow is returned.
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
/// Prompts the user to decide if they wish to install a package manager.
/// If the user confirms, prints a placeholder message.
fn prompt_install_package_manager() {
    println!("No known package manager detected. Would you like to install one? (y/N): ");
    // Flush stdout to ensure the prompt appears before waiting for input.
    let input = read_input();
    let answer = input.trim().to_lowercase();
    
    if answer == "y" || answer == "yes" {
        println!("Installation process initiated... (this is a placeholder)");
        // Here you can add logic to automate or instruct on installing a package manager.
    } else {
        println!("No package manager will be installed. Exiting.");
        std::process::exit(0);
    }

}

fn main() {
    // Get the OS using Rust's built-in constant.
    let detected_os = detect_os();
    if detected_os != "linux" && detected_os != "macos" && detected_os != "windows" {
        eprintln!("Your operating system is unsupported.");
        std::process::exit(1);
    }

    println!("Operating System: {}", detected_os);

    // Determine the package manager based on the OS.
    let detected_pm = detect_package_manager(detected_os);

    if detected_pm == "unknown" {
        prompt_install_package_manager();
    }

    println!("Package Manager: {}", detected_pm);
}
