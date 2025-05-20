// src/args.rs
use clap::{error::ErrorKind, CommandFactory, Error, Parser, Subcommand};
use std::path::PathBuf;
use crate::applications::list_supported_applications;

use crate::categories::{list_categories, Category};
use crate::config::TranquilityConfig;
use crate::installer::{install_apps, uninstall_apps};
use crate::{print_info, print_success};
use crate::system::SystemInfo;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct TranquilityArgs {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize or reset config
    Init {
        #[arg(long)]
        reset: bool,
    },
    /// Install something
    Install {
        /// Install Everything
        #[arg(long)]
        all: bool,
        /// Install only server applications
        #[arg(long)]
        server: bool,
    },

    /// Uninstall something
    Uninstall {
        /// Uninstall Everything
        #[arg(long)]
        all: bool,
        /// Uninstall only server applications
        #[arg(long)]
        server: bool,
    },

    /// List all categories
    Categories {},

    /// List installed items
    List {
        /// Show only server applications
        #[arg(long)]
        server: bool,

        /// Filter by category
        #[arg(long, value_enum)]
        category: Vec<Category>,
    },
}

pub fn handle_args(args: TranquilityArgs) {
    let config_path = TranquilityConfig::config_dir()
      .unwrap().join("config.json");
    let sys = SystemInfo::new();
    // If no subcommand, show help (optional)
    if args.command.is_none() {
        println!("{}", sys.to_pretty_string());
        println!(); // newline after help
        return;
    }

    // Run logic based on subcommand
    match args.command {
        Some(Commands::Init { reset }) => {
            if reset {
                TranquilityConfig::reset().expect("Failed to reset config");
                print_success!("✅ Config reset to default at {}", config_path.display());
            } else {
                TranquilityConfig::load_or_init().expect("Failed to initialize config");
                print_success!("✅ Config initialized at {}", config_path.display());
            }
        }
        Some(Commands::Install {all, server}) => {
            print_info!("Installing...");
            install_apps(all, server);
        }
        Some(Commands::Uninstall {all, server}) => {
            print_info!("Uninstalling...");
            uninstall_apps(all, server);
        }
        Some(Commands::Categories {  }) => {
            list_categories();
        }
        Some(Commands::List { server, category}) => {
            list_supported_applications(server, category);
        }
        None => {}
    }
}

pub fn handle_arg_errors(err: Error) {
    match err.kind() {
        ErrorKind::UnknownArgument | ErrorKind::InvalidSubcommand => {
            // Show help instead of the default error
            TranquilityArgs::command().print_help().unwrap();
            println!(); // newline after help
        }
        _ => {
            // Print actual error for other issues (like missing required arguments)
            err.print().expect("Failed to print error");
        }
    }
    std::process::exit(1);
}
