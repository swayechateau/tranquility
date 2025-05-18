// src/os/linux/fedora.rs
use std::process::Command;

pub fn install() {
    // check if dnf is installed
    if Command::new("dnf").arg("--version").status().is_ok() {
        println!("✅ dnf is installed.");
    } else {
        println!("❌ dnf is not installed, please install to continue.");
    }
}