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