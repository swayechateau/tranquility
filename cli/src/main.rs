use std::process::Command;

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

fn main() {
    // Get the OS using Rust's built-in constant.
    let os = detect_os();
    println!("Operating System: {}", os);

    // Determine the package manager based on the OS.
    let package_manager = detect_package_manager(os);

    println!("Package Manager: {}", package_manager);
}
