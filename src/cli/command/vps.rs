use clap::{Args, Subcommand};
use crate::{
    log_error,
    core::tquil::vps,
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

#[derive(Args, Debug)]
pub struct VpsAddCommand {
    #[arg(long)]
    id: Option<String>,
    #[arg(long)]
    name: Option<String>,
    #[arg(long)]
    host: Option<String>,
    #[arg(long)]
    user: Option<String>,
    #[arg(long)]
    port: Option<String>,
    #[arg(long = "private-key")]
    private_key: Option<String>,
}

#[derive(Args, Debug)]
pub struct VpsConnectCommand {
    #[arg(long)]
    id: Option<String>,
    #[arg(long, num_args = 0..=1, default_missing_value = "")]
    copy_id: Option<String>,
}

#[derive(Args, Debug)]
pub struct VpsListCommand {
    #[arg(long)]
    user: Option<String>,
    #[arg(long)]
    host: Option<String>,
    #[arg(long)]
    dry_run: bool,
}

#[derive(Args, Debug)]
pub struct VpsDeleteCommand {
    #[arg(long)]
    id: Option<String>,
}

#[derive(Args, Debug)]
pub struct VpsScriptCommand {
    #[arg(long)]
    id: Option<String>,
    #[arg(long)]
    inline: Option<String>,
    #[arg(long)]
    file: Option<String>,
}

#[derive(Args, Debug)]
pub struct VpsUpdateCommand {
    #[arg(long)]
    id: Option<String>,
    #[arg(long)]
    name: Option<String>,
    #[arg(long)]
    host: Option<String>,
    #[arg(long)]
    user: Option<String>,
    #[arg(long)]
    port: Option<String>,
    #[arg(long = "private-key")]
    private_key: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum VpsSubcommand {
    Add(VpsAddCommand),
    Connect(VpsConnectCommand),
    List(VpsListCommand),
    Update(VpsUpdateCommand),
    Delete(VpsDeleteCommand),
    Script(VpsScriptCommand),
}

pub fn run(cmd: VpsCommand, dry_run: bool) -> anyhow::Result<()> {
    if cmd.schema {
        crate::core::model::vps::vps_config_schema();
        return Ok(());
    }

    if cmd.delete {
        if let Err(e) = vps::delete::confirm_and_delete_vps_config(dry_run) {
            log_error!("delete", "vps", &format!("❌ Failed to delete config: {e}"));
        }
        return Ok(());
    }

    match cmd.command {
        Some(VpsSubcommand::Add(add)) => vps_command_add_run(add, dry_run),
        Some(VpsSubcommand::List(list)) => vps_command_list_run(list, dry_run),
        Some(VpsSubcommand::Connect(connect)) => vps_command_connect_run(connect, dry_run),
        Some(VpsSubcommand::Update(update)) => vps_command_update_run(update, dry_run),
        Some(VpsSubcommand::Delete(delete)) => vps_command_delete_run(delete, dry_run),
        Some(VpsSubcommand::Script(script)) => vps_command_script_run(script, dry_run),
        None => {
            println!("Please specify a subcommand for 'vps':");
            println!("  add       Add a new VPS entry");
            println!("  connect   Connect to a VPS");
            println!("  list      List configured VPS entries");
            println!("  update    Update an existing VPS entry");
            println!("  delete    Delete a VPS entry");
            println!("  script    Run a script on a VPS");
            Ok(())
        }
    }
}

fn vps_command_add_run(add: VpsAddCommand, dry_run: bool) -> anyhow::Result<()> {
    vps::add::run(
        add.id,
        add.name,
        add.host,
        add.user,
        add.port,
        add.private_key,
        dry_run,
    )
}

pub fn vps_command_list_run(list: VpsListCommand, dry_run: bool) -> anyhow::Result<()> {
    vps::list::run(list.user, list.host, dry_run)
}

fn vps_command_connect_run(connect: VpsConnectCommand, dry_run: bool) -> anyhow::Result<()> {
    vps::connect::run(connect.id, connect.copy_id, dry_run)
}

fn vps_command_update_run(update: VpsUpdateCommand, dry_run: bool) -> anyhow::Result<()> {
    vps::update::run(update.id, update.name, update.host, update.user, update.port, update.private_key, dry_run)
}

fn vps_command_delete_run(delete: VpsDeleteCommand, dry_run: bool) -> anyhow::Result<()> {
    vps::delete::run(delete.id, dry_run)
}

fn vps_command_script_run(script: VpsScriptCommand, dry_run: bool) -> anyhow::Result<()> {
    vps::script::run(script.id, script.inline, script.file, dry_run)
}
