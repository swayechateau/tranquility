use std::path::PathBuf;

use crate::{
    engine::{
        Error,
        config::BaseConfig,
        models::runtime::{ResolvedConfig, Runtime},
    },
    infra::ui::Ui,
};

pub type AppResult<T> = Result<T, Error>;

#[derive(Debug)]
pub struct Context {
    runtime: Runtime,
}

impl Context {
    pub fn new(runtime: Runtime) -> Self {
        Self { runtime }
    }

    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    pub fn set_workdir(&mut self, path: PathBuf) {
        self.runtime.set_cwd(path);
    }

    pub fn is_interactive(&self) -> bool {
        self.runtime.is_interactive()
    }

    pub fn cwd(&self) -> &PathBuf {
        self.runtime.cwd()
    }

    pub fn dry_run(&self) -> bool {
        self.runtime.options().dry_run()
    }

    pub fn auto_yes(&self) -> bool {
        self.runtime.options().auto_yes()
    }

    pub fn force(&self) -> bool {
        self.runtime.options().force()
    }

    pub fn config(&self) -> Option<&ResolvedConfig> {
        self.runtime.config()
    }

    pub fn base_config(&self) -> &BaseConfig {
        self.runtime
            .config()
            .map(|c| &c.base)
            .unwrap_or_else(|| BaseConfig::load_once())
    }

    pub fn explicit_config_path(&self) -> Option<&PathBuf> {
        self.runtime.explicit_config_path()
    }

    pub fn project_config_path(&self) -> Option<PathBuf> {
        self.runtime.config().map(|c| c.path.clone())
    }

    pub fn ui_config(&self) -> scriba::Config {
        self.runtime.output_config()
    }

    pub fn ui(&self) -> Ui {
        Ui::cached_with_config(self.ui_config(), self.runtime.options().output_envelope())
    }
}
