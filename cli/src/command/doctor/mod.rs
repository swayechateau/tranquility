// Module: Command/Doctor
// Location: cli/src/command/doctor/mod.rs
use std::{fs, path::{Path}};
use clap::{arg, Args};
use super::config::fix_config;
use crate::{
    config::TranquilityConfig, 
    model::application::get_apps, 
    package_manager::PackageManager, 
    system::SystemInfo
};
use tranquility::{log_error, print_success, print_warn};

#[derive(Args, Debug)]
pub struct DoctorCommand {
    /// Fix any issues automatically
    #[arg(long)]
    fix: bool,
}

pub fn doctor_command(cmd: DoctorCommand, dry_run:bool) {
    if cmd.fix {
        fix_config();
    }

    println!("\n🩺 Running Tranquility System Doctor...\n");

    // Config file check
    match TranquilityConfig::load_or_init() {
        Ok(cfg) => {
            print_success!("✅ Config file loaded");
            check_writable(&cfg.log_file(), "Log file path");
            check_file(&cfg.applications_file, "Applications file");
            check_file(&cfg.vps_file, "VPS config");
        }
        Err(e) => {log_error!("load","config",&format!("failed: {e}"));}
    }

    // System info
    let sys = SystemInfo::new();
    println!("{}", sys.to_pretty_string());

    // Package manager check
    println!("🔍 Checking common package managers:");
    for pm in PackageManager::supported_on_os(sys.os_type_raw()) {
        if pm.check_installed() {
            print_success!("✅ {} is installed", pm.name());
        } else {
            print_warn!("⚠️  {} not found in PATH", pm.name());
        }
    }

    // App schema check
    let app_count = get_apps().applications.len();
    if app_count == 0 {
        print_warn!("⚠️  No applications loaded from config.");
    } else {
        print_success!("✅ Loaded {} application(s) from config", app_count);
    }

    println!("\n🩺 Doctor check complete.\n");
}

fn check_file(path: &Path, label: &str) {
    if path.exists() {
        print_success!("✅ {} exists: {}", label, path.display());
    } else {
        print_warn!("⚠️  {} not found at {}", label, path.display());
    }
}

fn check_writable(path: &Path, label: &str) {
    let parent = path.parent().unwrap_or(path);
    if fs::create_dir_all(parent).is_ok() && fs::OpenOptions::new().create(true).append(true).open(path).is_ok() {
        print_success!("✅ {} is writable: {}", label, path.display());
    } else {
        log_error!("check-writable",label,&format!("not writable: {}", path.display()));
    }
}
