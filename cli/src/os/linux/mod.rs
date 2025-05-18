use crate::common::{determine_distro, install_package_manager, install_shell};

pub mod fedora;
pub mod ubuntu;
pub mod debian;
pub mod arch;

// ────────────── Linux ──────────────
pub fn install() {
    // check package managers are installed
    install_package_manager();
    // check if shell is installed
    install_shell();

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

// ────────────── Distro Specific ──────────────
fn install_ubuntu() {
    println!("📦 Detected Ubuntu.");
    ubuntu::install();
}
fn install_debian() {
    println!("📦 Detected Debian.");
    debian::install();
}
fn install_fedora() {
    println!("📦 Detected Fedora.");
    fedora::install();
}
fn install_arch() {
    println!("📦 Detected Arch.");
    arch::install();
}
