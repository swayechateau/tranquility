// src/commad/font.rs

use crate::{fonts::{auto_refresh, choose_and_install_fonts, choose_and_uninstall_fonts, install_nerd_font, list_fonts, uninstall_font, update_fonts, NERD_FONT_LIST}, print_success};

pub fn install(all: bool, name: Vec<String>) {
    if all {
        choose_and_install_fonts(true);
    } else if !name.is_empty() {
        for font in name {
            install_nerd_font(&font);
        }
    } else {
        choose_and_install_fonts(false);
    }
    auto_refresh();
    print_success!("✅ Font installation complete.");
}

pub fn uninstall(all: bool, name: Vec<String>) {
    if all {
        choose_and_uninstall_fonts(all);
    } else if !name.is_empty() {
        for font in name {
            uninstall_font(&font);
        }
    } else {
        choose_and_uninstall_fonts(false);
    }
    auto_refresh();
    print_success!("✅ Font uninstall complete.");
}

pub fn update() {
    println!("🔁 Updating installed Nerd Fonts...");
    update_fonts();
}

pub fn list(installed_only: bool) {
    println!("📦 Available Nerd Fonts:");
    list_fonts(installed_only);
}
