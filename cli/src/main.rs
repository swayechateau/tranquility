use std::env;
use std::process::Command;
use std::io::{self, Write};

fn main() {
    // Determine OS and package manager.
    let mut detected_os = detect_os();
    if detected_os != "linux" && detected_os != "macos" && detected_os != "windows" {
        eprintln!("Your operating system is unsupported.");
        eprintln!("Supported operating systems are Linux, macOS, and Windows.");
        std::process::exit(1);
    }
    if detected_os == "windows" && !check_wsl2_installed() {
        // ask user if they want to install WSL 2
        if prompt_default_y("Do you want to install WSL 2? [Y/n] ") {
            install_wsl2();
        }
    }
    println!("Operating System: {}", detected_os);

    let mut detected_pm = detect_package_manager(detected_os);
    if detected_os == "linux" && detected_pm == "unknown" {
        eprintln!("No recognized package manager found on Linux. Your system is either unsupported for automatic installation or doesn't have a recognized package manager. Please install one manually.");
        std::process::exit(1);
    }

    if detected_pm == "unknown" {
        prompt_select_package_manager();
        detected_os = detect_os();
        detected_pm = detect_package_manager(detected_os);
    }
    println!("Using the {} package manager...", detected_pm);

    // Check command-line arguments.
    let args = check_args();

    // Run the interactive installation.
    install_apps(detected_os, detected_pm, args);
}

/// Check command-line arguments and return a flag.
fn check_args() -> &'static str {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        for arg in &args[1..] {
            match arg.as_str() {
                "-a" => {
                    println!("Option -a is provided, installing everything!");
                    return "all";
                }
                "-b" => {
                    println!("Option -b is provided, for the brave souls, wanting only the core dev tools!");
                    return "base";
                }
                _ => {
                    println!("Unrecognized option: {}, you thought we wouldn't know", arg);
                    std::process::exit(1);
                }
            }
        }
    }
    println!("No optional arguments provided... I see, you wish for something custom!");
    "custom"
}

/// Checks whether a given command exists in PATH.
fn command_exists(cmd: &str) -> bool {
    let output = if cfg!(target_os = "windows") {
        Command::new("where").arg(cmd).output()
    } else {
        Command::new("which").arg(cmd).output()
    };
    output.map(|o| o.status.success() && !o.stdout.is_empty()).unwrap_or(false)
}

/// Returns the OS using Rust's built-in constant.
fn detect_os() -> &'static str {
    env::consts::OS
}

/// Determines the package manager based on the OS.
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

