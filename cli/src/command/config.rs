// src/command/config.rs
use std::{fs, io};
use std::path::Path;

use crate::config::TranquilityConfig;
use crate::{log_error, log_info, print_success};
use crate::schema::config::validate_file as config_validate_file;
use crate::schema::vps::validate_file as vps_validate_file;
use crate::schema::application::validate_file as application_validate_file;

pub fn config_command_reset() {
    TranquilityConfig::reset().expect("Failed to reset config");
    log_info!("reset", "config", "✅ reinitialized");
    print_success!("Config reinitialized");
}

pub fn config_command_override_application_file(path: &Path) {

    if let Err(e) = override_application_file(path) {
        log_error!("override", "application_config_file", &format!("❌ Failed to override application config file: {e}"));
    }
}

pub fn config_command_override_config_file(path: &Path) {
    if let Err(e) = override_file(path) {
        log_error!("override", "config_file", &format!("❌ Failed to override config file: {e}"));
    }
}

pub fn config_command_override_vps_file(path: &Path) {
    if let Err(e) = override_vps_file(path) {
        log_error!("override", "vps_config_file", &format!("❌ Failed to override VPS config file: {e}"));
    }
}


fn override_application_file(path: &Path) -> io::Result<()> {
    let config = get_config();
    application_validate_file(path.to_str().unwrap()).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("❌ Validation failed:\n{}", e))
    })?;

    fs::copy(path, &config.applications_file)?;
    println!("✅ Overrode applications file with {:?}", path);
    Ok(())
}

fn override_file(path: &Path) -> io::Result<()> {
    config_validate_file(path.to_str().unwrap()).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("❌ Validation failed:\n{}", e))
    })?;

    fs::copy(path, &get_config().config_file())?;
    println!("✅ Overrode config file with {:?}", path);
    Ok(())
}

fn override_vps_file(path: &Path) -> io::Result<()> {
    let config = get_config();
    vps_validate_file(path.to_str().unwrap()).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("❌ Validation failed:\n{}", e))
    })?;

    fs::copy(path, &config.vps_file)?;
    println!("✅ Overrode VPS file with {:?}", path);
    Ok(())
}

fn get_config() -> TranquilityConfig {
    TranquilityConfig::load_or_init().expect("Failed to load config")
}