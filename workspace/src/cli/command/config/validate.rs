// Module: Command/Config/Validate
// Location:cli/src/command/config/validate.rs
use crate::{
    config,
    models::{application, vps},
    print_error, print_success,
};
use std::path::PathBuf;

pub fn validate_config(path: PathBuf) {
    if !config::schema::validate_file(&path) {
        print_error!(
            "❌ Validation failed for {}; try running --debug for more info",
            path.display()
        );
        return;
    }

    print_success!("✅ Config file is valid");
}

pub fn validate_applications(path: PathBuf) {
    if !application::schema::validate_file(&path) {
        print_error!(
            "❌ Validation failed for {}; try running --debug for more info",
            path.display()
        );
        return;
    }

    print_success!("✅ Application config file is valid");
}

pub fn validate_vps(path: PathBuf) {
    if !vps::schema::validate_file(&path) {
        print_error!(
            "❌ Validation failed for {}; try running --debug for more info",
            path.display()
        );
        return;
    }

    print_success!("✅ VPS config file is valid");
}
