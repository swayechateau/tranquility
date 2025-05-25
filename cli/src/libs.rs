//src/lib.rs
#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {
        $crate::print::print_with_prefix($crate::print::PrefixColor::RED, "error", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! print_success {
    ($($arg:tt)*) => {
        $crate::print::print_with_prefix($crate::print::PrefixColor::GREEN, "success", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! print_info {
    ($($arg:tt)*) => {
        $crate::print::print_with_prefix($crate::print::PrefixColor::BLUE, "info", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! print_warn {
    ($($arg:tt)*) => {
        $crate::print::print_with_prefix($crate::print::PrefixColor::YELLOW, "warn", format!($($arg)*))
    };
}

/// Log Macros
#[macro_export]
macro_rules! log {
    // No duration
    ($level:expr, $action:expr, $app:expr, $status:expr) => {{
        let source = if $level == "error" {
            Some(concat!(file!(), ":", line!()))
        } else {
            None
        };
        $crate::logger::log_to_full(
            $level,
            $action,
            $app,
            $status,
            None,
            source,
            $crate::logger::LogDestination::Primary($crate::logger::default_log_path()),
        )
    }};

    // With duration
    ($level:expr, $action:expr, $app:expr, $status:expr, $duration:expr) => {{
        let source = if $level == "error" {
            Some(concat!(file!(), ":", line!()))
        } else {
            None
        };
        $crate::logger::log_to_full(
            $level,
            $action,
            $app,
            $status,
            Some($duration),
            source,
            $crate::logger::LogDestination::File($crate::logger::default_log_path()),
        )
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
