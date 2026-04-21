//! Configuration management command.
//!
//! Handles config file operations: Reset, Override, and Validate.

use crate::{cli::CliResult, core::context::Context};
use clap::{Args as ClapArgs, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, ClapArgs)]
#[command(about = "Inspect and manage Tranquility configuration")]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: ConfigSubcommand,
}

#[derive(Debug, Clone, Subcommand)]
pub enum ConfigSubcommand {
    /// Reset one of the config files to its default
    Reset {
        /// Target to reset: config, applications, or vps
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

#[derive(Debug, Clone, ValueEnum)]
pub enum ConfigTarget {
    Config,
    Applications,
    Vps,
}

pub async fn run(ctx: &Context, args: Args) -> CliResult<()> {
    match args.subcommand {
        ConfigSubcommand::Reset { target } => {
            use crate::core::usecases::config;
            match target {
                ConfigTarget::Config => config::reset_config(ctx).await?,
                ConfigTarget::Applications => config::reset_applications(ctx).await?,
                ConfigTarget::Vps => config::reset_vps(ctx).await?,
            }
        }
        ConfigSubcommand::Override { target, file } => {
            use crate::core::usecases::config;
            match target {
                ConfigTarget::Config => config::override_config(ctx, &file).await?,
                ConfigTarget::Applications => config::override_applications(ctx, &file).await?,
                ConfigTarget::Vps => config::override_vps(ctx, &file).await?,
            }
        }
        ConfigSubcommand::Validate { target, file } => {
            use crate::core::usecases::config;
            match target {
                ConfigTarget::Config => config::validate_config(ctx, &file).await?,
                ConfigTarget::Applications => config::validate_applications(ctx, &file).await?,
                ConfigTarget::Vps => config::validate_vps(ctx, &file).await?,
            }
        }
    }
    Ok(())
}
