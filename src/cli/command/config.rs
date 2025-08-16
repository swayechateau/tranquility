// Module: Command/Config
// Location: cli/src/command/config.rs
use std::path::PathBuf;

use anyhow::Ok;
use clap::{Args, Subcommand};
use crate::{core::tquil::configure, log_info, print_info};

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

pub fn run(cmd: ConfigCommand, dry_run: bool) -> anyhow::Result<()> {
    if dry_run {
        print_info!("🔍 Dry run mode enabled. No changes will be made.");
    }
    match cmd.command {
        Some(ConfigSubcommand::Reset { target }) => {
            match target {
                ConfigTarget::Config => {
                    configure::reset::reset_config();
                    Ok(())
                }
                ConfigTarget::Applications => {
                    configure::reset::reset_applications();
                    Ok(())
                }
                ConfigTarget::Vps => {
                    configure::reset::reset_vps();
                    Ok(())
                }
            }
        }

        Some(ConfigSubcommand::Override { target, file }) => {
            match target {
                ConfigTarget::Config => {
                    configure::replace::replace_config(file);
                    Ok(())
                }
                ConfigTarget::Applications => {
                    configure::replace::replace_applications(file);
                    Ok(())
                }
                ConfigTarget::Vps => {
                    configure::replace::replace_vps(file);
                    Ok(())
                }
            }
        }

        Some(ConfigSubcommand::Validate { target, file }) => {
            match target {
                ConfigTarget::Config => {
                    configure::validate::validate_config(file);
                    Ok(())
                }
                ConfigTarget::Applications => {
                    configure::validate::validate_applications(file);
                    Ok(())
                }
                ConfigTarget::Vps => {
                    configure::validate::validate_vps(file);
                    Ok(())
                }
            }
        }

        None => {
            print_info!("❗ No config subcommand provided. Use `--help` to see available options.");
            log_info!("config-cmd", "none", "not provided");
            Ok(())
        }
    }
}
