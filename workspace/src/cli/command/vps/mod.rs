// Module: Command/Vps
// Location: cli/src/command/vps/mod.rs

pub mod add;
pub mod connect;
pub mod delete;
pub mod list;
pub mod script;
pub mod update;

use clap::{Args, Subcommand};

use crate::{
    cli::print_subcommand_help, config::TranquilityConfig, log_error, log_info, models::vps,
};

#[derive(Args, Debug)]
pub struct VpsCommand {
    #[arg(long)]
    delete: bool,
    #[arg(long)]
    schema: bool,
    #[command(subcommand)]
    command: Option<VpsSubcommand>,
}

#[derive(Subcommand, Debug)]
pub enum VpsSubcommand {
    Add(add::VpsAddCommand),
    List(list::VpsListCommand),
    Connect(connect::VpsConnectCommand),
    Update(update::VpsUpdateCommand),
    Delete(delete::VpsDeleteCommand),
    Script(script::VpsScriptCommand),
}

pub fn handle_vps_command(cmd: VpsCommand, dry_run: bool) {
    if cmd.schema {
        vps::vps_config_schema(); // ← now actually calls the function
        return;
    }

    if cmd.delete {
        if let Err(e) = delete::confirm_and_delete_vps_config(dry_run) {
            log_error!("delete", "vps", &format!("❌ Failed to delete config: {e}"));
        }
        return;
    }

    match cmd.command {
        Some(VpsSubcommand::Add(add)) => add::vps_command_add(add, dry_run),
        Some(VpsSubcommand::List(list)) => list::vps_command_list(list, dry_run),
        Some(VpsSubcommand::Connect(connect)) => connect::vps_command_connect(connect, dry_run),
        Some(VpsSubcommand::Update(update)) => update::vps_command_update(update, dry_run),
        Some(VpsSubcommand::Delete(delete)) => delete::vps_command_delete(delete, dry_run),
        Some(VpsSubcommand::Script(script)) => script::vps_command_script(script, dry_run),
        None => print_subcommand_help("vps"),
    }
}

/// Fixes and updates the VPS config file in place, if needed
pub fn fix_vps() -> std::io::Result<()> {
    let config = TranquilityConfig::load_once();
    let path = &config.vps_file;

    let mut vps_config = vps::json::VpsConfig::load_from_file(path)?;
    match vps_config.fix_and_save(path) {
        Ok(()) => log_info!("reset", "vps-config-file", "vps config file updated"),
        Err(e) => {
            log_error!(
                "reset",
                "vps-config-file",
                &format!("❌ Failed to update VPS config file: {e}")
            );
            return Err(e);
        }
    }

    Ok(())
}
