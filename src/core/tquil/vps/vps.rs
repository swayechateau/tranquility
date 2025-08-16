use crate::core::{config::TranquilityConfig, model::vps::json::VpsConfig};

/// Fixes and updates the VPS config file in place, if needed
pub fn fix_vps() -> std::io::Result<()> {
    let config = TranquilityConfig::load_once();
    let path = &config.vps_file;

    let mut vps_config = VpsConfig::load_from_file(path)?;
    match vps_config.fix_and_save(path) {
        Ok(()) => crate::log_info!("reset", "vps-config-file", "vps config file updated"),
        Err(e) => {
            crate::log_error!("reset", "vps-config-file", &format!("❌ Failed to update VPS config file: {e}"));
            return Err(e);
        },
    }
    
    Ok(())
}
