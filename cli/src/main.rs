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
fn prompt_select_package_manager() {
    println!("No known package manager detected. Would you like to install one? (y/N): ");
    // Flush stdout to ensure the prompt appears before waiting for input.
    let input = read_input();
    let answer = input.trim().to_lowercase();
    
    if answer == "y" || answer == "yes" {
        println!("Installation process initiated... (this is a placeholder)");
        select_package_manager(detect_os());
    } else {
        println!("No package manager will be installed. Exiting.");
        std::process::exit(0);
    }

}

/// Prompts the user to select a package manager to install.
fn select_package_manager(os: &str) {
    println!("Select a package manager to install:");
    match os {
        "linux" => {
            println!("1. apt");
            println!("2. yum");
            println!("3. dnf");
            println!("4. pacman");
        },
        "macos" => {
            println!("1. brew");
        },
        "windows" => {
            println!("1. choco");
            println!("2. winget");
        },
        _ => {
            eprintln!("Unsupported operating system.");
            std::process::exit(1);
        },
    }

    let input = read_input();
    let selection = input.trim().parse::<u8>().unwrap();

    match os {
        "linux" => {
            match selection {
                1 => install_package_manager(os, "apt"),
                2 => install_package_manager(os, "yum"),
                3 => install_package_manager(os, "dnf"),
                4 => install_package_manager(os, "pacman"),
                _ => {
                    eprintln!("Invalid selection.");
                    std::process::exit(1);
                },
            }
        },
        "macos" => {
            match selection {
                1 => install_package_manager(os, "brew"),
                _ => {
                    eprintln!("Invalid selection.");
                    std::process::exit(1);
                },
            }
        },
        "windows" => {
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
            eprintln!("Unsupported operating system.");
            std::process::exit(1);
        },
    }
}

// Package manager installation functions.
fn install_package_manager(os: &str, package_manager: &str) {
    match os {
        "linux" => {
            match package_manager {
                "apt" => {
                    println!("Installing using apt...");

                },
                "yum" => {
                    println!("Installing using yum...");
                    // Add your yum package installation command here.
                },
                "dnf" => {
                    println!("Installing using dnf...");
                    // Add your dnf package installation command here.
                },
                "pacman" => {
                    println!("Installing using pacman...");
                    // Add your pacman package installation command here.
                },
                _ => {
                    println!("Unsupported package manager.");
                    std::process::exit(1);
                },
            }
        },
        "macos" => {
            match package_manager {
                "brew" => {
                    println!("Installing using brew...");

                },
                _ => {
                    println!("Unsupported package manager.");
                    std::process::exit(1);
                },
            }
        },
        "windows" => {
            match package_manager {
                "choco" => {
                    println!("Installing using choco...");
                    // Add your choco package installation command here.
                },
                "winget" => {
                    println!("Installing using winget...");
                    // Add your winget package installation command here.
                },
                _ => {
                    println!("Unsupported package manager.");
                    std::process::exit(1);
                },
            }
        },
        _ => {
            println!("Unsupported operating system.");
            std::process::exit(1);
        },
    }
}

// Main function to run the code.
fn main() {
    // Get the OS using Rust's built-in constant.
    let detected_os = detect_os();
    if detected_os != "linux" && detected_os != "macos" && detected_os != "windows" {
        eprintln!("Your operating system is unsupported.");
        eprintln!("Supported operating systems are Linux, macOS, and Windows.");
        eprintln!("Exiting...");
        std::process::exit(1);
    }

    println!("Operating System: {}", detected_os);


    // Determine the package manager based on the OS.
    let detected_pm = detect_package_manager(detected_os);
    // let detected_pm = detect_package_manager("linux");

    if detected_pm == "unknown" {
        prompt_select_package_manager();
    }

    println!("Package Manager: {}", detected_pm);
}
