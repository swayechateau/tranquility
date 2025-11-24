// Module: Command/Font/Refresh
// Location: cli/src/command/font/refresh.rs

use std::process::Command;

use crate::{log_error, log_info, print_error, print_success};

pub fn auto_refresh() {
    let status = Command::new("fc-cache")
        .args(["-f", "-v"])
        .status()
        .expect("Failed to run fc-cache");

    if status.success() {
        log_info!("refresh", "nerd-fonts", "success");
        print_success!("Font cache refreshed.");
    } else {
        log_error!("refresh", "nerd-fonts", "failed fc-cache");
        print_error!("Failed to refresh font cache.");
    }
}