pub mod reset;
pub mod replace;
pub mod validate;

use crate::{core::config::TranquilityConfig, log_error};

fn get_config() -> TranquilityConfig {
    (*TranquilityConfig::load_once()).clone()
}

pub fn fix_config() {
    if let Err(e) = TranquilityConfig::fix() {
        log_error!("fix","config",&format!("failed: {e}"));
    }
}