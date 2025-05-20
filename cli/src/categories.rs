// src/categories.rs
use clap::ValueEnum;
use serde::Deserialize;

/// Software categories
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum, Deserialize)]
pub enum Category {
    Fonts,
    PackageManagement,
    Shells,
    Browsers,
    Servers,
    TerminalEmmulators,
    PasswordManagement,
    Encyption,
    RemoteDesktop,
    VPN,
    DownloadMangement,
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
    DatabasesManagement,
    ProgrammingLanguages,
    Editors,
    Containerization,
    Engines,
    AI,
    DevTools,
}

/// Return all categories manually
pub fn all_categories() -> &'static [Category] {
    use Category::*;
    &[
        Fonts,
        PackageManagement,
        Shells,
        Browsers,
        Servers,
        TerminalEmmulators,
        PasswordManagement,
        Encyption,
        RemoteDesktop,
        VPN,
        DownloadMangement,
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
        DatabasesManagement,
        ProgrammingLanguages,
        Editors,
        Containerization,
        Engines,
        AI,
        DevTools,
    ]
}

/// Loop through and print out the categories
pub fn list_categories() {
    println!("📦 Available Categories:");
    for category in all_categories() {
        println!("- {}", category.to_possible_value().unwrap().get_name());
    }
}