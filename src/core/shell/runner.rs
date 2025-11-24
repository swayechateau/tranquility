use std::time::Instant;
use crate::{print_info, print_success,
    models::application::{Application, InstallMethod}};

pub struct InstallRunner<'a> {
    pub app: &'a Application,
    pub method: &'a InstallMethod,
    pub dry_run: bool,
}

impl<'a> InstallRunner<'a> {
    pub fn new(app: &'a Application, method: &'a InstallMethod, dry_run: bool) -> Self {
        Self { app, method, dry_run }
    }

    pub fn run_install(&self) -> std::time::Duration {
        print_info!("🚀 Installing {}...", self.app.name);
        let start = Instant::now();
        self.method.install(self.dry_run);
        let duration = start.elapsed();
        print_success!("✅ Installed {} in {:.2?}", self.app.name, duration);
        duration
    }

    pub fn run_uninstall(&self) -> std::time::Duration {
        print_info!("🧹 Uninstalling {}...", self.app.name);
        let start = Instant::now();
        self.method.uninstall(self.dry_run);
        let duration = start.elapsed();
        print_success!("🗑️ Uninstalled {} in {:.2?}", self.app.name, duration);
        duration
    }
}