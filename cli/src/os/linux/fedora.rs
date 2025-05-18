// src/os/linux/fedora.rs

pub fn install() {
    // check if dnf is installed
    if Command::new("dnf").arg("--version").status().is_ok() {
        println!("✅ dnf is already installed.");
    } else {
        println!("❌ dnf is not installed, please install to continue.");
    }
}