// src/os/linux/debian.rs
use crate::common::check_default_pm;

pub fn install() {
    // check if apt is installed
    check_default_pm();
}
