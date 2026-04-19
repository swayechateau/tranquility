pub mod font;
pub mod logger;
pub mod print;
pub mod shell;
pub mod zip;

pub fn expand_home(input: &str) -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| "~".to_string());

    if input.starts_with("~/") {
        return input.replacen("~", &home, 1);
    }

    if input.contains("$HOME") {
        return input.replace("$HOME", &home);
    }

    if input.contains("%HOME%") {
        return input.replace("%HOME%", &home);
    }

    input.to_string()
}
