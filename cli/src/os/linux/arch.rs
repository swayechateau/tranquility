// src/os/linux/arch.rs

use crate::common::check_default_pm;

pub fn install() {
    // check if pacman is installed
    check_default_pm();
}
