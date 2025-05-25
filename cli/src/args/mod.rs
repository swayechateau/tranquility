// src/args.rs
use clap::{error::ErrorKind, CommandFactory, Error, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use strum::Display;

use crate::{
    categories::{list_categories, Category}, command::{
        apps::{install::install_apps_command, uninstall::uninstall_apps_command},
        config::{
            config_command_override_application_file, config_command_override_config_file,
            config_command_override_vps_file, config_command_reset,
        },
        doctor, font, logs,
        vps::{confirm_and_delete_vps_config, connect_to_vps, json_schema_example, prompt_and_add_vps, vps_command_list},
    }, log_error, logger, model::application::list_supported_applications, print_info, system::SystemInfo
};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct TranquilityArgs {
    /// Turn debugging information on (repeatable)
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum, Display)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Check and fix common issues
    Doctor {
        /// Fix any issues automatically
        #[arg(long)]
        fix: bool,
    },
    /// Configuration management
    Config {
        /// Reset the config to default values
        #[arg(long)]
        reset: bool,
        #[arg(long)]
        override_config: Option<PathBuf>,
        #[arg(long)]
        override_applications: Option<PathBuf>,

        #[arg(long)]
        override_vps: Option<PathBuf>,
    },

    /// Show tranquility logs
    Logs {
        /// Only show JSON lines
        #[arg(long)]
        json_only: bool,

        /// Only show the log file path
        #[arg(long)]
        path: bool,

        /// Show last N log lines
        #[arg(long, default_value_t = 50)]
        tail: usize,

        /// Filter logs by date (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,

        /// Filter logs by level (error, warn, info)
        #[arg(long, default_value = "info")]
        level: String,
    },
    /// Application management
    Apps {
        #[command(subcommand)]
        action: Option<AppAction>,
    },

    /// Font management
    Fonts {
        #[command(subcommand)]
        action: Option<FontAction>,
    },
    /// VPS host management
    Vps {
        #[command(subcommand)]
        action: Option<VpsAction>,
        #[arg(long)]
        script: bool,
        #[arg(long)]
        schema: bool,
        #[arg(long)]
        list: bool,
        #[arg(long)]
        delete: bool,
    },
    /// List supported applications, fonts, or VPS hosts
    List {
        #[command(subcommand)]
        action: Option<ListAction>,
    },
}

#[derive(Subcommand, Debug)]
pub enum AppAction {
    /// Install default applications and from applications.json
    Install {
        #[arg(long)]
        all: bool,
        #[arg(long)]
        server: bool,
        #[arg(long = "dry-run")]
        dry_run: bool,
    },

    /// Uninstall default applications and from applications.json
    Uninstall {
        #[arg(long)]
        all: bool,
        #[arg(long)]
        server: bool,
        #[arg(long = "dry-run")]
        dry_run: bool,
    },

    /// List all categories
    Categories {},

    /// List supported applications
    List {
        #[arg(long)]
        server: bool,
        #[arg(long, value_enum)]
        category: Vec<Category>,
    },
}

#[derive(Subcommand, Debug)]
pub enum FontAction {
    /// Install Nerd Fonts
    Install {
        /// Install all fonts
        #[arg(long)]
        all: bool,

        /// Font name(s) (comma-separated or repeated)
        #[arg(long, value_name = "NAME")]
        name: Vec<String>,
    },

