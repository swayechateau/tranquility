mod args;
mod model;
mod fonts;
mod shell;
mod command;
mod logging;
#[macro_use]
mod print;
mod config;
use model::{categories,package_manager,system};
use clap::{Parser};
use args::{handle_arg_errors, handle_args, TranquilityArgs};
use figlet_rs::FIGfont;

fn main() {
    tranquility_figlet();
    match TranquilityArgs::try_parse() {
        Ok(args) => {
            handle_args(args);
        }
        Err(err) => {
            handle_arg_errors(err);
        }
    }
}

fn tranquility_figlet() {
    let standard_font = FIGfont::standard().unwrap();
    let figure_1 = standard_font.convert("TRANQULITY");
    assert!(figure_1.is_some());
    println!("{}", figure_1.unwrap());
}