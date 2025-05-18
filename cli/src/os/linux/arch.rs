// src/os/linux/arch.rs

use std::process::Command;

pub fn install() {
    // check if pacman is installed
    if Command::new("pacman").arg("--version").status().is_ok() {
        println!("✅ pacman is installed.");
    } else {
        println!("❌ pacman is not installed, please install to continue.");
    }
}
