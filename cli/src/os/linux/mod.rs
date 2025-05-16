use std::process::Command;
use dialoguer::Confirm;

// ────────────── Linux ──────────────
pub fn install() {
    // Check if Nix is installed
    if Command::new("nix").arg("--version").status().is_ok() {
        println!("✅ Nix is already installed.");
    } else {
        println!("❌ Nix is not installed.");
        install_nix();
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

fn install_ubuntu() {
    println!("📦 Detected Ubuntu. You could install something here...");
    // Add real install logic here
}
fn install_debian() {
    println!("📦 Detected Debian.");
}
fn install_fedora() {
    println!("📦 Detected Fedora.");
}
fn install_arch() {
    println!("📦 Detected Arch.");
}
