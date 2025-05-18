// src/os/linux/debian.rs

use std::process::Command;

pub fn install() {
    // check if apt is installed
    if Command::new("apt").arg("--version").status().is_ok() {
        println!("✅ apt is installed.");
    } else {
        println!("❌ apt is not installed, please install to continue.");
    }
}
