// src/models/categories.rs
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use strum::{Display, EnumIter, IntoEnumIterator};

/// Software categories
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, ValueEnum, Deserialize, Serialize, EnumIter, Display, JsonSchema,
)]
#[serde(rename_all = "PascalCase")]
pub enum Category {
    Fonts,
    PackageManagement,
    Shells,
    Browsers,
    Servers,
    TerminalEmulators,
    PasswordManagement,
    Encryption,
    RemoteDesktop,
    VPN,
    DownloadManagement,
    Imaging,
    WindowManagement,
    CLITools,
    Customization,
    Communication,
    Creative,
    Productivity,
    Utilities,
    Office,
    OfficeAddons,
    NoteTaking,
    TaskManagement,
    Virtualization,
    Gaming,
    Networking,
    Essential,
    Development,
    Recording,
    Streaming,
    DatabaseManagement,
    ProgrammingLanguages,
    Editors,
    Containerization,
    Engines,
    AI,
    DevTools,
}

impl Category {
    /// Returns a human-readable display name
    pub fn display(&self) -> String {
        match self {
            Category::TerminalEmulators => "Terminal Emulators".to_string(),
            Category::PasswordManagement => "Password Managers".to_string(),
            Category::PackageManagement => "Package Managers".to_string(),
            Category::DownloadManagement => "Download Managers".to_string(),
            Category::DatabaseManagement => "Database Management".to_string(),
            Category::ProgrammingLanguages => "Programming Languages".to_string(),
            Category::DevTools => "Development Tools".to_string(),
            _ => self.to_string(),
        }
    }
}

/// Print out all categories with display names
pub fn list_categories() {
    println!("📦 Available Categories:");
    for category in Category::iter() {
        println!("- {}", category.display());
    }
}
