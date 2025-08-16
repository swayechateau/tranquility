// Module: Command/App
// Location: cli/src/command/app/mod.rs
use clap::{Args, Subcommand};
use crate::{core::{tquil, model::{application::list_supported_applications, category::{list_categories, Category}}}};

#[derive(Args, Debug)]
pub struct AppCommand {
    #[command(subcommand)]
    command: Option<AppSubcommand>,
}

#[derive(Subcommand, Debug)]
pub enum AppSubcommand {
    /// Install default applications and from applications.json
    Install {
        #[arg(long)]
        all: bool,
        #[arg(long)]
        server: bool
    },

    /// Uninstall default applications and from applications.json
    Uninstall {
        #[arg(long)]
        all: bool,
        #[arg(long)]
        server: bool
    },

    /// List all categories
    Categories {},

    /// List supported applications
    List {
        #[arg(long)]
        server: bool,
        #[arg(long, value_enum)]
        category: Vec<Category>,
    },
}

pub fn run(cmd: AppCommand, dry_run:bool) -> anyhow::Result<()> {
    match cmd.command {
        Some(AppSubcommand::Install { all, server }) => {
            tquil::apps::install(all, server, dry_run);
            Ok(())
        }
        Some(AppSubcommand::Uninstall { all, server }) => {
            tquil::apps::uninstall(all, server, dry_run);
            Ok(())
        }
        Some(AppSubcommand::Categories {}) => {
            list_categories();
            Ok(())
        }
        Some(AppSubcommand::List { server, category }) => {
            list_supported_applications(server, category);
            Ok(())
        }
        None => {
            println!("Please specify a subcommand for 'app':");
            println!("  install   Install applications");
            println!("  uninstall Uninstall applications");
            println!("  categories List all application categories");
            println!("  list      List supported applications");
            Ok(())
        },
    }
}