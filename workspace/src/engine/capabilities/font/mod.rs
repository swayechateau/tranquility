use std::{fs, io, path::Path, path::PathBuf};

use crate::engine::error::{Error, ErrorCode};
use crate::engine::models::font::NERD_FONT_LIST;

// ---------------------------------------------------------------------------
// Font directory helpers
// ---------------------------------------------------------------------------

pub fn get_font_dir() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        dirs::home_dir().unwrap_or_default().join("Library/Fonts")
    }
    #[cfg(target_os = "windows")]
    {
        dirs::data_dir()
            .unwrap_or_default()
            .join("Microsoft/Windows/Fonts")
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        dirs::home_dir()
            .unwrap_or_default()
            .join(".local/share/fonts")
    }
}

pub fn is_font_installed(font: &str) -> bool {
    get_font_dir().join(font).exists()
}

fn refresh_font_cache() {
    #[cfg(target_os = "linux")]
    {
        let status = std::process::Command::new("fc-cache")
            .args(["-f", "-v"])
            .status();
        match status {
            Ok(s) if s.success() => tracing::info!("Font cache refreshed."),
            Ok(s) => tracing::warn!("fc-cache exited with status: {}", s),
            Err(e) => tracing::warn!("Failed to run fc-cache: {}", e),
        }
    }
}

// ---------------------------------------------------------------------------
// Zip extraction
// ---------------------------------------------------------------------------

fn extract_zip(data: &[u8], target_dir: &Path) -> Result<(), Error> {
    use std::io::Cursor;

    let cursor = Cursor::new(data);
    let mut archive = zip::ZipArchive::new(cursor).map_err(|e| {
        Error::from_code(ErrorCode::ProcessFailure).with_context("zip_open", e.to_string())
    })?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| {
            Error::from_code(ErrorCode::ProcessFailure).with_context("zip_read", e.to_string())
        })?;

        let outpath = match file.enclosed_name() {
            Some(p) => target_dir.join(p),
            None => continue,
        };

        // Skip hidden files/dirs
        if outpath
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with('.'))
            .unwrap_or(false)
        {
            continue;
        }

        if file.is_dir() {
            fs::create_dir_all(&outpath).map_err(|e| {
                Error::from_code(ErrorCode::ProcessFailure).with_context("io", e.to_string())
            })?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent).map_err(|e| {
                    Error::from_code(ErrorCode::ProcessFailure).with_context("io", e.to_string())
                })?;
            }
            let mut outfile = fs::File::create(&outpath).map_err(|e| {
                Error::from_code(ErrorCode::ProcessFailure).with_context("io", e.to_string())
            })?;
            io::copy(&mut file, &mut outfile).map_err(|e| {
                Error::from_code(ErrorCode::ProcessFailure).with_context("io", e.to_string())
            })?;
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Per-font install / uninstall helpers
// ---------------------------------------------------------------------------

async fn install_one(font: &str, dry_run: bool) -> Result<(), Error> {
    if is_font_installed(font) {
        println!("  ⏭  {}: already installed — skipping", font);
        return Ok(());
    }

    if dry_run {
        println!("  [dry run] would install: {}", font);
        return Ok(());
    }

    let font_url = format!(
        "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/{}.zip",
        font
    );

    let font_dir = get_font_dir().join(font);
    fs::create_dir_all(&font_dir).map_err(|e| {
        Error::from_code(ErrorCode::ProcessFailure).with_context("io", e.to_string())
    })?;

    println!("  ⬇  Downloading {}...", font);
    let response = reqwest::get(&font_url).await.map_err(|e| {
        Error::from_code(ErrorCode::ProcessFailure).with_context("download", e.to_string())
    })?;

    if !response.status().is_success() {
        return Err(Error::from_code(ErrorCode::ProcessFailure).with_context(
            "download",
            format!("HTTP {} for {}", response.status(), font),
        ));
    }

    let bytes = response.bytes().await.map_err(|e| {
        Error::from_code(ErrorCode::ProcessFailure).with_context("read_bytes", e.to_string())
    })?;

    println!("  📦 Extracting {}...", font);
    extract_zip(&bytes, &font_dir)?;

    println!("  ✅ Installed {}", font);
    Ok(())
}

fn uninstall_one(font: &str, dry_run: bool) {
    if !is_font_installed(font) {
        println!("  ⏭  {}: not installed — skipping", font);
        return;
    }

    if dry_run {
        println!("  [dry run] would uninstall: {}", font);
        return;
    }

    let font_dir = get_font_dir().join(font);
    if fs::remove_dir_all(&font_dir).is_ok() {
        println!("  🗑  Uninstalled {}", font);
    } else {
        println!("  ❌ Failed to remove {} — check permissions", font);
    }
}

// ---------------------------------------------------------------------------
// Public capability functions
// ---------------------------------------------------------------------------

