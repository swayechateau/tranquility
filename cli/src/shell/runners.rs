use crate::models::{Application, InstallBlock, InstallMethod, Uninstall};
use crate::shell::ShellCommand;
use crate::{print_error, print_info};

pub struct InstallRunner<'a> {
    pub app: &'a Application,
    pub install_block: &'a InstallBlock,
}

impl<'a> InstallRunner<'a> {
    pub fn new(app: &'a Application, block: &'a InstallBlock) -> Self {
        InstallRunner { app, install_block: block }
    }

    pub fn run_install(&self) {
        print_info!("🚀 Installing {}...", self.app.name);

        self.run_steps("Pre-install", &self.install_block.preinstall_steps);

        if let Some(cmd) = &self.install_block.command {
            ShellCommand::new(cmd).with_sudo(true).run_verbose();
        } else if !self.install_block.steps.is_empty() {
            self.run_steps("Install", &self.install_block.steps);
        } else {
            print_error!("❌ No install steps defined for {}", self.app.name);
        }

        self.run_steps("Post-install", &self.install_block.postinstall_steps);
    }

    pub fn run_uninstall(&self, uninstall: &Uninstall) {
        print_info!("🧹 Uninstalling {}...", self.app.name);

        if let Some(cmd) = &uninstall.command {
            ShellCommand::new(cmd).with_sudo(true).run_verbose();
        }

        if !uninstall.steps.is_empty() {
            self.run_steps("Uninstall Steps", &uninstall.steps);
        }
    }

    fn run_steps(&self, label: &str, steps: &[String]) {
        if !steps.is_empty() {
            print_info!("🔧 {} steps:", label);
            for step in steps {
                ShellCommand::new("sh")
                    .with_args(&["-c", step])
                    .with_sudo(true)
                    .run_verbose();
            }
        }
    }
}
