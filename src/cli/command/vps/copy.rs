// Module: Command/Vps/Connect
// Location: cli/src/command/vps/connect.rs

use clap::Args;
use dialoguer::{theme::ColorfulTheme, Select};
use std::io;
use tranquility::{log_error, print_info, print_warn};

use crate::{
    model::config::TranquilityConfig,
    model::vps::json::{VpsConfig},
    shell::ShellCommand,
};

#[derive(Args, Debug)]
pub struct VpsCopyCommand {
    #[arg(long)]
    id: Option<String>,
    #[arg(long)]
    destination: Option<String>,
    #[arg(long)]
    source: Option<String>,
    /// Copy files from remote server to remote server
    #[arg(long)]
    remote : bool,
    /// Copy files from the remote server to the local machine
    #[arg(long)]
    local : bool,
    
    
    
}

pub fn vps_command_copy(cmd: VpsCopyCommand, dry_run: bool) {
    if let Err(e) = connect_to_vps(cmd.id, dry_run) {
        log_error!("copy", "vps", &format!("❌ Failed to copy: {e}"));
    }
}

fn connect_to_vps(id: Option<String>, dry_run: bool) -> io::Result<()> {
    let config = TranquilityConfig::load_or_init()?;
    let mut vps_config = VpsConfig::load_from_file(&config.vps_file)?;
    vps_config.fix();

    if vps_config.vps.is_empty() {
        print_warn!("⚠️ No VPS entries found in your configuration.");
        return Ok(());
    }

    let vps = match id {
        Some(ref id) => vps_config.vps.iter().find(|v| v.id.as_deref() == Some(id)),
        None => {
            let options: Vec<String> = vps_config
                .vps
                .iter()
                .map(|v| {
                    let user = v.effective_user();
                    let name = v.name.clone().unwrap_or_else(|| "-".into());
                    format!("{}@{} ({})", user, v.host, name)
                })
                .collect();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("🔌 Select a VPS to connect to")
                .items(&options)
                .default(0)
                .interact()
                .map_err(|e| io::Error::other( format!("Prompt failed: {e}")))?;

            Some(&vps_config.vps[selection])
        }
    };

    let vps = match vps {
        Some(v) => v,
        None => {
            print_warn!("❌ No VPS found with the specified ID.");
            return Ok(());
        }
    };

    let port = vps.effective_port().to_string();
    let user = vps.effective_user();
    let host = &vps.host;

    let mut args = vec!["-tt"];

    if let Some(ref key) = vps.private_key {
        args.push("-i");
        args.push(key);
    }

    if port != "22" {
        args.push("-p");
        args.push(&port);
    }

    let remote = format!("{user}@{host}");
    args.push(&remote);

    if dry_run {
        print_warn!("(dry run) Would run: ssh {}", args.join(" "));
        return Ok(());
    }

    print_info!("🔌 Connecting to {remote} on port {port}...");
    ShellCommand::new("ssh")
        .with_args(args)
        .run_interactive(false)?;

    Ok(())
}
