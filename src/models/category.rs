// Module: Model/Category
// Location: cli/src/model/category.rs
use clap::ValueEnum;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};
use tabled::{Table, Tabled, settings::Style};

/// Software categories
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    ValueEnum,
    Deserialize,
    Serialize,
    EnumIter,
    Display,
    JsonSchema,
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

    /// Returns the ValueEnum representation (kebab-case CLI style)
    pub fn cli_name(&self) -> String {
        format!("{self:?}").to_lowercase().replace('_', "-")
    }

    /// Returns the Serde PascalCase name
    pub fn serde_name(&self) -> String {
        format!("{self:?}")
    }
}

/// Print out all categories with display names
/// Printable row for tabled
#[derive(Tabled)]
struct CategoryRow {
    #[tabled(rename = "Display Name")]
    display: String,
    #[tabled(rename = "CLI (--category)")]
    cli: String,
    #[tabled(rename = "Serde (PascalCase)")]
    serde: String,
}

/// Print out all categories using tabled
pub fn list_categories() {
    let rows: Vec<CategoryRow> = Category::iter()
        .map(|cat| CategoryRow {
            display: cat.display(),
            cli: cat.cli_name(),
            serde: cat.serde_name(),
        })
        .collect();

    let table = Table::new(rows).with(Style::modern()).to_string();
    println!("📦 Available Categories:\n{table}");
}
