// Module: Command/List
// Location: cli/src/command/list/mod.rs

use clap::{Args, Subcommand};

use crate::{
    cli::{
        command::{
            font,
            vps::list::{VpsListCommand, vps_command_list},
        },
        print_subcommand_help,
    },
    models::{
        application::list_supported_applications,
        category::{Category, list_categories},
    },
};
#[derive(Args, Debug)]
pub struct ListCommand {
    #[command(subcommand)]
    command: Option<ListSubCommand>,
}

#[derive(Subcommand, Debug)]
pub enum ListSubCommand {
    Apps {
        #[arg(long)]
        server: bool,
        #[arg(long)]
        category: Vec<Category>,
    },
    Fonts {
        /// Show only installed fonts
        #[arg(long)]
        installed: bool,
        /// Show available and installed fonts
        #[arg(long)]
        all: bool,
    },
    Vps(VpsListCommand),
    Categories {},
}

pub fn handle_list_command(cmd: ListCommand, dry_run: bool) {
    match cmd.command {
        Some(ListSubCommand::Apps { server, category }) => {
            list_supported_applications(server, category);
        }
        Some(ListSubCommand::Fonts { installed, all }) => {
            font::list::list(installed, all);
        }
        Some(ListSubCommand::Vps(vps)) => vps_command_list(vps, dry_run),
        Some(ListSubCommand::Categories {}) => list_categories(),
        None => print_subcommand_help("list"),
    }
}
