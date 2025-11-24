// Module: Command/Tranquility
// Location: cli/src/command/tranquility.rs
use crate::{log_error, print_info};
use clap::{CommandFactory, Error, Parser, Subcommand, error::ErrorKind};

use crate::{core::logger, models::system::SystemInfo};

use super::command::{app, config, doctor, font, list, logs, vps};

/// Tranquility CLI command line parser
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct TranquilityCommand {
    /// Turn debugging information on (repeatable)
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub debug: bool,

    /// Show verbose output
    #[arg(long, action = clap::ArgAction::SetTrue)]
    #[arg(long)]
    dry_run: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Tranquility CLI sub commands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Check and fix common issues
    Doctor(doctor::DoctorCommand),
    /// Configuration management
    Config(config::ConfigCommand),
    /// Show tranquility logs
    Logs(logs::LogsCommand),
    /// Application management
    App(app::AppCommand),
    /// Font management
    Font(font::FontCommand),
    /// VPS host management
    Vps(vps::VpsCommand),
    /// List supported applications, fonts, or VPS hosts
    List(list::ListCommand),
}

/// handle_args function
pub fn handle_commands(commands: TranquilityCommand) {
    if commands.command.is_none() {
        println!("{}\n", SystemInfo::new().to_pretty_string());
        return;
    }

    if commands.debug {
        logger::set_debug(true);
    }

    if commands.dry_run {
        print_info!("💡 Running in dry-run mode. No changes will be made.");
    }

    match commands.command {
        Some(Commands::Config(config)) => config::handle_config_command(config, commands.dry_run),

        Some(Commands::Logs(logs)) => logs::handle_logs_command(logs, commands.dry_run),

        Some(Commands::Font(font)) => font::handle_fonts_command(font, commands.dry_run),

        Some(Commands::Doctor(doctor)) => doctor::doctor_command(doctor, commands.dry_run),

        Some(Commands::App(app)) => app::handle_app_command(app, commands.dry_run),

        Some(Commands::Vps(vps)) => vps::handle_vps_command(vps, commands.dry_run),

        Some(Commands::List(list)) => list::handle_list_command(list, commands.dry_run),

        None => {}
    }
}

pub fn handle_command_errors(err: Error) {
    match err.kind() {
        ErrorKind::UnknownArgument | ErrorKind::InvalidSubcommand => {
            TranquilityCommand::command().print_help().unwrap();
            println!();
        }
        _ => {
            err.print().expect("Failed to print error");
        }
    }
    std::process::exit(1);
}

pub fn print_subcommand_help(subcommand: &str) {
    let mut cmd = TranquilityCommand::command();
    if let Some(sub) = cmd.find_subcommand_mut(subcommand) {
        sub.print_help().unwrap();
        println!();
    } else {
        log_error!(
            "print",
            "help",
            &format!("❌ Subcommand `{subcommand}` not found.")
        );
    }
}
