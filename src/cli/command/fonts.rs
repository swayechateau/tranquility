// Module: Command/Font
// Location: cli/src/command/font/mod.rs
use clap::{Args, Subcommand};

use crate::{print_warn, core::tquil::font};

#[derive(Args, Debug)]
pub struct FontCommand {
    #[command(subcommand)]
    command: Option<FontSubcommand>,
}

#[derive(Subcommand, Debug)]
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
    /// 🔁 Update all installed fonts
    Update {},
    /// List installed fonts
    List {
        /// Show only installed fonts
        #[arg(long)]
        installed: bool,
        /// Show available and installed fonts
        #[arg(long)]
        all: bool,
    },
}

pub fn run(cmd: FontCommand, dry_run: bool) -> anyhow::Result<()> {
    if dry_run {
        print_warn!("[dry run] Simulating font command.");

        println!("Would show font command options: {:?}", cmd);
    }
    match cmd.command {
        Some(FontSubcommand::Install { all, name }) => {
            font::install::install(all, name);
            Ok(())
        }
        Some(FontSubcommand::Uninstall { all, name }) => {
            font::uninstall::uninstall(all, name);
            Ok(())
        }
        Some(FontSubcommand::Update {}) => {
            font::update::update();
            Ok(())
        }
        Some(FontSubcommand::List { installed, all }) => {
            font::list::list(installed, all);
            Ok(())
        }
        None => {
            font::list::list(false, false);
            Ok(())
        }
    }
}
