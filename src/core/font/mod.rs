use std::path::PathBuf;

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