/// Reads input from the user and returns the trimmed string.
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// Prompts the user about installing a package manager.
fn prompt_select_package_manager() {
    println!("No known package manager detected. Would you like to install one? (y/N): ");
    let input = read_input();
    let answer = input.trim().to_lowercase();
    if answer == "y" || answer == "yes" {
        println!("Installation process initiated...");
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

/// Installs the chosen package manager.
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
/// Check if WSL 2 is installed on Windows.
fn check_wsl2_installed() -> bool {
    println!("Checking if WSL 2 is installed...");
    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-InputFormat", "None",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            "(Get-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux).State -eq 'Enabled'",
        ])
        .output();

    match output {
        Ok(output) if output.status.success() && String::from_utf8_lossy(&output.stdout).trim() == "True" => {
            println!("WSL 2 is already installed.");
            true
        }
        _ => {
            println!("WSL 2 is not installed.");
            false
        }
    }
}
/// Install WSL 2 on Windows.
fn install_wsl2() {
    println!("Installing WSL 2...");

    let commands = [
        "Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux -NoRestart",
        "Enable-WindowsOptionalFeature -Online -FeatureName VirtualMachinePlatform -NoRestart",
        "Restart-Computer -Confirm",
    ];

    for cmd in &commands {
        let status = Command::new("powershell")
            .args(&["-NoProfile", "-InputFormat", "None", "-ExecutionPolicy", "Bypass", "-Command", cmd])
            .status();
        if !status.unwrap().success() {
            eprintln!("Failed to execute command: {}", cmd);
            return;
        }
    }

    let wsl_update_path = "https://aka.ms/wsl2kernel";
    let wsl_update_package = format!("{}/wsl_update.msi", env::temp_dir().display());
    let status = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-InputFormat", "None",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            &format!("Invoke-WebRequest -Uri {} -OutFile {} -UseBasicParsing", wsl_update_path, wsl_update_package),
        ])
        .status();
    if !status.unwrap().success() {
        eprintln!("Failed to download WSL 2 Linux kernel update package.");
        return;
    }
    let status = Command::new("msiexec.exe")
        .args(&["/i", &wsl_update_package, "/qn"])
        .status();
    if !status.unwrap().success() {
        eprintln!("Failed to install WSL 2 Linux kernel update package.");
        return;
    }

    let status = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-InputFormat", "None",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            "wsl --set-default-version 2",
        ])
        .status();
    if !status.unwrap().success() {
        eprintln!("Failed to set WSL version to 2.");
        return;
    }

    println!("Select a Linux distribution to install:");
    println!("1. Ubuntu");
    println!("2. Debian");
    println!("3. Fedora");
    println!("4. openSUSE");
    println!("5. SLES");
    let input = read_input();
    let distro = match input.trim() {
        "1" => "Ubuntu",
        "2" => "Debian",
        "3" => "Fedora",
        "4" => "openSUSE",
        "5" => "SLES",
        _ => {
            eprintln!("No valid selection, using Ubuntu as Linux distribution.");
            "Ubuntu"
        },
    };
    let distro_url = format!("https://aka.ms/wsl-{}", distro);
    let distro_package = format!("{}/distro.appx", env::temp_dir().display());
    let status = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-InputFormat", "None",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            &format!("Invoke-WebRequest -Uri {} -OutFile {} -UseBasicParsing", distro_url, distro_package),
        ])
        .status();
    if !status.unwrap().success() {
        eprintln!("Failed to download Linux distribution package.");
        return;
    }
    let status = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-InputFormat", "None",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            &format!("Add-AppxPackage -Path {}", distro_package),
        ])
        .status();
    if !status.unwrap().success() {
        eprintln!("Failed to install Linux distribution package.");
        return;
    }

    let status = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-InputFormat", "None",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            &format!("wsl --set-default {}", distro),
        ])
        .status();
    if !status.unwrap().success() {
        eprintln!("Failed to set default WSL distribution.");
    }

    println!("WSL 2 installed successfully.");
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

/// Updates the package list for apt.
fn update_apt() {
    println!("Updating package list (apt)...");
    let status = Command::new("bash")
        .arg("-c")
        .arg("sudo apt update")
        .status();
    match status {
        Ok(status) if status.success() => println!("apt package list updated successfully."),
        _ => eprintln!("Failed to update apt package list."),
    }
}

/// Updates the package list for yum.
fn update_yum() {
    println!("Updating package list (yum)...");
    let status = Command::new("bash")
        .arg("-c")
        .arg("sudo yum check-update")
        .status();
    match status {
        Ok(status) if status.success() => println!("yum package list updated successfully."),
        _ => eprintln!("Failed to update yum package list."),
    }
}

/// Updates the package list for dnf.
fn update_dnf() {
    println!("Updating package list (dnf)...");
    let status = Command::new("bash")
        .arg("-c")
        .arg("sudo dnf check-update")
        .status();
    match status {
        Ok(status) if status.success() => println!("dnf package list updated successfully."),
        _ => eprintln!("Failed to update dnf package list."),
    }
}

/// Updates the package list for pacman.
fn update_pacman() {
    println!("Updating package list (pacman)...");
    let status = Command::new("bash")
        .arg("-c")
        .arg("sudo pacman -Sy")
        .status();
    match status {
        Ok(status) if status.success() => println!("pacman package list updated successfully."),
        _ => eprintln!("Failed to update pacman package list."),
    }
}

/// Updates Homebrew.
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

/// Updates Chocolatey.
fn update_choco() {
    println!("Updating package list (choco)...");
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
        Ok(status) if status.success() => println!("Chocolatey packages upgraded successfully."),
        _ => eprintln!("Failed to upgrade Chocolatey packages."),
    }
}

