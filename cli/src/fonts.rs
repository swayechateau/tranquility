// src/fonts.rs
use std::{fs, io};
use std::io::Write;
use std::process::Command;
use colored::Colorize;
use dirs;
use crate::{print_error, print_info, print_success};

// List of Nerd Fonts
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

/// Install a Nerd Font by name
pub fn install_nerd_font(font: &str) {
    if !NERD_FONT_LIST.contains(&font) {
        print_error!("Invalid font name: {}", font);
        return;
    }

    let font_version = "v3.3.0";
    let font_url = format!(
        "https://github.com/ryanoasis/nerd-fonts/releases/download/{}/{}.zip",
        font_version, font
    );

    let fonts_dir = dirs::home_dir()
        .expect("Unable to determine home directory")
        .join(".local/share/fonts")
        .join(font);

    fs::create_dir_all(&fonts_dir).expect("Failed to create font directory");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let zip_path = temp_dir.path().join(format!("{}.zip", font));

    print_info!("Downloading {}", font);
    let status = Command::new("wget")
        .arg("-q")
        .arg("-O")
        .arg(&zip_path)
        .arg(&font_url)
        .status()
        .expect("Failed to run wget");

    if !status.success() {
        print_error!("Download failed for {}", font);
        return;
    }

    print_info!("Extracting...");
    let unzip_status = Command::new("unzip")
        .arg("-q")
        .arg(zip_path.to_str().unwrap())
        .arg("-d")
        .arg(temp_dir.path())
        .status();

    if !unzip_status.expect("Failed to run unzip").success() {
        print_error!("Extraction failed for {}", font);
        return;
    }

    for entry in fs::read_dir(temp_dir.path()).unwrap() {
        let path = entry.unwrap().path();
        if let Some(ext) = path.extension() {
            if ext == "ttf" || ext == "otf" {
                let _ = fs::copy(&path, fonts_dir.join(path.file_name().unwrap()));
            }
        }
    }

    // Refresh font cache
    let cache_status = Command::new("fc-cache")
        .arg("-fv")
        .status();

    if let Ok(status) = cache_status {
        if !status.success() {
            print_error!("Font cache refresh failed after installing {}", font);
        }
    }

    print_success!("{} installed.", font);
}

/// Interactive font chooser or auto-installer if `all == true`
pub fn choose_and_install_fonts(all: bool) {
    if all {
        println!("{}", "Installing all Nerd Fonts...".green().bold());
        for font in NERD_FONT_LIST.iter() {
            install_nerd_font(font);
        }
        return;
    }

    println!("{}", "Welcome to Nerd Font Installer!".bold());
    println!("You can:");
    println!("1. Install a single font");
    println!("2. Install multiple fonts (comma-separated)");
    println!("3. Install all fonts");
    print!("Enter your choice (1/2/3): ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    match choice {
        "1" => {
            print!("Enter the font name: ");
            io::stdout().flush().unwrap();
            let mut font = String::new();
            io::stdin().read_line(&mut font).unwrap();
            install_nerd_font(font.trim());
        }
        "2" => {
            print!("Enter font names separated by commas: ");
            io::stdout().flush().unwrap();
            let mut fonts = String::new();
            io::stdin().read_line(&mut fonts).unwrap();
            let font_list: Vec<&str> = fonts
                .trim()
                .split(',')
                .map(|f| f.trim())
                .filter(|f| !f.is_empty())
                .collect();
            for font in font_list {
                install_nerd_font(font);
            }
        }
        "3" => {
            println!("Installing all {} fonts...", NERD_FONT_LIST.len());
            for font in NERD_FONT_LIST.iter() {
                install_nerd_font(font);
            }
        }
        _ => {
            print_error!("Invalid choice.");
        }
    }
}
