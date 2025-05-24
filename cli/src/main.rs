//src/main.rs
mod args;
mod model;
mod fonts;
mod shell;
mod command;
mod logger;
mod schema;
mod print;
#[macro_use]
mod libs;
use model::{categories, config, package_manager, system::{self, SystemInfo}};
use clap::{Parser};
use args::{handle_arg_errors, handle_args, TranquilityArgs};
use print::tranquility_figlet;
use logger::{log_event};

fn main() {
    tranquility_figlet();
    let sys = SystemInfo::new();
    println!("{}", sys.to_pretty_string());

    log_event("info", "Starting", "tranquility", "success", None);
    match TranquilityArgs::try_parse() {
        Ok(args) => {
            handle_args(args);
        }
        Err(err) => {
            handle_arg_errors(err);
        }
    }
}

