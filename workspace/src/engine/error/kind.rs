use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ErrorKind {
    // User/CLI errors
    User,
    // Configuration errors
    Config,
    // Validation errors
    Validation,
    // I/O and execution errors
    Io,
    // Process/shell execution errors
    Process,
    // VPS operations
    Vps,
    // Package manager operations
    PackageManager,
    // Font management
    FontManagement,
    // Application management
    ApplicationManagement,
    // Runtime errors
    Runtime,
}

impl ErrorKind {
    pub const fn prefix(self) -> u16 {
        match self {
            Self::User => 0,
            Self::Config => 1,
            Self::Validation => 2,
            Self::Io => 3,
            Self::Process => 4,
            Self::Vps => 5,
            Self::PackageManager => 6,
            Self::FontManagement => 7,
            Self::ApplicationManagement => 8,
            Self::Runtime => 9,
        }
    }
}
