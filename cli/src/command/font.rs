// src/commad/font.rs

use crate::fonts::{
    auto_refresh,
    choose_and_install_fonts,
    choose_and_uninstall_fonts,
    install_nerd_font,
    list_fonts,
    uninstall_font,
    update_fonts,
};
use crate::print_success;

/// 🎯 Handle font install command
pub fn install(all: bool, names: Vec<String>) {
    if all {
        choose_and_install_fonts(true);
    } else if !names.is_empty() {
        for font in names {
            install_nerd_font(&font);
        }
    } else {
        choose_and_install_fonts(false);
    }
    auto_refresh();
    print_success!("✅ Font installation complete.");
}

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

/// 🔁 Update only installed fonts
pub fn update() {
    println!("🔁 Updating installed Nerd Fonts...");
    update_fonts();
    auto_refresh();
    print_success!("✅ Font update complete.");
}

/// 📦 List fonts based on filter
pub fn list(installed: bool, all: bool) {
    if all {
        println!("📦 Listing all Nerd Fonts:");
        list_fonts(None);
        return;
    }
    if installed {
        println!("✅ Listing only installed fonts:");
        list_fonts(Some(true));
        return;
    }
    println!("📄 Listing only available (not installed) fonts:");
    list_fonts(Some(false));
}
