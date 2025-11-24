use std::fs;

use crate::{
    core::font::{get_font_dir, is_font_installed},
    log_error, log_info, log_warn,
    models::font::NERD_FONT_LIST,
    print_info, print_success, print_warn,
};
use colored::Colorize;
use dialoguer::{MultiSelect, theme::ColorfulTheme};

use super::refresh::auto_refresh;
/// 🗑️ Handle font uninstall command
pub fn uninstall(all: bool, names: Vec<String>) {
    if all {
        choose_and_uninstall_fonts(true);
    } else if !names.is_empty() {
        for font in names {
            uninstall_font(&font);
        }
    } else {
        choose_and_uninstall_fonts(false);
    }
    auto_refresh();
    print_success!("✅ Font uninstall complete.");
}

/// 🗑️ Uninstall a font by removing its folder
pub fn uninstall_font(font: &str) {
    if !is_font_installed(font) {
        print_warn!("{} is not installed.", font);
        log_info!("uninstall", font, "skipped as not installed");
        return;
    }
    let font_dir = get_font_dir().join(font);
    if font_dir.exists() {
        if fs::remove_dir_all(&font_dir).is_ok() {
            print_success!("🗑️ Uninstalled font: {}", font);
            log_info!("uninstall", font, "success");
        } else {
            print_warn!("Failed to remove font: {}", font);
            log_error!("uninstall", font, "failed");
        }
    } else {
        print_warn!("Font not found: {}", font);
        log_info!("uninstall", font, "not_found");
    }

    log_info!("uninstall", font, "success");
}

/// 🔘 Prompt to uninstall fonts
pub fn choose_and_uninstall_fonts(all: bool) {
    if all {
        println!("{}", "Uninstalling all Nerd Fonts...".green().bold());
        log_info!("uninstall", "fonts", "all");
        for font in NERD_FONT_LIST.iter() {
            uninstall_font(font);
        }
        return;
    }
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select fonts to uninstall")
        .items(NERD_FONT_LIST)
        .interact()
        .unwrap_or_else(|_| {
            print_info!("❌ Prompt failed.");
            log_error!("uninstall", "font", "prompt_failed");
            std::process::exit(1);
        });

    if selections.is_empty() {
        print_warn!("No fonts selected.");
        log_warn!("uninstall", "font", "no_selection");
        return;
    }

    for i in selections {
        uninstall_font(NERD_FONT_LIST[i]);
    }
}
