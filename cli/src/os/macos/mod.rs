// src/os/macos/mod.rs
use crate::common::{check_default_pm, install_package_manager};

// ────────────── macOS ──────────────
pub fn install() {
    check_default_pm();
    install_package_manager();
}
