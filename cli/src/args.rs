// src/args.rs
use clap::{error::ErrorKind, CommandFactory, Error, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use strum::Display;

use crate::{
    categories::{list_categories, Category},
    command::{
        apps::{install::install_apps_command, uninstall::uninstall_apps_command},
        config, doctor, font, logs,
        vps::{
            confirm_and_delete_vps_config, connect_to_vps, json_schema_example, prompt_and_add_vps,
        },
    },
    config::TranquilityConfig,
    model::application::list_supported_applications,
    print_error, print_info,
    system::SystemInfo,
};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct TranquilityArgs {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Turn debugging information on (repeatable)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

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
        /// Reset the config to default values
        #[arg(long)]
        reset: bool,
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
        schema: bool,
        #[arg(long)]
        list: bool,
        #[arg(long)]
        delete: bool,
    },

    Config {
        #[arg(long)]
        override_config: Option<PathBuf>,
        #[arg(long)]
        override_applications: Option<PathBuf>,

        #[arg(long)]
        override_vps: Option<PathBuf>,
    },

    /// Show tranquility logs
    Logs {
        #[arg(long, default_value = "50")]
        tail: usize,

        #[arg(long, value_enum, default_value = "info")]
        level: LogLevel,

        #[arg(long)]
        json: bool,

        #[arg(long)]
        date: Option<String>,
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
    },
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
    },
}

pub fn handle_args(args: TranquilityArgs) {
    let sys = SystemInfo::new();
    if args.command.is_none() {
        println!("{}", sys.to_pretty_string());
        println!();
        return;
    }

    match args.command {
        Some(Commands::Config {
            override_config,
            override_applications,
            override_vps,
        }) => {
            let config_path = TranquilityConfig::config_dir()
                .unwrap_or_default()
                .join("config.json");

            if let Some(path) = override_config {
                config::override_config_file(path.as_path(), &config_path);
            }

            // Reload config in case it was overridden
            let tran_config = TranquilityConfig::load_or_init();
            let conf = match tran_config {
                Ok(c) => c,
                Err(e) => {
                    print_error!("❌ Failed to load config: {e}");
                    return;
                }
            };

            if let Some(path) = override_applications {
                config::override_application_config_file(path.as_path(), &conf);
            }

            if let Some(path) = override_vps {
                config::override_vps_config_file(path.as_path(), &conf);
            }
        }

        Some(Commands::Logs {
            tail,
            level,
            json,
            date,
        }) => {
            logs::show_logs(tail, &level.to_string().to_lowercase(), json, date);
        }

        Some(Commands::Fonts { action }) => match action {
            Some(FontAction::Install { all, name }) => {
                font::install(all, name);
            }
            Some(FontAction::Uninstall { all, name }) => {
                font::uninstall(all, name);
            }
            Some(FontAction::List { installed }) => {
                font::list(installed);
            }
            Some(FontAction::Update {}) => {
                font::update();
            }
            None => font::list(false),
        },
        Some(Commands::Doctor { reset, fix }) => {
            doctor::run_doctor(reset, fix);
        }

        Some(Commands::Apps { action }) => match action {
            Some(AppAction::Install {
                all,
                server,
                dry_run,
            }) => {
                if dry_run {
                    print_info!("💡 Running in dry-run mode. No changes will be made.");
                }
                install_apps_command(all, server, dry_run);
            }
            Some(AppAction::Uninstall {
                all,
                server,
                dry_run,
            }) => {
                if dry_run {
                    print_info!("💡 Running in dry-run mode. No changes will be made.");
                }
                uninstall_apps_command(all, server, dry_run);
            }
            Some(AppAction::Categories {}) => list_categories(),
            Some(AppAction::List { server, category }) => {
                list_supported_applications(server, category)
            }
            None => print_info!("Use a subcommand like install or list for 'apps'"),
        },

        Some(Commands::Vps {
            list,
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
                    print_error!("❌ Failed to delete VPS config: {e}");
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
                }) => {
                    if let Err(e) = prompt_and_add_vps(name, host, username, port, private_key) {
                        print_error!("❌ Failed to add VPS entry: {e}");
                    }
                }
                None => {
                    if let Err(e) = connect_to_vps(list) {
                        print_error!("❌ VPS connection failed: {e}");
                    }
                }
            }
        }

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