/// Updates Winget.
fn update_winget() {
    println!("Updating package list (winget)...");
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
        Ok(status) if status.success() => println!("Winget packages upgraded successfully."),
        _ => eprintln!("Failed to upgrade Winget packages."),
    }
}

/// Updates the package manager by delegating to the proper update function.
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

/// Installs required packages for all platforms.
fn install_required_packages(os: &str, pm: &str) {
    println!("Installing required packages...");
    let shared = vec!["git", "curl", "wget"];
    let packages = match os {
        "macos" => shared,
        "linux" => shared,
        "windows" => shared,
        _ => vec![],
    };
    for package in packages {
        pkg_man_install(pm, package);
    }
}

/// Runs a command with arguments.
fn run_command(cmd: &str, args: &[&str]) {
    println!("Running command: {} {:?}", cmd, args);
    let status = Command::new(cmd)
        .args(args)
        .status()
        .expect("Failed to execute command");
    if !status.success() {
        eprintln!("Command {:?} failed", cmd);
    }
}

/// Installs a package using the detected package manager.
fn pkg_man_install(pm: &str, package: &str) {
    match pm {
       "apt" => run_command("sudo", &["apt", "install", "-y", package]),
       "yum" => run_command("sudo", &["yum", "install", "-y", package]),
       "dnf" => run_command("sudo", &["dnf", "install", "-y", package]),
       "pacman" => run_command("sudo", &["pacman", "-S", "--noconfirm", package]),
       "brew" => run_command("brew", &["install", package]),
       "choco" => run_command("choco", &["install", package, "-y"]),
       "winget" => run_command("winget", &["install", package, "-e"]),
       _ => {
            eprintln!("Unsupported package manager: {}", pm);
            std::process::exit(1);
       }
    }
}

/// Opens a URL using the default browser.
fn open_url(url: &str) {
    if cfg!(target_os = "macos") {
        run_command("open", &[url]);
    } else if cfg!(target_os = "windows") {
        // On Windows, use "cmd /C start" to open a URL.
        run_command("cmd", &["/C", "start", url]);
    } else {
        run_command("xdg-open", &[url]);
    }
}

/// Prompts the user with a question (default Yes).
fn prompt_default_y(prompt: &str) -> bool {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    let answer = answer.trim();
    answer.is_empty() || answer.to_lowercase().starts_with('y')
}

/// Prompts the user with a question (default No).
fn prompt_default_n(prompt: &str) -> bool {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    let answer = answer.trim();
    answer.to_lowercase().starts_with('y')
}

