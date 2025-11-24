use clap::Parser;
use tranquility::{
    cli::{TranquilityCommand, handle_command_errors, handle_commands},
    config,
    core::print::tranquility_figlet,
};

fn main() {
    config::TranquilityConfig::load_once();
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
