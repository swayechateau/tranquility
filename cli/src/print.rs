// src/print.rs
use colored::Colorize;

pub enum PrefixColor {
    RED,
    GREEN,
    YELLOW,
    BLUE,
}

/// Prints a message with a color-coded prefix
pub fn print_with_prefix(color: PrefixColor, prefix: &'static str, message: String) {
    let message_prefix = prefix.bold();
    let colored_prefix = match color {
        PrefixColor::BLUE => message_prefix.blue(),
        PrefixColor::GREEN => message_prefix.green(),
        PrefixColor::RED => message_prefix.red(),
        PrefixColor::YELLOW => message_prefix.yellow()
    };

    if prefix == "error" {
        eprintln!("{colored_prefix}: {message}");
    } else {
        println!("{colored_prefix}: {message}");
    }
}

#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {
        $crate::print::print_with_prefix($crate::print::PrefixColor::RED, "error", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! print_success {
    ($($arg:tt)*) => {
        $crate::print::print_with_prefix($crate::print::PrefixColor::GREEN, "success", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! print_info {
    ($($arg:tt)*) => {
        $crate::print::print_with_prefix($crate::print::PrefixColor::BLUE, "info", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! print_warn {
    ($($arg:tt)*) => {
        $crate::print::print_with_prefix($crate::print::PrefixColor::YELLOW, "warn", format!($($arg)*));
    };
}