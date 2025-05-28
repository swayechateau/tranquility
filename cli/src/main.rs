// Module: CLI
// Location: cli/src/main.rs
mod core;
mod command;
mod model;
mod shell;
mod utils;
mod schema;

use clap::Parser;
use model::{category, config, package_manager, system};
use command::{tranquility::{handle_command_errors, handle_commands, TranquilityCommand}};
use tranquility::log_info;
use core::{print::tranquility_figlet};

fn main() {
    tranquility_figlet();
    log_info!("Starting", "tranquility", "success");
    match TranquilityCommand::try_parse() {
        Ok(args) => {
            handle_commands(args);
        }
        Err(err) => {
            handle_command_errors(err);
        }
    }
}
