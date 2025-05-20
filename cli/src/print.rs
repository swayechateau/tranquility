// src/print.rs
use colored::Colorize;

pub enum PrefixColor {
    RED,
    GREEN,
    YELLOW,
    BLUE,
    NORMAL,
}

/// Prints an error message to stderr with a red "error" prefix
pub fn print_error(message: String) {
    print_with_prefix(PrefixColor::RED, "error", message);
}

/// Prints a success message with a green "success" prefix
pub fn print_success(message: String) {
    print_with_prefix(PrefixColor::GREEN, "success", message);
}

/// Prints an info message with a blue "info" prefix
pub fn print_info(message: String) {
    print_with_prefix(PrefixColor::BLUE, "info", message);
}

/// Prints a warning message with a yellow "warn" prefix
pub fn print_warn(message: String) {
    print_with_prefix(PrefixColor::YELLOW, "warn", message);
}

/// Prints a message with a color-coded prefix
pub fn print_with_prefix(color: PrefixColor, prefix: &'static str, message: String) {
    let message_prefix = prefix.bold();
    let colored_prefix = match color {
        PrefixColor::BLUE => message_prefix.blue(),
        PrefixColor::GREEN => message_prefix.green(),
        PrefixColor::RED => message_prefix.red(),
        PrefixColor::YELLOW => message_prefix.yellow(),
        PrefixColor::NORMAL => message_prefix,
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