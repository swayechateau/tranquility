use std::path::PathBuf;

use crate::engine::config::BaseConfig;

/// The resolved (loaded) tranquility config for the current session.
#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    pub path: PathBuf,
    pub base: BaseConfig,
    /// Set when the config file couldn't be loaded/validated and `base` is
    /// an in-memory fallback rather than what's actually on disk at `path`.
    pub load_error: Option<String>,
}

/// Runtime resolution state: holds the loaded config and any explicit override path.
#[derive(Debug, Clone, Default)]
pub struct RuntimeResolution {
    pub config: Option<ResolvedConfig>,
    pub explicit_config_path: Option<PathBuf>,
}
