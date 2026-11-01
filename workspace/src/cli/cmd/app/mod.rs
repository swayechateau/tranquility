use crate::engine::models::category::Category;
use crate::{
    cli::{CliResult, args::print_subcommand_help},
    core::context::Context,
    core::usecases::apps,
};
use clap::{Args as ClapArgs, Subcommand};

#[derive(Debug, Clone, ClapArgs)]
#[command(about = "Manage applications")]
pub struct Args {
    /// Apply to all applications (install/uninstall)
    #[arg(long, global = true)]
    all: bool,
    /// Include server-compatible applications
    #[arg(long, global = true)]
    server: bool,
    #[command(subcommand)]
    pub subcommand: Option<AppSubcommand>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum AppSubcommand {
    /// Install default applications and any from the configured applications file
    Install,

    /// Uninstall default applications and any from the configured applications file
    Uninstall,

    /// List all categories
    Categories,

    /// List supported applications
    List {
        /// Filter by category
        #[arg(long, value_enum)]
        category: Vec<Category>,
    },
}

pub async fn run(ctx: &Context, args: Args) -> CliResult<()> {
    match args.subcommand {
        Some(AppSubcommand::Install) => apps::install(ctx, args.all, args.server).await,
        Some(AppSubcommand::Uninstall) => apps::uninstall(ctx, args.all, args.server).await,
        Some(AppSubcommand::Categories) => apps::categories(ctx).await,
        Some(AppSubcommand::List { category }) => apps::list(ctx, args.server, category).await,
        None => {
            print_subcommand_help("app");
            Ok(())
        }
    }
}
