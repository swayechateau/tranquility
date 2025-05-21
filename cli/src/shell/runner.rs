
use crate::model::application::Application;
// src/shell/runner.rs
use crate::shell::ShellCommand;
use crate::{print_error, print_info};
use colored::Colorize;

pub struct InstallRunner<'a> {
    pub app: &'a Application,
    pub install_block: &'a InstallBlock,
    pub dry_run: bool,
}

impl<'a> InstallRunner<'a> {
    pub fn new(app: &'a Application, block: &'a InstallBlock, dry_run: bool) -> Self {
        Self { app, install_block: block, dry_run }
    }

    pub fn run_install(&self) {
        print_info!("🚀 Installing {}...", self.app.name);

        self.run_steps("Pre-install", &self.install_block.preinstall_steps);

        // 1. Shell command or custom steps
        if let Some(cmd) = &self.install_block.command {
            ShellCommand::new(cmd).with_sudo(true).run_verbose(self.dry_run);
        } else if !self.install_block.steps.is_empty() {
            self.run_steps("Install", &self.install_block.steps);
        }

        // 2. Package manager install (auto-select first valid one)
        if !self.install_block.package_managers.is_empty() && !self.app.dependencies.is_empty() {
            let mut installed = false;
            for pm in &self.install_block.package_managers {
                if pm.check_install() {
                    if self.dry_run {
                        print_info!("💡 [Dry Run] Would install dependencies via {}", pm.name().green());
                    } else {
                        print_info!("📦 Using package manager: {}", pm.name().green());
                    }
                    for dep in &self.app.dependencies {
                        pm.install(dep, None, self.dry_run);
                    }
                    installed = true;
                    break;
                }
            }

            if !installed {
                print_error!("❌ No usable package manager found for: {}", self.app.name);
            }
        }

        self.run_steps("Post-install", &self.install_block.postinstall_steps);
    }

    pub fn run_uninstall(&self, uninstall: &Uninstall) {
        print_info!("🧹 Uninstalling {}...", self.app.name);

        if let Some(cmd) = &uninstall.command {
            ShellCommand::new(cmd).with_sudo(true).run_verbose(self.dry_run);
        }

        if !uninstall.steps.is_empty() {
            self.run_steps("Uninstall Steps", &uninstall.steps);
        }

        if !self.install_block.package_managers.is_empty() && !self.app.dependencies.is_empty() {
            for pm in &self.install_block.package_managers {
                if pm.check_installed() {
                    if self.dry_run {
                        print_info!("💡 [Dry Run] Would install dependencies via {}", pm.name().green());
                    } else {
                        print_info!("📦 Using package manager: {}", pm.name().green());
                    }
                    for dep in &self.app.dependencies {
                        pm.uninstall(dep, self.dry_run);
                    }
                    break;
                }
            }
        }
    }

    fn run_steps(&self, label: &str, steps: &[String]) {
        if !steps.is_empty() {
            print_info!("🔧 {} steps:", label);
            for step in steps {
                ShellCommand::new("sh")
                    .with_args(&["-c", step])
                    .with_sudo(true)
                    .run_verbose(self.dry_run);
            }
        }
    }
}
