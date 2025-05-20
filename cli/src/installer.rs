// src/installer.rs

use crate::print::{print_with_prefix, PrefixColor};

pub fn install_apps(all: bool, server: bool) {
    // if server and not supported server print a warning unsupported server variable proceed at own risk
    print_warn!("unsupported server variant - please proceed at your own risk!");
    print_with_prefix(PrefixColor::NORMAL, "Procceeding", "installing on unsupported server...".to_string());
    // if server and all set - install all server app
    // if not server but all set - install all apps 
    // if not all or server, loop through categories and ask user which catergory do they wish to install apps from
    // add them to the list and proceed with the install
}

pub fn uninstall_apps(all:bool, server:bool) {
    // if server and not supported server print a warning unsupported server variable proceed at own risk
    // if server and all set - uninstall all server apps
    // if not server but all set - uninstall all apps 
}

