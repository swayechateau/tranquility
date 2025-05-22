// src/fonts.rs
use std::{fs, path::PathBuf, process::Command};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use dirs;

use crate::{print_error, print_info, print_success, print_warn};

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
        print_success!("Font cache refreshed.");
    } else {
        print_error!("Failed to refresh font cache.");
    }
}

/// 🖋 Install a Nerd Font by name
pub fn install_nerd_font(font: &str) {
    if !NERD_FONT_LIST.contains(&font) {
        print_error!("Invalid font name: {}", font);
        return;
    }

    if is_font_installed(font) {
        print_warn!("{} is already installed.", font);
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
    let status = Command::new("wget")
        .args(["-q", "-O", zip_path.to_str().unwrap(), &font_url])
        .status()
        .expect("Failed to run wget");

    if !status.success() {
        print_error!("Download failed for {}", font);
        return;
    }

    print_info!("Extracting...");
    let unzip_status = Command::new("unzip")
        .args(["-q", zip_path.to_str().unwrap(), "-d", temp_dir.path().to_str().unwrap()])
        .status()
        .expect("Failed to run unzip");

    if !unzip_status.success() {
        print_error!("Extraction failed for {}", font);
        return;
    }

    for entry in fs::read_dir(temp_dir.path()).unwrap() {
        let path = entry.unwrap().path();
        if matches!(path.extension().and_then(|e| e.to_str()), Some("ttf" | "otf")) {
            let _ = fs::copy(&path, fonts_dir.join(path.file_name().unwrap()));
        }
    }

    print_success!("{} installed.", font);
}

/// 💬 Prompt-based installer
pub fn choose_and_install_fonts(all: bool) {
    if all {
        println!("{}", "Installing all Nerd Fonts...".green().bold());
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
            std::process::exit(1);
        });

    if selections.is_empty() {
        print_warn!("No fonts selected.");
        return;
    }

    for i in selections {
        install_nerd_font(NERD_FONT_LIST[i]);
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


/// 🗑️ Uninstall a font by removing its folder
pub fn uninstall_font(font: &str) {
    if !is_font_installed(font) {
        print_warn!("{} is not installed.", font);
        return;
    }
    let font_dir = get_font_dir().join(font);
    if font_dir.exists() {
        if fs::remove_dir_all(&font_dir).is_ok() {
            print_success!("🗑️ Uninstalled font: {}", font);
        } else {
            print_error!("Failed to remove font: {}", font);
        }
    } else {
        print_warn!("Font not found: {}", font);
    }
}

/// 🔘 Prompt to uninstall fonts
pub fn choose_and_uninstall_fonts(all: bool) {
    if all {
        println!("{}", "Uninstalling all Nerd Fonts...".green().bold());
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
            std::process::exit(1);
        });

    if selections.is_empty() {
        print_warn!("No fonts selected.");
        return;
    }

    for i in selections {
        uninstall_font(NERD_FONT_LIST[i]);
    }

}

use tabled::{Table, Tabled, settings::Style};

/// Check if a font is installed by looking for its folder
pub fn is_font_installed(font: &str) -> bool {
    get_font_dir().join(font).exists()
}

/// Display table of fonts and their status
pub fn list_fonts(installed_only: bool) {
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
            let show = (installed_only && installed) || (!installed_only && !installed);

            if !show {
                return None;
            }

            Some(FontRow {
                name: font,
                status: if installed { "✅ Installed" } else { "❌ Not Installed" },
            })
        })
        .collect();

    if rows.is_empty() {
        if installed_only {
            print_warn!("No installed fonts found.");
        } else {
            print_warn!("All fonts are already installed.");
        }
        return;
    }

    let mut table = Table::new(rows);
    table.with(Style::modern());
    println!("{}", table);
}

/// 🔁 Reinstall only already-installed fonts
pub fn update_fonts() {
    println!("{}", "🔁 Updating installed Nerd Fonts...".green().bold());

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
    } else {
        print_success!("✅ Font update complete.");
    }
}