pub mod cli;
pub mod config;
pub mod core;
pub mod models;

/// List of supported file extensions for schema validation.
pub const SUPPORTED_EXTS: [&str; 4] = ["yaml", "yml", "json", "xml"];

#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {
        $crate::core::print::print_with_prefix($crate::core::print::PrefixColor::RED, "error", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! print_success {
    ($($arg:tt)*) => {
        $crate::core::print::print_with_prefix($crate::core::print::PrefixColor::GREEN, "success", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! print_info {
    ($($arg:tt)*) => {
        $crate::core::print::print_with_prefix($crate::core::print::PrefixColor::BLUE, "info", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! print_warn {
    ($($arg:tt)*) => {
        $crate::core::print::print_with_prefix($crate::core::print::PrefixColor::YELLOW, "warn", format!($($arg)*))
    };
}

/// Log Macros
#[macro_export]
macro_rules! log {
    // no duration
    ($level:expr, $action:expr, $app:expr, $status:expr) => {{
        let loaded = $crate::config::CONFIG.get().is_some();
        let source = if $level == "error" {
            Some(concat!(file!(), ":", line!()))
        } else {
            None
        };

        if loaded {
            $crate::core::logger::log_event($level, $action, $app, $status, None, source);
        } else {
            eprintln!(
                "[{}] {} {} - {}",
                $level.to_uppercase(),
                $action,
                $app,
                $status
            );
        }
    }};

    // with duration
    ($level:expr, $action:expr, $app:expr, $status:expr, $duration:expr) => {{
        let loaded = $crate::model::config::CONFIG.get().is_some();
        let source = if $level == "error" {
            Some(concat!(file!(), ":", line!()))
        } else {
            None
        };

        if loaded {
            $crate::core::logger::log_event(
                $level,
                $action,
                $app,
                $status,
                Some($duration),
                source,
            );
        } else {
            eprintln!(
                "[{}] {} {} - {} ({:?})",
                $level.to_uppercase(),
                $action,
                $app,
                $status,
                $duration
            );
        }
    }};
}

#[macro_export]
macro_rules! log_info {
    ($action:expr, $app:expr, $status:expr) => {
        $crate::log!("info", $action, $app, $status)
    };
    ($action:expr, $app:expr, $status:expr, $duration:expr) => {
        $crate::log!("info", $action, $app, $status, $duration)
    };
}

#[macro_export]
macro_rules! log_warn {
    ($action:expr, $app:expr, $status:expr) => {
        $crate::log!("warn", $action, $app, $status)
    };
    ($action:expr, $app:expr, $status:expr, $duration:expr) => {
        $crate::log!("warn", $action, $app, $status, $duration)
    };
}

#[macro_export]
macro_rules! log_error {
    ($action:expr, $app:expr, $status:expr) => {
        $crate::log!("error", $action, $app, $status)
    };
    ($action:expr, $app:expr, $status:expr, $duration:expr) => {
        $crate::log!("error", $action, $app, $status, $duration)
    };
}
