// src/command/config.rs
use std::{fs, io};
use std::path::Path;

use crate::config::TranquilityConfig;
use crate::print_error;
use crate::schema::config::validate_file as config_validate_file;
use crate::schema::vps::validate_file as vps_validate_file;
use crate::schema::application::validate_file as application_validate_file;

pub fn override_application_config_file(path: &Path, config: &TranquilityConfig) {
    if let Err(e) = override_application_file(path, config) {
        print_error!("❌ Failed to override application config file: {e}");
    }
}

pub fn override_config_file(path: &Path, config_path: &Path) {
    if let Err(e) = override_file(path, config_path) {
        print_error!("❌ Failed to override config file: {e}");
    }
}

pub fn override_vps_config_file(path: &Path, config: &TranquilityConfig) {
    if let Err(e) = override_vps_file(path, config) {
        print_error!("❌ Failed to override VPS config file: {e}");
    }
}


fn override_application_file(path: &Path, config: &TranquilityConfig) -> io::Result<()> {
    application_validate_file(path.to_str().unwrap()).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("❌ Validation failed:\n{}", e))
    })?;

    fs::copy(path, &config.applications_file)?;
    println!("✅ Overrode applications file with {:?}", path);
    Ok(())
}

fn override_file(path: &Path, config_path: &Path) -> io::Result<()> {
    config_validate_file(path.to_str().unwrap()).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("❌ Validation failed:\n{}", e))
    })?;

    fs::copy(path, config_path)?;
    println!("✅ Overrode config file with {:?}", path);
    Ok(())
}

fn override_vps_file(path: &Path, config: &TranquilityConfig) -> io::Result<()> {
    vps_validate_file(path.to_str().unwrap()).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("❌ Validation failed:\n{}", e))
    })?;

    fs::copy(path, &config.vps_file)?;
    println!("✅ Overrode VPS file with {:?}", path);
    Ok(())
}