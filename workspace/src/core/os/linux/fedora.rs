// src/os/linux/fedora.rs
use crate::common::check_default_pm;

pub fn install() {
    // check if dnf is installed
    check_default_pm();
}