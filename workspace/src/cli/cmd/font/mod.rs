use crate::{
    cli::{CliResult, args::print_subcommand_help},
    core::{context::Context, usecases::font},
};
use clap::{Args as ClapArgs, Subcommand};

#[derive(Debug, Clone, ClapArgs)]
#[command(about = "Manage Nerd Fonts")]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Option<FontSubcommand>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum FontSubcommand {
    /// Install Nerd Fonts
    Install {
        /// Install all fonts
        #[arg(long)]
        all: bool,
        /// Font name(s) (comma-separated or repeated)
        #[arg(long, value_name = "NAME")]
        name: Vec<String>,
    },
    /// Uninstall Nerd Fonts
    Uninstall {
        /// Uninstall all fonts
        #[arg(long)]
        all: bool,
        /// Font name(s) (comma-separated or repeated)
        #[arg(long, value_name = "NAME")]
        name: Vec<String>,
    },
    /// Update all installed fonts
    Update,
    /// List fonts
    List {
        /// Show only installed fonts
        #[arg(long)]
        installed: bool,
        /// Show available and installed fonts
        #[arg(long)]
        all: bool,
    },
}

pub async fn run(ctx: &Context, args: Args) -> CliResult<()> {
    match args.subcommand {
        Some(FontSubcommand::Install { all, name }) => {
            font::install(ctx, all, name).await.map_err(|e| e.into())
        }
        Some(FontSubcommand::Uninstall { all, name }) => {
            font::uninstall(ctx, all, name).await.map_err(|e| e.into())
        }
        Some(FontSubcommand::Update) => font::update(ctx).await.map_err(|e| e.into()),
        Some(FontSubcommand::List { installed, all }) => {
            font::list(ctx, installed, all).await.map_err(|e| e.into())
        }
        None => {
            print_subcommand_help("font");
            Ok(())
        }
    }
}
