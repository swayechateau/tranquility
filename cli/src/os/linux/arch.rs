// src/os/linux/arch.rs

use crate::common::{check_command, check_default_pm, run_shell_command};

pub fn install() {
    // check if pacman is installed
    check_default_pm();
    install_aur_helper();
}

fn install_aur_helper() {
    // check if yay is installed
    if !check_command("yay", "Yay", false) {
        println!("Installing yay...");
        run_shell_command("git clone https://aur.archlinux.org/yay.git");
        run_shell_command("cd yay && makepkg -si --noconfirm");
        run_shell_command("cd .. && rm -rf yay");
    }
}