/// Install one or more Nerd Fonts.  When `all` is false and `names` is empty,
/// an interactive multi-select prompt is shown (requires a TTY).
pub async fn install(
    dry_run: bool,
    interactive: bool,
    all: bool,
    names: Vec<String>,
) -> Result<(), Error> {
    let fonts_to_install: Vec<&'static str> = if all {
        NERD_FONT_LIST.iter().copied().collect()
    } else if !names.is_empty() {
        names
            .iter()
            .filter_map(|n| NERD_FONT_LIST.iter().copied().find(|&f| f == n.as_str()))
            .collect()
    } else if interactive {
        let selected_indices = tokio::task::spawn_blocking(|| {
            let theme = dialoguer::theme::ColorfulTheme::default();
            dialoguer::MultiSelect::with_theme(&theme)
                .with_prompt("Select fonts to install")
                .items(&NERD_FONT_LIST)
                .interact()
                .unwrap_or_default()
        })
        .await
        .map_err(|e| {
            Error::from_code(ErrorCode::ProcessFailure).with_context("prompt", e.to_string())
        })?;

        selected_indices
            .iter()
            .map(|&i| NERD_FONT_LIST[i])
            .collect()
    } else {
        eprintln!("No fonts specified. Use --all or provide font names.");
        return Ok(());
    };

    if fonts_to_install.is_empty() {
        println!("Nothing to install.");
        return Ok(());
    }

    println!("Installing {} font(s)...", fonts_to_install.len());
    for font in fonts_to_install {
        install_one(font, dry_run).await?;
    }
    refresh_font_cache();

    Ok(())
}

/// Uninstall one or more installed Nerd Fonts.
pub async fn uninstall(
    dry_run: bool,
    interactive: bool,
    all: bool,
    names: Vec<String>,
) -> Result<(), Error> {
    let fonts_to_remove: Vec<&'static str> = if all {
        NERD_FONT_LIST
            .iter()
            .copied()
            .filter(|&f| is_font_installed(f))
            .collect()
    } else if !names.is_empty() {
        names
            .iter()
            .filter_map(|n| NERD_FONT_LIST.iter().copied().find(|&f| f == n.as_str()))
            .collect()
    } else if interactive {
        let installed: Vec<&'static str> = NERD_FONT_LIST
            .iter()
            .copied()
            .filter(|&f| is_font_installed(f))
            .collect();

        if installed.is_empty() {
            println!("No Nerd Fonts are currently installed.");
            return Ok(());
        }

        let installed_clone = installed.clone();
        let selected_indices = tokio::task::spawn_blocking(move || {
            let theme = dialoguer::theme::ColorfulTheme::default();
            dialoguer::MultiSelect::with_theme(&theme)
                .with_prompt("Select fonts to uninstall")
                .items(&installed_clone)
                .interact()
                .unwrap_or_default()
        })
        .await
        .map_err(|e| {
            Error::from_code(ErrorCode::ProcessFailure).with_context("prompt", e.to_string())
        })?;

        selected_indices.iter().map(|&i| installed[i]).collect()
    } else {
        eprintln!("No fonts specified. Use --all or provide font names.");
        return Ok(());
    };

    if fonts_to_remove.is_empty() {
        println!("Nothing to uninstall.");
        return Ok(());
    }

    println!("Uninstalling {} font(s)...", fonts_to_remove.len());
    for font in fonts_to_remove {
        uninstall_one(font, dry_run);
    }
    refresh_font_cache();

    Ok(())
}

/// Reinstall all currently-installed Nerd Fonts.
pub async fn update(dry_run: bool) -> Result<(), Error> {
    let installed: Vec<&'static str> = NERD_FONT_LIST
        .iter()
        .copied()
        .filter(|&f| is_font_installed(f))
        .collect();

    if installed.is_empty() {
        println!("No Nerd Fonts installed — nothing to update.");
        return Ok(());
    }

    println!("Updating {} installed font(s)...", installed.len());
    for font in &installed {
        uninstall_one(font, dry_run);
        install_one(font, dry_run).await?;
    }
    refresh_font_cache();
    println!("Font update complete.");

    Ok(())
}

/// List Nerd Fonts in a table.
///
/// - `installed_only = true`  → show only installed fonts
/// - `all = true`             → show all fonts with install status
/// - otherwise                → show only fonts that are NOT installed
/// Row data for font listing — returned to callers for format-aware rendering.
pub struct FontListRow {
    pub name: &'static str,
    pub installed: bool,
}

/// Return rows for font listing. Callers are responsible for rendering.
pub fn font_list_rows(installed_only: bool, all: bool) -> Vec<FontListRow> {
    NERD_FONT_LIST
        .iter()
        .copied()
        .filter_map(|font| {
            let installed = is_font_installed(font);
            let include = if all {
                true
            } else if installed_only {
                installed
            } else {
                !installed
            };
            if include {
                Some(FontListRow {
                    name: font,
                    installed,
                })
            } else {
                None
            }
        })
        .collect()
}
