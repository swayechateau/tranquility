// Module: Command/Config/Reset
// Location:cli/src/command/config/reset.rs
use tranquility::{log_info, print_success};
use crate::model::config::TranquilityConfig;

pub fn reset_config() {
    TranquilityConfig::reset().expect("Failed to reset config");
    log_info!("reset", "config", "✅ reinitialized");
    print_success!("Config reinitialized");
}
pub fn reset_applications() {

}
pub fn reset_vps() {

}
