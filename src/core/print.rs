// Module: Core/Print
// Location: cli/src/core/print.rs
use colored::Colorize;
use figlet_rs::FIGfont;

pub enum PrefixColor {
    RED,
    GREEN,
    YELLOW,
    BLUE,
}

pub fn tranquility_figlet() {
    let standard_font = FIGfont::standard().unwrap();
    let figure_1 = standard_font.convert("TRANQULITY");
    assert!(figure_1.is_some());
    println!("{}", figure_1.unwrap());
}

/// Prints a message with a color-coded prefix
pub fn print_with_prefix(color: PrefixColor, prefix: &'static str, message: String) {
    let message_prefix = prefix.bold();
    let colored_prefix = match color {
        PrefixColor::BLUE => message_prefix.blue(),
        PrefixColor::GREEN => message_prefix.green(),
        PrefixColor::RED => message_prefix.red(),
        PrefixColor::YELLOW => message_prefix.yellow(),
    };

    if prefix == "error" {
        eprintln!("{colored_prefix}: {message}");
    } else {
        println!("{colored_prefix}: {message}");
    }
}
