use crate::{core::font::is_font_installed, log_warn, models::font::NERD_FONT_LIST};

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
                    status: if installed {
                        "✅ Installed"
                    } else {
                        "❌ Not Installed"
                    },
                })
            } else {
                None
            }
        })
        .collect();

    if rows.is_empty() {
        match show_installed {
            Some(true) => log_warn!("list", "fonts", "No fonts installed."),
            Some(false) => log_warn!("list", "fonts", "All fonts are installed."),
            None => log_warn!("list", "fonts", "No fonts found."),
        }
        return;
    }

    let mut table = Table::new(rows);
    table.with(Style::modern());
    println!("{}", table);
}
