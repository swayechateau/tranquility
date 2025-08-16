// Module: Command/Tranquility
// Location: src/command/cli.rs
use clap::{Parser, Subcommand};

use crate::{
    cli::command::{app, config, doctor, fonts, hosts, list, logs, vps},
    core::{
        config::TranquilityConfig,
        model::system::SystemInfo, 
        utils::{logger, print::tranquility_figlet}
    }, print_info
};

/// Tranquility CLI command line parser
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on (repeatable)
    #[arg(short='d', long, action = clap::ArgAction::SetTrue)]
    pub debug: bool,
    /// Show dry run output
    #[arg(short='r', long, action = clap::ArgAction::SetTrue)]
    pub dry_run: bool,
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
    Font(fonts::FontCommand),
    /// VPS host management
    Vps(vps::VpsCommand),
    /// List supported applications, fonts, or VPS hosts
    List(list::ListCommand),
    /// Setup dev domains in system hosts file
    Hosts(hosts::HostCommand),
}

pub fn run() -> anyhow::Result<()> {
    TranquilityConfig::load_once();
    tranquility_figlet();

    let args: Cli = Cli::parse();

    if args.debug {
        logger::set_debug(true);
    }

    if args.dry_run {
        print_info!("💡 Running in dry-run mode. No changes will be made.");
    }
    match args.command {
        Some(Commands::Config(config)) => config::run(config, args.dry_run),
        Some(Commands::Logs(logs)) => logs::run(logs, args.dry_run),
        Some(Commands::Font(font)) => fonts::run(font, args.dry_run),
        Some(Commands::Doctor(doctor)) => doctor::run(doctor, args.dry_run),
        Some(Commands::App(app)) => app::run(app, args.dry_run),
        Some(Commands::Vps(vps)) => vps::run(vps, args.dry_run),
        Some(Commands::List(list)) => list::run(list, args.dry_run),
        Some(Commands::Hosts(hosts)) => hosts::run(hosts, args.dry_run),
        None => {
            println!("{}\n", SystemInfo::new().to_pretty_string());
            Ok(())
        }

    }
}
