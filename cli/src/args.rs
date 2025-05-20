// src/args.rs
use clap::{error::ErrorKind, CommandFactory, Error, Parser, Subcommand};
use std::path::PathBuf;
use crate::applications::list_supported_application_for_current_os;

use crate::categories::Category;
use crate::print::print_info;

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
    // If no subcommand, show help (optional)
    if args.command.is_none() {
        TranquilityArgs::command()
            .print_help()
            .expect("Failed to print help");
        println!(); // newline after help
        return;
    }

    // Run logic based on subcommand
    match args.command {
        Some(Commands::Install {all, server}) => {
            print_info("Installing...".to_string());
        }
        Some(Commands::Uninstall {all, server}) => {
            print_info("Uninstalling...".to_string());
        }
        Some(Commands::List { server, category}) => {
            print_info("Listing...".to_string());
            list_supported_application_for_current_os(server, category);
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
