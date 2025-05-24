// src/fonts.rs
use std::{fs::{self, File}, io::Write, path::PathBuf, process::Command};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use dirs;
use reqwest::blocking::get;
use zip_extract::extract;

use crate::{logger::log_event, print_error, print_info, print_success, print_warn};

/// 📦 List of supported Nerd Fonts
pub const NERD_FONT_LIST: [&str; 69] = [
    "3270", "0xProto", "Agave", "AnonymousPro", "Arimo", "AurulentSansMono", "BigBlueTerminal", "BitstreamVeraSansMono",
    "CascadiaCode", "CascadiaMono", "CodeNewRoman", "ComicShannsMono", "CommitMono", "Cousine", "D2Coding", "DaddyTimeMono",
    "DejaVuSansMono", "DepartureMono", "DroidSansMono", "EnvyCodeR", "FantasqueSansMono", "FiraCode", "FiraMono", "FontPatcher",
    "GeistMono", "Go-Mono", "Gohu", "Hack", "Hasklig", "HeavyData", "Hermit", "iA-Writer", "IBMPlexMono", "Inconsolata", "InconsolataGo",
    "InconsolataLGC", "IntelOneMono", "Iosevka", "IosevkaTerm", "IosevkaTermSlab", "JetBrainsMono", "Lekton", "LiberationMono",
    "Lilex", "MartianMono", "Meslo", "Monaspace", "Monofur", "Monoid", "Mononoki", "MPlus", "NerdFontsSymbolsOnly", "Noto", "OpenDyslexic",
    "Overpass", "ProFont", "ProggyClean", "Recursive", "RobotoMono", "ShareTechMono", "SourceCodePro", "SpaceMono", "Terminus", "Tinos",
    "Ubuntu", "UbuntuMono", "UbuntuSans", "VictorMono", "ZedMono"
];

pub fn auto_refresh() {
    let status = Command::new("fc-cache")
        .args(["-f", "-v"])
        .status()
        .expect("Failed to run fc-cache");

    if status.success() {
        log_event("info", "fc-cache", "nerd-fonts", "success", None);
        print_success!("Font cache refreshed.");
    } else {
        log_event("error", "fc-cache", "nerd-fonts", "failed", None);
        print_error!("Failed to refresh font cache.");
    }
}

/// 🧱 Get system-specific Nerd Font install directory
pub fn get_font_dir() -> PathBuf {
    #[cfg(target_os = "linux")]
    {
        dirs::home_dir().unwrap().join(".local/share/fonts")
    }
    #[cfg(target_os = "macos")]
    {
        dirs::home_dir().unwrap().join("Library/Fonts")
    }
    #[cfg(target_os = "windows")]
    {
        dirs::data_dir().unwrap().join("Microsoft/Windows/Fonts")
    }
}

/// Check if a font is installed by looking for its folder
pub fn is_font_installed(font: &str) -> bool {
    get_font_dir().join(font).exists()
}

/// Display table of fonts and their status
pub fn list_fonts(show_installed: Option<bool>) {
    use tabled::{Table, Tabled, settings::Style};

    #[derive(Tabled)]
    struct FontRow<'a> {
        #[tabled(rename = "Font")]
        name: &'a str,
        #[tabled(rename = "Status")]
        status: &'a str,
    }

    let rows: Vec<FontRow> = NERD_FONT_LIST
        .iter()
        .filter_map(|font| {
            let installed = is_font_installed(font);
            let include = match show_installed {
                Some(true) => installed,
                Some(false) => !installed,
                None => true,
            };

            if include {
                Some(FontRow {
                    name: font,
                    status: if installed { "✅ Installed" } else { "❌ Not Installed" },
                })
            } else {
                None
            }
        })
        .collect();

    if rows.is_empty() {
        match show_installed {
            Some(true) => print_warn!("No fonts installed."),
            Some(false) => print_warn!("All fonts are installed."),
            None => print_warn!("No fonts found."),
        }
        return;
    }

    let mut table = Table::new(rows);
    table.with(Style::modern());
    println!("{}", table);
}


