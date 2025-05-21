// src/command/init.rs
use crate::{config::TranquilityConfig, print_success};
use std::path::PathBuf;

pub fn init_command(reset: bool, config_path: &PathBuf) {
    if reset {
        TranquilityConfig::reset().expect("Failed to reset config");
        print_success!("✅ Config reset to default at {}", config_path.display());
    } else {
        TranquilityConfig::load_or_init().expect("Failed to initialize config");
        print_success!("✅ Config initialized at {}", config_path.display());
    }
}
