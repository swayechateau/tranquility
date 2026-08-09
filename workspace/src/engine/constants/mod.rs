pub mod defaults;
pub mod emoji;
pub mod env;
pub mod paths;

pub use emoji::*;
pub use env::*;
pub use paths::*;

pub const APP_NAME: &str = "tranquility";
pub const BIN_NAME: &str = "tquil";

/// List of supported file extensions for schema validation.
pub const SUPPORTED_EXTS: [&str; 4] = ["yaml", "yml", "json", "xml"];
pub const CONFIG_FILE_NAME: &str = "config.yaml";
pub const STATE_FILE_NAME: &str = "state.json";

pub const CONFIG_DIR_NAME: &str = "tranquility";
pub const CACHE_DIR_NAME: &str = "tranquility";
pub const STATE_DIR_NAME: &str = "tranquility";
