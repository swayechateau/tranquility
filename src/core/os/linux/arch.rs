// src/os/linux/arch.rs

use crate::common::{check_command, check_default_pm, run_shell_command};

pub fn install() {
    // check if pacman is installed
    check_default_pm();
}


