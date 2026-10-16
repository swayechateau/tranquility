use std::fmt;

use serde::{Deserialize, Serialize};

use crate::engine::error::ErrorKind;

pub mod exit_status {
    // User input errors
    pub const USER: i32 = 2;
    // Configuration errors
    pub const CONFIG: i32 = 10;
    // Validation errors
    pub const VALIDATION: i32 = 30;
    // I/O errors
    pub const IO: i32 = 25;
    // Process/shell execution errors
    pub const PROCESS: i32 = 26;
    // VPS operation errors
    pub const VPS: i32 = 40;
    // Package manager errors
    pub const PACKAGE_MANAGER: i32 = 41;
    // Font management errors
    pub const FONT_MANAGEMENT: i32 = 42;
    // Application management errors
    pub const APPLICATION_MANAGEMENT: i32 = 43;
    // Runtime errors
    pub const RUNTIME: i32 = 50;
}

macro_rules! error_codes {
    (
        $(
            $variant:ident => ($kind:ident, $num:expr, $msg:expr)
        ),* $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum ErrorCode {
            $($variant),*
        }

        impl ErrorCode {
            pub const fn kind(self) -> ErrorKind {
                match self {
                    $(Self::$variant => ErrorKind::$kind),*
                }
            }

            pub const fn message(self) -> &'static str {
                match self {
                    $(Self::$variant => $msg),*
                }
            }

            pub const fn index(self) -> u16 {
                match self {
                    $(Self::$variant => $num),*
                }
            }

            pub const fn numeric(self) -> u16 {
                self.kind().prefix() * 1000 + self.index()
            }

            pub fn id(self) -> String {
                format!("E{:04}", self.numeric())
            }

            pub const fn exit_code(self) -> i32 {
                match self.kind() {
                    // User input errors
                    ErrorKind::User => exit_status::USER,
                    // Configuration errors
                    ErrorKind::Config => exit_status::CONFIG,
                    // Validation errors
                    ErrorKind::Validation => exit_status::VALIDATION,
                    // I/O errors
                    ErrorKind::Io => exit_status::IO,
                    // Process/shell execution errors
                    ErrorKind::Process => exit_status::PROCESS,
                    // VPS operation errors
                    ErrorKind::Vps => exit_status::VPS,
                    // Package manager errors
                    ErrorKind::PackageManager => exit_status::PACKAGE_MANAGER,
                    // Font management errors
                    ErrorKind::FontManagement => exit_status::FONT_MANAGEMENT,
                    // Application management errors
                    ErrorKind::ApplicationManagement => exit_status::APPLICATION_MANAGEMENT,
                    // Runtime errors
                    ErrorKind::Runtime => exit_status::RUNTIME,
                }
            }
        }

        impl fmt::Display for ErrorCode {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{} ({})", self.message(), self.id())
            }
        }
    };
}

error_codes! {
    // User/CLI errors
    UserCancelled => (User, 1, "Operation cancelled by user"),
    UserInteractionRequired => (User, 2, "Interactive input required"),
    UserInvalidArgument => (User, 3, "Invalid argument provided"),
    FormatUnsupported => (User, 4, "Format is not supported"),
    InvalidInput => (User, 5, "Input is invalid"),

    // Configuration errors
    ConfigInvalid => (Config, 1, "Configuration is invalid"),
    ConfigUnreadable => (Config, 2, "Configuration is unreadable"),
    ConfigNotFound => (Config, 3, "Configuration file not found"),
    ConfigMissingField => (Config, 4, "Configuration is missing required field"),
    ConfigPathInvalid => (Config, 5, "Configuration path is invalid"),

    // Validation errors
    ValidationFailed => (Validation, 1, "Validation failed"),
    SchemaMismatch => (Validation, 2, "Configuration does not match schema"),
    RequiredFieldMissing => (Validation, 3, "Required field is missing"),
    InvalidDataType => (Validation, 4, "Invalid data type"),

    // I/O errors
    IoFailure => (Io, 1, "I/O operation failed"),
    SerializationFailure => (Io, 2, "Serialization or deserialization failed"),
    UiPromptFailed => (Io, 3, "User prompt failed"),
    OutputRenderFailure => (Io, 4, "Output rendering failed"),
    FileNotFound => (Io, 5, "File not found"),
    PermissionDenied => (Io, 6, "Permission denied"),

    // Process/shell execution errors
    ProcessFailure => (Process, 1, "Process execution failed"),
    ShellCommandFailed => (Process, 2, "Shell command failed"),
    CommandNotFound => (Process, 3, "Command not found"),
    CommandTimeout => (Process, 4, "Command timed out"),
    ProcessNonZeroExit => (Process, 5, "Process exited with non-zero status"),

    // VPS operation errors
    VpsOperationFailed => (Vps, 1, "VPS operation failed"),
    VpsNotFound => (Vps, 2, "VPS host not found"),
    VpsConnectionFailed => (Vps, 3, "Failed to connect to VPS"),
    VpsSshKeyMissing => (Vps, 4, "SSH private key not found"),
    VpsConfigInvalid => (Vps, 5, "VPS configuration is invalid"),
    VpsAlreadyExists => (Vps, 6, "VPS host already exists"),

    // Package manager errors
    PackageManagerNotFound => (PackageManager, 1, "Package manager not found"),
    PackageNotFound => (PackageManager, 2, "Package not found"),
    PackageInstallFailed => (PackageManager, 3, "Package installation failed"),
    PackageUninstallFailed => (PackageManager, 4, "Package uninstallation failed"),
    PackageUpdateFailed => (PackageManager, 5, "Package update failed"),
    PackageManagerError => (PackageManager, 6, "Package manager encountered an error"),

    // Font management errors
    FontNotFound => (FontManagement, 1, "Font not found"),
    FontInstallFailed => (FontManagement, 2, "Font installation failed"),
    FontUninstallFailed => (FontManagement, 3, "Font uninstallation failed"),
    FontListingFailed => (FontManagement, 4, "Failed to list fonts"),
    FontAlreadyInstalled => (FontManagement, 5, "Font is already installed"),

    // Application management errors
    ApplicationNotFound => (ApplicationManagement, 1, "Application not found"),
    ApplicationInstallFailed => (ApplicationManagement, 2, "Application installation failed"),
    ApplicationUninstallFailed => (ApplicationManagement, 3, "Application uninstallation failed"),
    ApplicationVersionConflict => (ApplicationManagement, 4, "Application version conflict"),
    ApplicationDependencyMissing => (ApplicationManagement, 5, "Application dependency is missing"),
    ApplicationAlreadyInstalled => (ApplicationManagement, 6, "Application is already installed"),

    // Runtime errors
    RuntimeFailure => (Runtime, 1, "Runtime error occurred"),
    InvalidState => (Runtime, 2, "Invalid runtime state"),
    OperationNotSupported => (Runtime, 3, "Operation not supported on this system"),
    OperationNotSupportedOnPlatform => (Runtime, 4, "Operation not supported on this platform"),
}
