// src/categories.rs
// Software categories
#[derive(Copy, Clone, Debug)]
pub enum Category {
    Fonts,
    PackageManagers,
    Shells,
    Browsers,
    Servers,
    Utilities(UtilitiesCategory),
    CLI(CliCategory),
    Customization(CustomizationCategory),
    Communication,
    Creative,
    Productivity(ProductivityCategory),
    Virtualization,
    Gaming,
    Development(DevelopmentCategory),
    Recording,
    Streaming,
}

#[derive(Copy, Clone, Debug)]
pub enum UtilitiesCategory {
    PasswordManager,
    RemoteDesktop,
    VPN,
    MediaDownloader,
    ImageBurner,
    WindowManager,
}

#[derive(Copy, Clone, Debug)]
pub enum CliCategory {
    Emulators,
    Development,
    Productivity,
    Networking,
    Essential,
    Miscellaneous,
}

#[derive(Copy, Clone, Debug)]
pub enum CustomizationCategory {
    Theming,
}

#[derive(Copy, Clone, Debug)]
pub enum ProductivityCategory {
    Office,
    OfficeAddons,
    NoteTaking,
    TaskManagement,
}

#[derive(Copy, Clone, Debug)]
pub enum DevelopmentCategory {
    Emulators,
    DatabasesManagement,
    ProgrammingLanguages,
    Editors,
    Containerization,
    Engines,
    AI,
    DevTools,
}