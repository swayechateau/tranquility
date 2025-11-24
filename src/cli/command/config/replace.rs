use std::{fs, io, path::PathBuf};

use crate::{
    cli::command::config::get_config,
    config::{self, TranquilityConfig, deserialize_config},
    log_error, log_info,
    models::{application, vps},
};

pub fn replace_config(file: PathBuf) {
    if let Err(e) = override_file(file) {
        log_error!(
            "override",
            "config_file",
            &format!("❌ Failed to override config file: {e}")
        );
    }
}

pub fn replace_applications(file: PathBuf) {
    if let Err(e) = override_application_file(file) {
        log_error!(
            "override",
            "application_config_file",
            &format!("❌ Failed to override application config file: {e}")
        );
    }
}

pub fn replace_vps(file: PathBuf) {
    if let Err(e) = override_vps_file(file) {
        log_error!(
            "override",
            "vps_config_file",
            &format!("❌ Failed to override VPS config file: {e}")
        );
    }
}

fn override_application_file(path: PathBuf) -> io::Result<()> {
    let config = get_config();
    let file_path = &path;
    if !application::schema::validate_file(&path) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("❌ Validation failed for {}", path.display()),
        ));
    }

    fs::copy(file_path, &config.applications_file)?;
    log_info!(
        "override",
        "application_config_file",
        &format!("✅ Overrode applications file with {:?}", file_path)
    );
    Ok(())
}

pub fn override_vps_file(path: PathBuf) -> io::Result<()> {
    // Validate the file before replacing
    if !vps::schema::validate_file(&path) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("❌ Validation failed for file: {}", path.display()),
        ));
    }

    let config = get_config();

    // Attempt to copy validated file into config location
    fs::copy(&path, &config.vps_file)?;

    log_info!(
        "override",
        "vps_config_file",
        &format!("✅ VPS config overridden with file: {}", path.display())
    );

    Ok(())
}

fn override_file(path: PathBuf) -> io::Result<()> {
    let file_path = &path;
    if !config::schema::validate_file(&path) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("❌ Schema validation failed for {}", path.display()),
        ));
    }

    let content = fs::read_to_string(file_path)?;
    let _config = deserialize_config(file_path, &content)?;

    let dest = TranquilityConfig::config_path()?;
    fs::copy(file_path, &dest)?;
    log_info!(
        "override",
        "config_file",
        &format!("✅ Overrode config file with {:?}", file_path)
    );
    Ok(())
}
