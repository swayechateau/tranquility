use colored::Colorize;

use crate::{
    core::font::is_font_installed, log_info, log_warn, models::font::NERD_FONT_LIST, print_info,
    print_success, print_warn,
};

use super::{install::install_nerd_font, refresh::auto_refresh, uninstall::uninstall_font};

/// 🔁 Update only installed fonts
pub fn update() {
    println!("🔁 Updating installed Nerd Fonts...");
    update_fonts();
    auto_refresh();
    print_success!("✅ Font update complete.");
}

/// 🔁 Reinstall only already-installed fonts
pub fn update_fonts() {
    println!("{}", "🔁 Updating installed Nerd Fonts...".green().bold());
    log_info!("update", "font", "started");
    let mut updated_any = false;
    for font in NERD_FONT_LIST.iter() {
        if is_font_installed(font) {
            print_info!("Updating {}", font);
            uninstall_font(font);
            install_nerd_font(font);
            updated_any = true;
        }
    }
    log_info!("update", "font", "completed");

    if !updated_any {
        print_warn!("No fonts were installed — nothing to update.");
        log_warn!("update", "font", "no_fonts");
    } else {
        print_success!("✅ Font update complete.");
        log_info!("update", "font", "completed");
    }
}
