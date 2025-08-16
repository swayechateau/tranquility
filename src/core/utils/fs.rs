use crate::core::constants::APP_NAME;

// config directory utilities
pub fn get_config_dir() -> std::path::PathBuf {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let config_dir = home_dir.join(".config");
    config_dir.join(APP_NAME)
}