    /// Uninstall Nerd Fonts
    Uninstall {
        /// Uninstall all fonts
        #[arg(long)]
        all: bool,
        /// Font name(s) (comma-separated or repeated)
        #[arg(long, value_name = "NAME")]
        name: Vec<String>,
    },
    /// 🔁 Update all installed fonts
    Update {},
    /// List installed fonts
    List {
        /// Show only installed fonts
        #[arg(long)]
        installed: bool,
        /// Show available and installed fonts
        #[arg(long)]
        all: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum ListAction {
    Apps {
        #[arg(long)]
        server: bool,
        #[arg(long)]
        category: Vec<Category>,
    },
    Fonts {
        /// Show only installed fonts
        #[arg(long)]
        installed: bool,
        /// Show available and installed fonts
        #[arg(long)]
        all: bool,
    },
    Vps {},
    Categories {},
}

#[derive(Subcommand, Debug)]
pub enum VpsAction {
    Add {
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        host: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        port: Option<String>,
        #[arg(long = "private-key")]
        private_key: Option<String>,
        #[arg(long = "dry-run")]
        dry_run: bool,
    },
}

pub fn handle_args(args: TranquilityArgs) {
    if args.command.is_none() {
        println!("{}\n", SystemInfo::new().to_pretty_string());
        return;
    }

    if args.debug {
        logger::set_debug(true);
    }

    match args.command {
        Some(Commands::Config {
            reset,
            override_config,
            override_applications,
            override_vps,
        }) => {
            if reset {
                config_command_reset();
            }
            if let Some(path) = override_config {
                config_command_override_config_file(path.as_path());
            }
            if let Some(path) = override_applications {
                config_command_override_application_file(path.as_path());
            }
            if let Some(path) = override_vps {
                config_command_override_vps_file(path.as_path());
            }
        }

        Some(Commands::Logs {
            tail,
            level,
            json_only,
            date,
            path,
        }) => {
            logs::show_logs(
                tail,
                &level.to_lowercase(),
                json_only,
                date,
                path,
            );
        }

        Some(Commands::Fonts { action }) => match action {
            Some(FontAction::Install { all, name }) => font::install(all, name),
            Some(FontAction::Uninstall { all, name }) => font::uninstall(all, name),
            Some(FontAction::Update {}) => font::update(),
            Some(FontAction::List { installed, all }) => font::list(installed, all),
            None => font::list(false, false),
        },

        Some(Commands::Doctor { fix }) => doctor::run_doctor(fix),

        Some(Commands::Apps { action: Some(app_action) }) => match app_action {
            AppAction::Install { all, server, dry_run } => {
                notify_dry_run(dry_run);
                install_apps_command(all, server, dry_run);
            }
            AppAction::Uninstall { all, server, dry_run } => {
                notify_dry_run(dry_run);
                uninstall_apps_command(all, server, dry_run);
            }
            AppAction::Categories {} => list_categories(),
            AppAction::List { server, category } => {
                list_supported_applications(server, category);
            }
        },
        Some(Commands::Apps { action: None }) => print_subcommand_help("apps"),

        Some(Commands::Vps {
            list,
            script,
            schema,
            delete,
            action,
        }) => {
            if schema {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&json_schema_example()).unwrap()
                );
                return;
            }

            if delete {
                if let Err(e) = confirm_and_delete_vps_config() {
                    log_error!("delete", "vps", &format!("❌ Failed to delete VPS config: {e}"));
                }
                return;
            }

            match action {
                Some(VpsAction::Add {
                    name,
                    host,
                    username,
                    port,
                    private_key,
                    dry_run,
                }) => {
                    if let Err(e) = prompt_and_add_vps(name, host, username, port, private_key, dry_run) {
                        log_error!("add","vps", &format!("❌ Failed to add VPS entry: {e}"));
                    }
                }
                None => {
                    if let Err(e) = connect_to_vps(list, script) {
                        log_error!("connect","vps", &format!("❌ VPS connection failed: {e}"));
                    }
                }
            }
        }

        Some(Commands::List { action: Some(list_action) }) => match list_action {
            ListAction::Apps { server, category } => {
                list_supported_applications(server, category);
            }
            ListAction::Fonts { installed, all } => {
                font::list(installed, all);
            }
            ListAction::Vps {} => {
                let _ = vps_command_list();
            }
            ListAction::Categories {} => list_categories(),
        },
        Some(Commands::List { action: None }) => print_subcommand_help("list"),
        None => {}
    }
}

pub fn handle_arg_errors(err: Error) {
    match err.kind() {
        ErrorKind::UnknownArgument | ErrorKind::InvalidSubcommand => {
            TranquilityArgs::command().print_help().unwrap();
            println!();
        }
        _ => {
            err.print().expect("Failed to print error");
        }
    }
    std::process::exit(1);
}

fn notify_dry_run(dry_run: bool) {
    if dry_run {
        print_info!("💡 Running in dry-run mode. No changes will be made.");
    }
}

fn print_subcommand_help(subcommand: &str) {
    let mut cmd = TranquilityArgs::command();
    if let Some(sub) = cmd.find_subcommand_mut(subcommand) {
        sub.print_help().unwrap();
        println!();
    } else {
        eprintln!("❌ Subcommand `{}` not found.", subcommand);
    }
}
