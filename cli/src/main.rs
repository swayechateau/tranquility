// src/main.rs
mod os;
mod common;

use common::{check_default_pm, determine_arch, determine_os, install_package_manager, install_shell};
use os::linux;
use os::macos;
use os::windows;

fn main() {
    // check for the operating system, architecture, and package manager
    let os = determine_os();
    let arch = determine_arch();

    println!("🖥️ Operating System: {}", os);
    println!("⚙️ Architecture: {}", arch);

    // proceed with installation
    install(&os);
}

fn install(os: &str) {
    // recommended checks before proceeding
    check_default_pm();
    install_package_manager();
    install_shell();

    match os {
        "Linux" => linux::install(),
        "macOS" => macos::install(),
        "Windows" => windows::install(),
        _ => {
            eprintln!("❌ Unsupported operating system: {}", os);
            std::process::exit(1);
        }
    }
}