/// 🖋 Install a Nerd Font by name
pub fn install_nerd_font(font: &str) {
    if !NERD_FONT_LIST.contains(&font) {
        print_error!("Invalid font name: {}", font);
        log_event("error", "install", font, "invalid_font", None);
        return;
    }

    if is_font_installed(font) {
        print_warn!("{} is already installed.", font);
        log_event("info", "install", font, "skipped", Some(0.0));
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
    log_event("info", "install", font, "downloading", None);
    match get(&font_url) {
        Ok(response) => {
            let mut file = File::create(&zip_path).expect("Failed to create ZIP file");
            let content = response.bytes().expect("Failed to read ZIP content");
            file.write_all(&content).expect("Failed to write ZIP file");
        }
        Err(e) => {
            print_error!("❌ Download failed: {}", e);
            log_event("error", "install", font, "download_failed", None);
            return;
        }
    }

    print_info!("Extracting...");
    if let Err(e) = extract(File::open(&zip_path).unwrap(), &fonts_dir, true) {
        print_error!("❌ Extraction failed: {}", e);
        log_event("error", "install", font, "unzip_failed", None);
        return;
    }

    print_success!("✅ Installed {}", font);
    log_event("info", "install", font, "success", None);
}

/// 💬 Prompt-based installer
pub fn choose_and_install_fonts(all: bool) {
    if all {
        println!("{}", "Installing all Nerd Fonts...".green().bold());
        log_event("info", "install", "fonts", "all", None);
        for font in NERD_FONT_LIST.iter() {
            install_nerd_font(font);
        }
        return;
    }

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select fonts to install")
        .items(&NERD_FONT_LIST)
        .interact()
        .unwrap_or_else(|_| {
            print_error!("❌ Font selection failed.");
            log_event("error", "select", "font", "selection_failed", None);
            std::process::exit(1);
        });

    if selections.is_empty() {
        print_warn!("No fonts selected.");
        log_event("warn", "select", "font", "no_selection", None);
        return;
    }

    for i in selections {
        install_nerd_font(NERD_FONT_LIST[i]);
        log_event("info", "select", "font", NERD_FONT_LIST[i], None);
    }
    log_event("info", "install", "fonts", "installation_completed", None);
}

/// 🔁 Reinstall only already-installed fonts
pub fn update_fonts() {
    println!("{}", "🔁 Updating installed Nerd Fonts...".green().bold());
    log_event("info", "update", "fonts", "start", None);
    let mut updated_any = false;
    for font in NERD_FONT_LIST.iter() {
        if is_font_installed(font) {
            print_info!("Updating {}", font);
            uninstall_font(font);
            install_nerd_font(font);
            updated_any = true;
        }
    }

    if !updated_any {
        print_warn!("No fonts were installed — nothing to update.");
        log_event("warn", "update", "fonts", "no_fonts", None);
    } else {
        print_success!("✅ Font update complete.");
        log_event("info", "update", "fonts", "complete", None);
    }
}

/// 🗑️ Uninstall a font by removing its folder
pub fn uninstall_font(font: &str) {
    if !is_font_installed(font) {
        print_warn!("{} is not installed.", font);
        log_event("info", "uninstall", font, "skipped as not installed", None);
        return;
    }
    let font_dir = get_font_dir().join(font);
    if font_dir.exists() {
        if fs::remove_dir_all(&font_dir).is_ok() {
            print_success!("🗑️ Uninstalled font: {}", font);
            log_event("info", "uninstall", font, "success", None);
        } else {
            print_error!("Failed to remove font: {}", font);
            log_event("error", "uninstall", font, "failed", None);
        }
    } else {
        print_warn!("Font not found: {}", font);
        log_event("info", "uninstall", font, "not_found", None);
    }

    log_event("info", "uninstall", font, "success", None);
}

/// 🔘 Prompt to uninstall fonts
pub fn choose_and_uninstall_fonts(all: bool) {
    if all {
        println!("{}", "Uninstalling all Nerd Fonts...".green().bold());
        log_event("info", "uninstall", "fonts", "all", None);
        for font in NERD_FONT_LIST.iter() {
            uninstall_font(font);
        }
        return;
    }
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select fonts to uninstall")
        .items(&NERD_FONT_LIST)
        .interact()
        .unwrap_or_else(|_| {
            print_error!("❌ Prompt failed.");
            log_event("error", "uninstall", "font", "prompt_failed", None);
            std::process::exit(1);
        });

    if selections.is_empty() {
        print_warn!("No fonts selected.");
        log_event("warn", "uninstall", "font", "no_selection", None);
        return;
    }

    for i in selections {
        uninstall_font(NERD_FONT_LIST[i]);
    }

}
