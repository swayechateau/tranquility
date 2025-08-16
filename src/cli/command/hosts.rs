use std::path::PathBuf;

use clap::ArgAction;

use crate::core::tquil;

#[derive(Debug, clap::Args)]
pub struct HostCommand {
    /// Path to the input TOML file
    #[arg(short = 'i', long = "input")]
    input: Option<PathBuf>,
    /// Path to hosts file (auto-detected if omitted)
    #[arg(short = 'f', long = "hosts")]
    hosts: Option<PathBuf>,
    /// Override IPv4 to map domains to (default: 127.0.0.1)
    #[arg(long = "ipv4", default_value = "127.0.0.1")]
    ipv4: String,
    /// Also write IPv6 (::1) entries
    #[arg(long = "ipv6", action = ArgAction::SetTrue)]
    ipv6: bool,
    /// Dry run (print the would-be result without writing)
    #[arg(long = "dry-run", action = ArgAction::SetTrue)]
    dry_run: bool,
    /// Remove the managed section instead of updating it
    #[arg(long = "remove", action = ArgAction::SetTrue)]
    remove: bool,
}

pub fn run(args: HostCommand, dry_run: bool) -> anyhow::Result<()> {
    tquil::hosts::run(
        args.hosts,
        args.ipv4,
        args.ipv6,
        args.remove,
        dry_run,
    )
}