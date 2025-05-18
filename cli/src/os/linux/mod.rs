use std::process::Command;
use dialoguer::Confirm;

pub mod fedora;

// ────────────── Linux ──────────────
pub fn install() {
    // Check if Nix is installed
    if Command::new("nix").arg("--version").status().is_ok() {
        println!("✅ Nix is installed.");
    } else {
        println!("❌ Nix is not installed.");
        install_nix();
    }

    // check if snap is installed
    if Command::new("snap").arg("--version").status().is_ok() {
        println!("✅ snap is installed.");
    } else {
        println!("❌ snap is not installed.");
        install_snap();
    }

    // check if flatpak is installed
    if Command::new("flatpak").arg("--version").status().is_ok() {
        println!("✅ flatpak is installed.");
    } else {
        println!("❌ flatpak is not installed.");
        install_flatpak();
    }

    // check if shell is installed
    install_shell();

    // Check the distribution
    let distro = determine_distro();
    match distro.as_str() {
        "Ubuntu" => install_ubuntu(),
        "Debian" => install_debian(),
        "Fedora" => install_fedora(),
        "Arch" => install_arch(),
        _ => {
            println!("❌ Unsupported distribution: {}", distro);
            std::process::exit(1);
        }
    }
}

fn determine_distro() -> String {
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

pub fn run_shell_command(command: &str) {
    println!("🚀 Running shell command: {}", command);
    if let Err(err) = Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()
    {
        eprintln!("❌ Failed to execute command: {}\n{}", command, err);
        std::process::exit(1);
    }
}

// ────────────── Installers ──────────────
fn install_nix() {
    let mut cmd = "sh <(curl --proto '=https' --tlsv1.2 -L https://nixos.org/nix/install)".to_string();

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

    run_shell_command(&cmd);
}

fn install_snap() {
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

fn install_flatpak() {
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

fn install_shell() {
    // check if zsh is installed
    if Command::new("zsh").arg("--version").status().is_ok() {
        println!("✅ zsh is installed.");
    } else {
        println!("❌ zsh is not installed.");
        install_zsh();
    }

    // check if fish is installed
    if Command::new("fish").arg("--version").status().is_ok() {
        println!("✅ fish is installed.");
    } else {
        println!("❌ fish is not installed.");
        install_fish();
    }
}

fn install_zsh() {
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

fn install_fish() {
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

// ────────────── Distro Specific ──────────────
fn install_ubuntu() {
    println!("📦 Detected Ubuntu. You could install something here...");
    // Add real install logic here
}
fn install_debian() {
    println!("📦 Detected Debian.");
}
fn install_fedora() {
    println!("📦 Detected Fedora.");
    fedora::install();
}
fn install_arch() {
    println!("📦 Detected Arch.");
}
