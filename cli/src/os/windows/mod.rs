use std::process::Command;

// ────────────── Windows ──────────────
pub fn install() {
    println!("🔧 Trying to install Winget on Windows...");
    install_winget();
    install_choco();
    install_scoop();
}

// Optional helpers for Windows (not runnable from non-admin CLI)
fn install_choco() {
    let cmd = "Set-ExecutionPolicy Bypass -Scope Process -Force; \
               iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))";
    run_powershell_command(cmd);
}

fn install_scoop() {
    let cmd = "iwr -useb get.scoop.sh | iex";
    run_powershell_command(cmd);
}

fn install_winget() {
    println!("📦 Winget comes pre-installed on Windows 10/11. If missing, install via Microsoft Store.");
}

fn run_powershell_command(command: &str) {
    if let Err(err) = Command::new("powershell")
        .arg("-Command")
        .arg(command)
        .status()
    {
        eprintln!("❌ PowerShell command failed: {}\n{}", command, err);
        std::process::exit(1);
    }
}
