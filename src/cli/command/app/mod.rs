use clap::{Args, Subcommand};
use crate::{models::{application::list_supported_applications, category::{list_categories, Category}}};

use crate::cli::{print_subcommand_help};

pub mod install;
pub mod uninstall;

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

pub fn handle_app_command(cmd: AppCommand, dry_run:bool) {
    match cmd.command {
        Some(AppSubcommand::Install { all, server }) => {
            install::install_apps_command(all, server, dry_run);
        }
        Some(AppSubcommand::Uninstall { all, server }) => {
            uninstall::uninstall_apps_command(all, server, dry_run);
        }
        Some(AppSubcommand::Categories {}) => list_categories(),
        Some(AppSubcommand::List { server, category }) => {
            list_supported_applications(server, category);
        }
        None => print_subcommand_help("apps"),
    }
}