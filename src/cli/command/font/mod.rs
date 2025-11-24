// Module: Command/Font
// Location: cli/src/command/font/mod.rs
pub mod install;
pub mod list;
pub mod refresh;
pub mod uninstall;
pub mod update;

use crate::print_warn;
use clap::{Args, Subcommand};

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

pub fn handle_fonts_command(cmd: FontCommand, dry_run: bool) {
    if dry_run {
        print_warn!("[dry run] Simulating font command.");

        println!("Would show font command options: {:?}", cmd);
    }
    match cmd.command {
        Some(FontSubcommand::Install { all, name }) => install::install(all, name),
        Some(FontSubcommand::Uninstall { all, name }) => uninstall::uninstall(all, name),
        Some(FontSubcommand::Update {}) => update::update(),
        Some(FontSubcommand::List { installed, all }) => list::list(installed, all),
        None => list::list(false, false),
    }
}