/// Interactive installation flow for various application categories.
fn install_apps(os: &str, pm: &str, _: &str) {
    // First, update the package manager and install required packages.
    update_package_manager(pm);
    install_required_packages(os, pm);

    // macOS-specific installations.
    if os == "macos" {
        if prompt_default_y("Do you want to install Xcode command line tools? [Y/n] ") {
            println!("After installing the command line tools, please install Xcode from the App Store.");
            run_command("xcode-select", &["--install"]);
        }
        if prompt_default_y("Do you want to add fonts to Homebrew? [Y/n] ") {
            run_command("brew", &["tap", "homebrew/homebrew-cask"]);
        }
    }

    // Shell installation.
    if prompt_default_y("Do you want to install an additional shell? [Y/n] ") {
        if prompt_default_y("Do you want to install the fish shell? [Y/n] ") {
            println!("Installing fish...");
            pkg_man_install(pm, "fish");
        }
        // install zsh
        if prompt_default_y("Do you want to install the zsh shell? [Y/n] ") {
            println!("Installing zsh...");
            pkg_man_install(pm, "zsh");
        }
    }
    
    // let cli_list = [
    //     "git",
    //     "wget",
    //     "curl",
    //     "tmux",
    //     "terraform",
    //     "awscli",
    //     "deno",
    //     "node",
    //     "yarn",
    //     "jq",
    //     "go",
    //     "dotnet-sdk",
    //     "elixir",
    //     "python",
    //     "rust",
    //     "php",
    //     "ruby",
    // ];
    // CLI Tools installation.
    if prompt_default_y("Do you want to install CLI tools? [Y/n] ") {
        if prompt_default_y("Do you want to install git? [Y/n] ") {
            pkg_man_install(pm, "git");
        }
        if prompt_default_y("Do you want to install wget? [Y/n] ") {
            pkg_man_install(pm, "wget");
        }
        if prompt_default_y("Do you want to install curl? [Y/n] ") {
            pkg_man_install(pm, "curl");
        }
        if prompt_default_y("Do you want to install tmux? [Y/n] ") {
            pkg_man_install(pm, "tmux");
        }
        if prompt_default_y("Do you want to install terraform? [Y/n] ") {
            pkg_man_install(pm, "terraform");
        }
        if prompt_default_y("Do you want to install awscli? [Y/n] ") {
            pkg_man_install(pm, "awscli");
        }
        if prompt_default_y("Do you want to install deno? [Y/n] ") {
            pkg_man_install(pm, "deno");
        }
        if prompt_default_y("Do you want to install node (nodejs and npm)? [Y/n] ") {
            pkg_man_install(pm, "node");
        }
        if prompt_default_y("Do you want to install yarn? [Y/n] ") {
            pkg_man_install(pm, "yarn");
        }
        if prompt_default_y("Do you want to install jq? [Y/n] ") {
            pkg_man_install(pm, "jq");
        }
        if prompt_default_y("Do you want to install go? [Y/n] ") {
            pkg_man_install(pm, "go");
        }
        if prompt_default_y("Do you want to install dotnet-sdk? [Y/n] ") {
            pkg_man_install(pm, "dotnet-sdk");
        }
        if prompt_default_y("Do you want to install elixir? [Y/n] ") {
            pkg_man_install(pm, "elixir");
        }
        if prompt_default_n("Do you want to install python and pip? [y/N] ") {
            pkg_man_install(pm, "python");
        }
        if prompt_default_y("Do you want to install rust and cargo? [Y/n] ") {
            run_command("sh", &["-c", "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"]);
        }
        if prompt_default_n("Do you want to install php and composer? [y/N] ") {
            pkg_man_install(pm, "php");
        }
        if prompt_default_n("Do you want to install ruby and gem? [y/N] ") {
            pkg_man_install(pm, "ruby");
        }

        if prompt_default_y("Do you want to install docker cli? [Y/n] ") {
            pkg_man_install(pm, "docker");
        }

        if prompt_default_y("Do you want to install podman cli and podman? [Y/n] ") {
            pkg_man_install(pm, "podman");
            pkg_man_install(pm, "podman-compose");
        }
 
    }

    // Editors and IDEs installation.
    if prompt_default_y("Do you want to install an editor? [Y/n] ") {
        if prompt_default_y("Do you want to install neovim? [Y/n] ") {
            pkg_man_install(pm, "neovim");
        }
        if prompt_default_y("Do you want to install Visual Studio Code? [Y/n] ") {
            if pm == "brew" {
                run_command("brew", &["install", "--cask", "visual-studio-code"]);
            } else {
                pkg_man_install(pm, "visual-studio-code");
            }
        }
        if prompt_default_y("Do you want to install Visual Studio? [Y/n] ") {
            if pm == "brew" {
                run_command("brew", &["install", "--cask", "visual-studio"]);
            } else {
                pkg_man_install(pm, "visual-studio");
            }
        }
        if prompt_default_n("Do you want to install Sublime Text? [y/N] ") {
            if pm == "brew" {
                run_command("brew", &["install", "--cask", "sublime-text"]);
            } else {
                pkg_man_install(pm, "sublime-text");
            }
        }
        if prompt_default_n("Do you want to install Atom? [y/N] ") {
            if pm == "brew" {
                run_command("brew", &["install", "--cask", "atom"]);
            } else {
                pkg_man_install(pm, "atom");
            }
        }
        if prompt_default_n("Do you want to install Brackets? [y/N] ") {
            if pm == "brew" {
                run_command("brew", &["install", "--cask", "brackets"]);
            } else {
                pkg_man_install(pm, "brackets");
            }
        }
        if prompt_default_n("Do you want to install Eclipse? [y/N] ") {
            if pm == "brew" {
                run_command("brew", &["install", "--cask", "eclipse-java"]);
            } else {
                pkg_man_install(pm, "eclipse-java");
            }
        }
        if prompt_default_n("Do you want to install IntelliJ IDEA Community Edition? [y/N] ") {
            if pm == "brew" {
                run_command("brew", &["install", "--cask", "intellij-idea-ce"]);
            } else {
                pkg_man_install(pm, "intellij-idea-ce");
            }
        }
    }
    
    // Dev GUI Tools (MAC/Windows)
    if prompt_default_y("Do you want to install dev GUI tools? [Y/n] ") {
        if prompt_default_y("Do you want to install docker desktop? [Y/n] ") {
            if pm == "brew" {
                run_command("brew", &["install", "--cask", "docker"]);
                run_command("brew", &["install", "orbstack"]);
            } else {
                pkg_man_install(pm, "docker");
            }
        }
        if prompt_default_y("Do you want to install github desktop? [Y/n] ") {
            if pm == "brew" {
                run_command("brew", &["install", "--cask", "github"]);
            } else {
                pkg_man_install(pm, "github");
            }
        }
    }
    // Mobile Development
    if prompt_default_y("Do you want to install mobile development tools? [Y/n] ") {
        if prompt_default_y("Do you want to install android studio? [Y/n] ") {
            if pm == "brew" {
                run_command("brew", &["install", "--cask", "android-studio"]);
            } else {
                pkg_man_install(pm, "android-studio");
            }
        }
        if prompt_default_y("Do you want to install xcode? [Y/n] ") {
            println!("Please install xcode from the app store. https://apps.apple.com/us/app/xcode/id497799835?mt=12");
            open_url("https://apps.apple.com/us/app/xcode/id497799835?mt=12");
        }
    }

    // Virtualization
    if prompt_default_y("Do you want to install virtualization tools? [Y/n] ") {
        if prompt_default_y("Do you want to install virtualbox? [Y/n] ") {
            pkg_man_install(pm, "virtualbox");
        }
        if prompt_default_y("Do you want to install vagrant? [Y/n] ") {
            pkg_man_install(pm, "vagrant");
        }
    }

    // Browser Installation
    if prompt_default_y("Do you want to install a browser? [Y/n] ") {
        if prompt_default_y("Do you want to install Brave browser? [Y/n] ") {
            pkg_man_install(pm, "brave-browser");
        }
        if prompt_default_y("Do you want to install Opera browser? [Y/n] ") {
            if prompt_default_y("Install Standard Opera? [Y/n] ") {
                pkg_man_install(pm, "opera");
            }
            if prompt_default_y("Install OperaGX? [Y/n] ") {
                pkg_man_install(pm, "opera-gx");
            }
        }
        if prompt_default_y("Do you want to install Vivaldi browser? [Y/n] ") {
            if prompt_default_y("Install Standard Vivaldi? [Y/n] ") {
                pkg_man_install(pm, "vivaldi");
            }
        }
        if prompt_default_y("Do you want to install Tor browser? [Y/n] ") {
            pkg_man_install(pm, "tor-browser");
        }
        if prompt_default_y("Do you want to install Google Chrome browser? [Y/n] ") {
            if prompt_default_y("Install Standard Google Chrome? [Y/n] ") {
                pkg_man_install(pm, "google-chrome");
            }
        }
        if prompt_default_y("Do you want to install Microsoft Edge browser? [Y/n] ") {
            if prompt_default_y("Install Standard Microsoft Edge? [Y/n] ") {
                pkg_man_install(pm, "microsoft-edge");
            }
        }
        if prompt_default_y("Do you want to install Firefox browser? [Y/n] ") {
            if prompt_default_y("Install Standard Firefox? [Y/n] ") {
                pkg_man_install(pm, "firefox");
            }
        }
        if prompt_default_y("Do you want to install Polypane browser? [Y/n] ") {
            if pm == "brew" {
                run_command("brew", &["install", "--cask", "polypane"]);
            } else {
                pkg_man_install(pm, "polypane");
            }
        }
    }
    
    // Chat Application
    if prompt_default_y("Do you want to install chat applications? [Y/n] ") {
        if prompt_default_y("Do you want to install Microsoft Teams? [Y/n] ") {
            if pm == "brew" {
                run_command("brew", &["install", "--cask", "microsoft-teams"]);
            } else {

            }
        }
        if prompt_default_y("Do you want to install slack? [Y/n] ") {
            pkg_man_install(pm, "slack");
        }
        if prompt_default_y("Do you want to install discord? [Y/n] ") {
            pkg_man_install(pm, "discord");
        }
        if prompt_default_y("Do you want to install telegram-desktop? [Y/n] ") {
            pkg_man_install(pm, "telegram-desktop");
        }
        if prompt_default_y("Do you want to install line? [Y/n] ") {
            pkg_man_install(pm, "line");
        }
        if prompt_default_y("Do you want to install wechat? [Y/n] ") {
            pkg_man_install(pm, "wechat");
        }
        if prompt_default_y("Do you want to install kakaotalk? [Y/n] ") {
            pkg_man_install(pm, "kakaotalk");
        }
        if prompt_default_y("Do you want to install WhatsApp? [Y/n] ") {
            pkg_man_install(pm, "whatsapp");
        }
    }

    // Games Development Tools
    if prompt_default_y("Do you want to install games development tools? [Y/n] ") {
        if prompt_default_y("Do you want to install unity? [Y/n] ") {
            pkg_man_install(pm, "unity");
        }
        if prompt_default_n("Do you want to install unreal-engine? [y/N] ") {
            pkg_man_install(pm, "unreal-engine");
        }
        if prompt_default_y("Do you want to install godot? [Y/n] ") {
            pkg_man_install(pm, "godot");
        }
    }

    // Graphics Editor Tools
    if prompt_default_y("Do you want to install graphics editor tools? [Y/n] ") {
        if prompt_default_y("Do you want to install gimp? [Y/n] ") {
            pkg_man_install(pm, "gimp");
        }
        if prompt_default_y("Do you want to install inkscape? [Y/n] ") {
            pkg_man_install(pm, "inkscape");
        }
        if prompt_default_y("Do you want to install krita? [Y/n] ") {
            pkg_man_install(pm, "krita");
        }
        if prompt_default_y("Do you want to install blender? [Y/n] ") {
            pkg_man_install(pm, "blender");
        }
        if prompt_default_y("Do you want to install staruml? [Y/n] ") {
            pkg_man_install(pm, "staruml");
        }
    }

    // Video Editor Tools
    if prompt_default_y("Do you want to install video editor tools? [Y/n] ") {
        if prompt_default_y("Do you want to install obs? [Y/n] ") {
            pkg_man_install(pm, "obs");
        }
        if prompt_default_y("Do you want to install davinci-resolve? [Y/n] ") {
            pkg_man_install(pm, "davinci-resolve");
        }
        if prompt_default_y("Do you want to install handbrake? [Y/n] ") {
            pkg_man_install(pm, "handbrake");
        }
        if prompt_default_y("Do you want to install vlc? [Y/n] ") {
            pkg_man_install(pm, "vlc");
        }
    }
    
    // Audio Editor Tools
    if prompt_default_y("Do you want to install audio editor tools? [Y/n] ") {
        if prompt_default_y("Do you want to install audacity? [Y/n] ") {
            pkg_man_install(pm, "audacity");
        }
        if prompt_default_n("Do you want to install lmms? [y/N] ") {
            pkg_man_install(pm, "lmms");
        }
        if prompt_default_n("Do you want to install reaper? [y/N] ") {
            pkg_man_install(pm, "reaper");
        }
    }

    // Music Apps
    if prompt_default_y("Do you want to install music apps? [Y/n] ") {
        if prompt_default_y("Do you want to install spotify? [Y/n] ") {
            pkg_man_install(pm, "spotify");
        }
    }

    // Database Management Tools
    if prompt_default_y("Do you want to install database management tools? [Y/n] ") {
        if prompt_default_y("Do you want to install dbeaver-community? [Y/n] ") {
            pkg_man_install(pm, "dbeaver-community");
        }
        if prompt_default_y("Do you want to install mongodb-compass? [Y/n] ") {
            pkg_man_install(pm, "mongodb-compass");
        }
    }

    // VPN Tools
    if prompt_default_y("Do you want to install vpn tools? [Y/n] ") {
        if prompt_default_y("Do you want to install surfshark? [Y/n] ") {
            pkg_man_install(pm, "surfshark");
        }
        if prompt_default_y("Do you want to install openvpn-connect? [Y/n] ") {
            pkg_man_install(pm, "openvpn-connect");
        }
    }

    // REST client tools
    if prompt_default_y("Do you want to install REST client tools? [Y/n] ") {
        if prompt_default_y("Do you want to install insomnia? [Y/n] ") {
            pkg_man_install(pm, "insomnia");
        }
        if prompt_default_y("Do you want to install postman? [Y/n] ") {
            pkg_man_install(pm, "postman");
        }
    }

    // Download Manager Tools
    if prompt_default_y("Do you want to install download manager tools? [Y/n] ") {
        if prompt_default_y("Do you want to install jdownloader? [Y/n] ") {
            pkg_man_install(pm, "jdownloader");
        }
    }
    
    // Note Taking Tools
    if prompt_default_y("Do you want to install note taking tools? [Y/n] ") {
        if prompt_default_y("Do you want to install Notion? [Y/n] ") {
            pkg_man_install(pm, "notion");
        }
        if prompt_default_y("Do you want to install Typora? [Y/n] ") {
            pkg_man_install(pm, "typora");
        }
        if prompt_default_y("Do you want to install Obsidian? [Y/n] ") {
            pkg_man_install(pm, "obsidian");
        }
    }
    
    // Password Manager Tools
    if prompt_default_y("Do you want to install password manager tools? [Y/n] ") {
        if prompt_default_n("Do you want to install Bitwarden? [y/N] ") {
            pkg_man_install(pm, "bitwarden");
        }
        if prompt_default_y("Do you want to install KeePassXC? [Y/n] ") {
            pkg_man_install(pm, "keepassxc");
        }
    }
    
    // Gaming Clients
    if prompt_default_y("Do you want to install gaming clients? [Y/n] ") {
        if prompt_default_y("Do you want to install Steam? [Y/n] ") {
            pkg_man_install(pm, "steam");
        }
        if prompt_default_n("Do you want to install Epic Games? [y/N] ") {
            pkg_man_install(pm, "epic-games");
        }
        if prompt_default_y("Do you want to install GOG Galaxy? [Y/n] ") {
            pkg_man_install(pm, "gog-galaxy");
        }
        if prompt_default_y("Do you want to install Origin? [Y/n] ") {
            pkg_man_install(pm, "origin");
        }
        if prompt_default_y("Do you want to install Battle.net? [Y/n] ") {
            pkg_man_install(pm, "battle-net");
        }
        if prompt_default_y("Do you want to install PlayStation Remote Play? [Y/n] ") {
            pkg_man_install(pm, "sony-ps-remote-play");
        }
    }
    
    // Operating System Extensions
    if prompt_default_y("Do you want to install operating system extensions? [Y/n] ") {
        if prompt_default_y("Do you want to install copyq? [Y/n] ") {
            println!("Installing fish...");
            pkg_man_install(pm, "fish");
        }
        if prompt_default_y("Do you want to install rectangle? [Y/n] ") {
            println!("Installing rectangle...");
            pkg_man_install(pm, "rectangle");
        }
        if prompt_default_y("Do you want to install iterm2? [Y/n] ") {
            println!("Installing iterm2...");
            pkg_man_install(pm, "iterm2");
        }
    }

    println!("Installation complete.");
}
