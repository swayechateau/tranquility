use std::env;

mod os;

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

fn determine_os() -> String {
    if cfg!(target_os = "windows") {
        "Windows".to_string()
    } else if cfg!(target_os = "macos") {
        "macOS".to_string()
    } else if cfg!(target_os = "linux") {
        "Linux".to_string()
    } else {
        "Unknown".to_string()
    }
}

fn determine_arch() -> String {
    env::consts::ARCH.to_string()
}
