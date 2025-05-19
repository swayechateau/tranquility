
// error
// success
// info
// warn
use colored::Colorize;

pub enum PrefixColor {
    RED,
    GREEN,
    YELLOW,
    BLUE,
    NORMAL
}

/// Error: prints error message
pub fn error(message: String) {
    print_with_prefix(PrefixColor::RED, "error", message);
}
/// Success: prints success message
pub fn success(message: String) {
    print_with_prefix(PrefixColor::GREEN, "success", message);
}
/// Info: prints informational message
pub fn info(message: String) {
    print_with_prefix(PrefixColor::BLUE, "info", message);
}
/// Warn: prints warning message
pub fn warn(message: String) {
    print_with_prefix(PrefixColor::YELLOW, "warn", message);
}

pub fn print_with_prefix(color: PrefixColor, prefix: &'static str, message: String) {
    let message_prefix= String::from(prefix).bold();
    let colored_prefix = match color {
        PrefixColor::BLUE => message_prefix.blue(),
        PrefixColor::GREEN => message_prefix.green(),
        PrefixColor::RED => message_prefix.red(),
        PrefixColor::YELLOW => message_prefix.yellow(),
        PrefixColor::NORMAL => message_prefix
    };
    println!("{colored_prefix}: {message}")
}