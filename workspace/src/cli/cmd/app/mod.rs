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
    /// Install all applications
    #[arg(long)]
    all: bool,
    /// Include server-compatible applications
    #[arg(long)]
    server: bool,
    #[command(subcommand)]
    pub subcommand: Option<AppSubcommand>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum AppSubcommand {
    /// Install default applications and from applications.json
    Install,

    /// Uninstall default applications and from applications.json
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
        Some(AppSubcommand::Install) => apps::install(ctx, args.all, args.server)
            .await
            .map_err(|e| e.into()),
        Some(AppSubcommand::Uninstall) => apps::uninstall(ctx, args.all, args.server)
            .await
            .map_err(|e| e.into()),
        Some(AppSubcommand::Categories) => apps::categories(ctx).await.map_err(|e| e.into()),
        Some(AppSubcommand::List { category }) => apps::list(ctx, args.server, category)
            .await
            .map_err(|e| e.into()),
        None => {
            print_subcommand_help("app");
            Ok(())
        }
    }
}
