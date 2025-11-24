use clap::Args;
use dialoguer::{Select, theme::ColorfulTheme};
use std::io;

use crate::{
    config::TranquilityConfig,
    core::shell::ShellCommand,
    log_error,
    models::vps::json::{VpsConfig, VpsEntry},
    print_info, print_warn,
};

#[derive(Args, Debug)]
pub struct VpsConnectCommand {
    #[arg(long)]
    id: Option<String>,
    #[arg(long, num_args = 0..=1, default_missing_value = "")]
    copy_id: Option<String>,
}

pub fn vps_command_connect(cmd: VpsConnectCommand, dry_run: bool) {
    if let Err(e) = connect_to_vps(cmd.id, cmd.copy_id, dry_run) {
        log_error!("connect", "vps", &format!("❌ Failed to connect: {e}"));
    }
}

fn connect_to_vps(id: Option<String>, copy_id: Option<String>, dry_run: bool) -> io::Result<()> {
    let config = TranquilityConfig::load_once();
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
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Prompt failed: {e}")))?;

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
    let remote = format!("{user}@{host}");

    /* ---------- build the ssh args ---------- */
    let mut ssh_args = vec!["-tt"];
    if let Some(ref key) = vps.private_key {
        ssh_args.push("-i");
        ssh_args.push(key);
    }
    if port != "22" {
        ssh_args.push("-p");
        ssh_args.push(&port);
    }
    ssh_args.push(&remote);

    /* ---------- optional ssh-copy-id ---------- */
    if copy_id.is_some() {
        let pub_key_opt = resolve_pub_key(&copy_id, vps);

        let mut copy_args: Vec<&str> = Vec::new();
        if port != "22" {
            copy_args.push("-p");
            copy_args.push(&port);
        }
        if let Some(ref key_path) = pub_key_opt {
            copy_args.push("-i");
            copy_args.push(key_path);
        }
        copy_args.push(&remote);

        if dry_run {
            print_warn!("(dry run) Would run: ssh-copy-id {}", copy_args.join(" "));
        } else {
            ShellCommand::new("ssh-copy-id")
                .with_args(copy_args)
                .run_interactive(false)?;
        }
    }

    /* ---------- connect ---------- */
    if dry_run {
        print_warn!("(dry run) Would run: ssh {}", ssh_args.join(" "));
        return Ok(());
    }

    print_info!("🔌 Connecting to {remote} on port {port}...");
    ShellCommand::new("ssh")
        .with_args(ssh_args)
        .run_interactive(false)?;

    Ok(())
}

/// Decide which public-key file should be copied to the server.
///
/// Rules:
/// 1. If user gave `--copy-id <path>` *and it isn't empty*, use that.
/// 2. Else if the VPS has `private_key`, use `<private_key>.pub`.
/// 3. Otherwise return `None` → call ssh-copy-id with no -i flag.
fn resolve_pub_key(copy_id: &Option<String>, vps: &VpsEntry) -> Option<String> {
    match copy_id {
        Some(s) if !s.trim().is_empty() => Some(s.clone()),
        _ => vps.private_key.as_ref().map(|k| format!("{k}.pub")),
    }
}
