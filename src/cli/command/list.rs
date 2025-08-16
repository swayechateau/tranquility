// Module: Command/List
// Location: cli/src/command/list/mod.rs
use clap::{Args, Subcommand};

use crate::{
    cli::command::vps::{VpsListCommand, vps_command_list_run},
    core::{
        model::{
            application::list_supported_applications,
            category::{Category, list_categories},
        },
        tquil::font,
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

pub fn run(cmd: ListCommand, dry_run: bool) -> anyhow::Result<()> {
    match cmd.command {
        Some(ListSubCommand::Apps { server, category }) => {
            list_supported_applications(server, category);
            Ok(())
        }
        Some(ListSubCommand::Fonts { installed, all }) => {
            font::list::list(installed, all);
            Ok(())
        }
        Some(ListSubCommand::Vps(vps)) => {
            vps_command_list_run(vps, dry_run)
        }
        Some(ListSubCommand::Categories {}) => {
            list_categories();
            Ok(())
        }
        None => {
            println!("Please specify a subcommand for 'list':");
            println!("  apps       List supported applications");
            println!("  fonts      List supported fonts");
            println!("  vps        List supported VPS hosts");
            println!("  categories List all application categories");
            Ok(())
        }
    }
}
