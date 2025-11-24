// Module: Command/Font/Install
// Location: cli/src/command/font/install.rs

use colored::Colorize;
use reqwest::blocking::get;
use std::{
    fs::{self, File},
    io::Write,
};

use dialoguer::{MultiSelect, theme::ColorfulTheme};

use super::refresh::auto_refresh;
use crate::{
    core::{
        font::{get_font_dir, is_font_installed},
        zip,
    },
    log_error, log_info, log_warn,
    models::font::NERD_FONT_LIST,
    print_error, print_info, print_success, print_warn,
};

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

/// 🖋 Install a Nerd Font by name
pub fn install_nerd_font(font: &str) {
    if !NERD_FONT_LIST.contains(&font) {
        print_error!("Invalid font name: {}", font);
        log_error!("install", font, "invalid_font");
        return;
    }

    if is_font_installed(font) {
        print_warn!("{} is already installed.", font);
        log_info!("install", font, "skipped");
        return;
    }

    let font_url = format!(
        "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/{}.zip",
        font
    );

    let fonts_dir = get_font_dir().join(font);
    fs::create_dir_all(&fonts_dir).expect("Failed to create font directory");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let zip_path = temp_dir.path().join(format!("{}.zip", font));

    print_info!("Downloading {}", font);
    log_info!("install", font, "downloading");
    match get(&font_url) {
        Ok(response) => {
            let mut file = File::create(&zip_path).expect("Failed to create ZIP file");
            let content = response.bytes().expect("Failed to read ZIP content");
            file.write_all(&content).expect("Failed to write ZIP file");
        }
        Err(e) => {
            print_error!("❌ Download failed: {}", e);
            log_error!("install", font, &format!("download_failed: {}", e));
            return;
        }
    }

    print_info!("Extracting...");
    if let Err(e) = zip::extract(File::open(&zip_path).unwrap(), &fonts_dir, true) {
        print_error!("❌ Extraction failed: {}", e);
        log_error!("install", font, &format!("unzip_failed: {}", e));
        return;
    }

    print_success!("✅ Installed {}", font);
    log_info!("install", font, "success");
}

/// 💬 Prompt-based installer
pub fn choose_and_install_fonts(all: bool) {
    if all {
        println!("{}", "Installing all Nerd Fonts...".green().bold());
        log_info!("install", "fonts", "all");
        for font in NERD_FONT_LIST.iter() {
            install_nerd_font(font);
        }
        return;
    }

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select fonts to install")
        .items(NERD_FONT_LIST)
        .interact()
        .unwrap_or_else(|_| {
            print_error!("❌ Font selection failed.");
            log_error!("select", "font", "selection_failed");
            std::process::exit(1);
        });

    if selections.is_empty() {
        print_warn!("No fonts selected.");
        log_warn!("select", "font", "no_selection");
        return;
    }

    for i in selections {
        install_nerd_font(NERD_FONT_LIST[i]);
        log_info!("select", "font", NERD_FONT_LIST[i]);
    }
    log_info!("install", "font", "installation_completed");
}
