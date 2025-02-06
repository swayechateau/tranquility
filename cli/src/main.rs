use which::which;

fn command_exists(cmd: &str) -> bool {
    which(cmd).is_ok()
}

fn get_os() -> &'static str {
    // Get the OS using Rust's built-in constant.
    std::env::consts::OS
}

fn package_manager(os: &str) -> &'static str {
    // Determine the package manager based on the OS.
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

fn main() {
    // Get the OS using Rust's built-in constant.
    let os = get_os();
    println!("Operating System: {}", os);

    // Determine the package manager based on the OS.
    let package_manager = package_manager(os);

    println!("Package Manager: {}", package_manager);
}
