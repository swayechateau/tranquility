// Module: CLI
// Location: cli/src/main.rs
mod core;
mod command;
mod model;
mod shell;
mod utils;
mod schema;

use clap::Parser;
use command::{tranquility::{handle_command_errors, handle_commands, TranquilityCommand}};
use core::{print::tranquility_figlet};

fn main() {
    model::config::TranquilityConfig::load_once();
    tranquility_figlet();
    match TranquilityCommand::try_parse() {
        Ok(args) => {
            handle_commands(args);
        }
        Err(err) => {
            handle_command_errors(err);
        }
    }
}
