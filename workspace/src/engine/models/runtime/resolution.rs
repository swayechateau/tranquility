use std::path::PathBuf;

use crate::engine::config::BaseConfig;

/// The resolved (loaded) tranquility config for the current session.
#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    pub path: PathBuf,
    pub base: BaseConfig,
}

/// Runtime resolution state: holds the loaded config and any explicit override path.
#[derive(Debug, Clone, Default)]
pub struct RuntimeResolution {
    pub config: Option<ResolvedConfig>,
    pub explicit_config_path: Option<PathBuf>,
}
