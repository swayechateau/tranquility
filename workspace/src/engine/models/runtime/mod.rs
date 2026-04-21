use std::path::PathBuf;

use crate::engine::config::BaseConfig;

pub mod mode;
pub mod options;
pub mod resolution;
pub use options::*;
pub use resolution::*;

#[derive(Debug, Clone)]
pub struct Runtime {
    mode: mode::RunMode,
    options: RuntimeOptions,
    cwd: PathBuf,
    resolution: RuntimeResolution,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    pub fn new() -> Self {
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self {
            mode: mode::RunMode::Interactive,
            options: RuntimeOptions::new(),
            cwd,
            resolution: RuntimeResolution::default(),
        }
    }

    pub fn output_config(&self) -> scriba::Config {
        scriba::Config {
            interactive: matches!(self.mode, mode::RunMode::Interactive),
            format: self.options.output_format(),
            color: self.options.output_color(),
            level: self.options.log_level(),
            auto_yes: self.options.auto_yes(),
        }
    }

    pub fn mode(&self) -> &mode::RunMode {
        &self.mode
    }

    pub fn is_interactive(&self) -> bool {
        matches!(self.mode, mode::RunMode::Interactive)
    }

    pub fn options(&self) -> &RuntimeOptions {
        &self.options
    }

    pub fn options_mut(&mut self) -> &mut RuntimeOptions {
        &mut self.options
    }

    pub fn cwd(&self) -> &PathBuf {
        &self.cwd
    }

    pub fn config(&self) -> Option<&ResolvedConfig> {
        self.resolution.config.as_ref()
    }

    pub fn explicit_config_path(&self) -> Option<&PathBuf> {
        self.resolution.explicit_config_path.as_ref()
    }

    pub fn set_mode(&mut self, mode: mode::RunMode) -> &mut Self {
        self.mode = mode;
        self
    }

    pub fn set_dry_run(&mut self, dry_run: bool) -> &mut Self {
        self.options.set_dry_run(dry_run);
        self
    }

    pub fn set_cwd(&mut self, cwd: PathBuf) -> &mut Self {
        self.cwd = std::fs::canonicalize(&cwd).unwrap_or_else(|_| cwd);
        self
    }

    pub fn set_explicit_config_path(&mut self, path: Option<PathBuf>) -> &mut Self {
        self.resolution.explicit_config_path = path;
        self
    }

    pub fn load_config(&mut self) -> &mut Self {
        let config = BaseConfig::load_once().clone();
        let path = BaseConfig::config_path().unwrap_or_else(|_| {
            dirs::config_dir()
                .unwrap_or_default()
                .join("tranquility/config.yaml")
        });
        self.resolution.config = Some(ResolvedConfig { path, base: config });
        self
    }
}
