use crate::{
    cli::{CliResult, args::print_subcommand_help},
    core::{context::Context, usecases::list},
};
use clap::{Args as ClapArgs, Subcommand};

#[derive(Debug, Clone, ClapArgs)]
#[command(about = "List applications, fonts, VPS profiles, and categories")]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Option<ListSubcommand>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ListSubcommand {
    /// List supported applications
    Apps {
        /// Show only server-compatible apps
        #[arg(long)]
        server: bool,
        /// Filter by category (can be repeated)
        #[arg(long, value_name = "CATEGORY")]
        category: Vec<crate::engine::models::category::Category>,
    },
    /// List Nerd Fonts
    Fonts {
        /// Show only installed fonts
        #[arg(long)]
        installed: bool,
        /// Show available and installed fonts
        #[arg(long)]
        all: bool,
    },
    /// List VPS profiles
    Vps {
        /// Filter by user
        #[arg(long)]
        user: Option<String>,
        /// Filter by host
        #[arg(long)]
        host: Option<String>,
    },
    /// List all categories
    Categories,
}

pub async fn run(ctx: &Context, args: Args) -> CliResult<()> {
    match args.subcommand {
        Some(ListSubcommand::Apps { server, category }) => list::apps(ctx, server, category)
            .await
            .map_err(|e| e.into()),
        Some(ListSubcommand::Fonts { installed, all }) => {
            list::fonts(ctx, installed, all).await.map_err(|e| e.into())
        }
        Some(ListSubcommand::Vps { user, host }) => {
            list::vps(ctx, user.as_deref(), host.as_deref())
                .await
                .map_err(|e| e.into())
        }
        Some(ListSubcommand::Categories) => list::categories(ctx).await.map_err(|e| e.into()),
        None => {
            print_subcommand_help("list");
            Ok(())
        }
    }
}
