use std::path::PathBuf;

use crate::core::{config::host::{backup_file, count_domains, load_host_config, read_to_string, strip_managed_block, upsert_managed_block, write_string}, utils::default_hosts_path};
use anyhow::{anyhow, Context, Result};

pub fn run(hosts: Option<PathBuf>, ipv4: String, ipv6: bool,  remove: bool, dry_run: bool) -> Result<()> {
    let hosts_path = hosts.unwrap_or_else(default_hosts_path);
    if !hosts_path.exists() {
        return Err(anyhow!(
            "Hosts file not found at {}. Use --hosts to point to it.",
            hosts_path.display()
        ));
    }

    let cfg = load_host_config()?;
    let wanted_lines = cfg.build_lines(&ipv4, ipv6)?;

    let current = read_to_string(&hosts_path)
        .with_context(|| format!("reading hosts file: {}", hosts_path.display()))?;

    let new_content = if remove {
        strip_managed_block(&current)
    } else {
        upsert_managed_block(&current, &wanted_lines)
    };

    if dry_run {
        println!("{}", new_content);
        return Ok(());
    }

    // Backup before writing
    backup_file(&hosts_path, &current)?;
    write_string(&hosts_path, &new_content)
        .with_context(|| format!("writing hosts file: {}", hosts_path.display()))?;

    if remove {
        eprintln!(
            "Removed managed section from {}",
            hosts_path.display()
        );
    } else {
        eprintln!(
            "Updated managed section in {} with {} domain(s)",
            hosts_path.display(),
            count_domains(&wanted_lines)
        );
    }
    Ok(())
}
