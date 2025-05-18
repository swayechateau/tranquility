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
