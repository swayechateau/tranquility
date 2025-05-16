use std::process::Command;

// ────────────── macOS ──────────────
pub fn install() {
    // check if Homebrew is installed
    if Command::new("brew").arg("--version").status().is_ok() {
        println!("✅ Homebrew is already installed.");
    } else {
        println!("❌ Homebrew is not installed.");
        install_brew();
    }

    // check if Nix is installed
    if Command::new("nix").arg("--version").status().is_ok() {
        println!("✅ Nix is already installed.");
    } else {
        println!("❌ Nix is not installed.");
        install_nix();
    }

}

fn install_brew() {
    let cmd = "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"";
    run_shell_command(cmd);
}

fn install_nix() {
    let cmd = "sh <(curl --proto '=https' --tlsv1.2 -L https://nixos.org/nix/install)";
    run_shell_command(cmd);
}

fn run_shell_command(command: &str) {
    println!("🚀 Running: {}", command);
    if let Err(err) = Command::new("zsh")
        .arg("-c")
        .arg(command)
        .status()
    {
        eprintln!("❌ Failed to execute command: {}\n{}", command, err);
        std::process::exit(1);
    }
}
