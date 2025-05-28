// Module: command/vps
// Location: cli/src/command/vps/mod.rs
pub mod add;
pub mod connect;
pub mod delete;
pub mod list;
pub mod update;
pub mod script;

use clap::{Args, Subcommand};

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
    Add(add::VpsAddCommad),
    List(list::VpsListCommad),
    Connect(connect::VpsConnectCommand),
    Update(update::VpsUpdateCommand),
    Delete(delete::VpsDeleteCommand),
    Script(script::VpsScriptCommand),
}

pub fn handle_vps_command(cmd: VpsCommand, dry_run: bool) {
    if cmd.schema {
        crate::model::vps::vps_config_schema;
        return;
    }
    if cmd.delete {
        delete::confirm_and_delete_vps_config(dry_run);
        return;
    }
    match cmd.command {
        Some(VpsSubcommand::Add(add)) => {add::vps_command_add(add, dry_run);}
        Some(VpsSubcommand::List(list)) => {list::vps_command_list(list, dry_run);}
        Some(VpsSubcommand::Connect(connect)) => {connect::vps_command_connect(connect, dry_run);}
        Some(VpsSubcommand::Update(update)) => {update::vps_command_update(update, dry_run);}
        Some(VpsSubcommand::Delete (delete )) => {delete::vps_command_delete(delete, dry_run);}
        Some(VpsSubcommand::Script(script)) => {script::vps_command_script(script, dry_run);}
        None => { super::tranquility::print_subcommand_help("vps")},
    }
}
