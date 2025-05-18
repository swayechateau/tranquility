// src/os/linux/ubuntu.rs
use crate::common::check_default_pm;

pub fn install() {
    // check if apt is installed
    check_default_pm();
}
