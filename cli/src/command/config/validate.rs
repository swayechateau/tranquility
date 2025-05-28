// Module: Command/Config/Validate
// Location:cli/src/command/config/validate.rs
use std::path::PathBuf;
use tranquility::{log_error, log_info};
use crate::schema;

pub fn validate_config(path: PathBuf) {
    if !schema::config::validate_file(&path) {
        log_error!("validate", "config_file",&format!("❌ Validation failed for {}", path.display()));
    }

    log_info!(
        "validate",
        "config_file",
        "✅ VPS file is valid"
    );
}

pub fn validate_applications(path: PathBuf) {
    if !schema::application::validate_file(&path) {
        log_error!("validate", "application_config_file",&format!("❌ Validation failed for {}", path.display()));
    }

    log_info!(
        "validate",
        "application_config_file",
        "✅ VPS file is valid"
    );
}

pub fn validate_vps(path: PathBuf) {
    if !schema::vps::validate_file(&path) {
        log_error!("validate", "vps_config_file",&format!("❌ Validation failed for {}", path.display()));
    }

    log_info!(
        "validate",
        "vps_config_file",
        "✅ VPS file is valid"
    );
}
