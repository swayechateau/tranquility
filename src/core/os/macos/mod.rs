// src/os/macos/mod.rs
use crate::common::{check_command, run_shell_command};

// ────────────── macOS ──────────────
pub fn install() {
    install_xcode_cli();
}

fn install_xcode_cli() {
    // Install Xcode Cli
    if !check_command("xcode-select", "Xcode Cli",true) {
        println!("Installing Xcode Command Line Tools...");
        run_shell_command("xcode-select --install");
    }
}
