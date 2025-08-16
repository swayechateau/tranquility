// Module: Command/Config/Validate
// Location:cli/src/command/config/validate.rs
use std::path::PathBuf;
use crate::{core::schema, print_error, print_success};

pub fn validate_config(path: PathBuf) {
    if !schema::config::validate_file(&path) {
        print_error!("❌ Validation failed for {}; try running --debug for more info", path.display());
        return;
    }

    print_success!("✅ Config file is valid");
}

pub fn validate_applications(path: PathBuf) {
    if !schema::application::validate_file(&path) {
        print_error!("❌ Validation failed for {}; try running --debug for more info", path.display());
        return;
    }

    print_success!("✅ Application config file is valid");
}

pub fn validate_vps(path: PathBuf) {
    if !schema::vps::validate_file(&path) {
        print_error!("❌ Validation failed for {}; try running --debug for more info", path.display());
        return;
    }

    print_success!("✅ VPS config file is valid");
}
