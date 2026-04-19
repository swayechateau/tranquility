pub mod replace;
pub mod reset;
pub mod validate;
use std::path::PathBuf;

use crate::{config::TranquilityConfig, log_error, log_info, print_info};
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct ConfigCommand {
    #[command(subcommand)]
    pub command: Option<ConfigSubcommand>,
}

#[derive(Debug, Subcommand)]
pub enum ConfigSubcommand {
    /// Reset one of the config files to its default
    Reset {
        /// Target to reset: config (default), applications, or vps
        #[arg(value_enum, default_value = "config")]
        target: ConfigTarget,
    },

    /// Override a config file with a custom file
    Override {
        /// Target to override: config, applications, or vps
        #[arg(value_enum, default_value = "config")]
        target: ConfigTarget,

        /// Path to the override file
        #[arg(long)]
        file: PathBuf,
    },

    /// Validate a config file
    Validate {
        /// Target to validate: config, applications, or vps
        #[arg(value_enum, default_value = "config")]
        target: ConfigTarget,

        /// Path to the file to validate
        #[arg(long)]
        file: PathBuf,
    },
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ConfigTarget {
    Config,
    Applications,
    Vps,
}

pub fn handle_config_command(cmd: ConfigCommand, dry_run: bool) {
    if dry_run {
        print_info!("🔍 Dry run mode enabled. No changes will be made.");
    }
    match cmd.command {
        Some(ConfigSubcommand::Reset { target }) => match target {
            ConfigTarget::Config => reset::reset_config(),
            ConfigTarget::Applications => reset::reset_applications(),
            ConfigTarget::Vps => reset::reset_vps(),
        },

        Some(ConfigSubcommand::Override { target, file }) => match target {
            ConfigTarget::Config => replace::replace_config(file),
            ConfigTarget::Applications => replace::replace_applications(file),
            ConfigTarget::Vps => replace::replace_vps(file),
        },

        Some(ConfigSubcommand::Validate { target, file }) => match target {
            ConfigTarget::Config => validate::validate_config(file),
            ConfigTarget::Applications => validate::validate_applications(file),
            ConfigTarget::Vps => validate::validate_vps(file),
        },

        None => {
            print_info!("❗ No config subcommand provided. Use `--help` to see available options.");
            log_info!("config-cmd", "none", "not provided")
        }
    }
}

fn get_config() -> TranquilityConfig {
    (*TranquilityConfig::load_once()).clone()
}

pub fn fix_config() {
    if let Err(e) = TranquilityConfig::fix() {
        log_error!("fix", "config", &format!("failed: {e}"));
    }
}
