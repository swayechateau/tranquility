use crate::{
    cli::{CliResult, args::print_subcommand_help},
    core::{context::Context, usecases::vps},
};
use clap::{Args as ClapArgs, Subcommand};

#[derive(Debug, Clone, ClapArgs)]
#[command(about = "Manage VPS profiles")]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Option<VpsSubcommand>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum VpsSubcommand {
    /// Add a new VPS profile
    Add {
        /// Friendly profile name, e.g. "Temp Server" (default: the host)
        name: Option<String>,
        /// Unique one-word id (default: generated from the name, e.g. tempserver)
        #[arg(long)]
        id: Option<String>,
        /// Hostname or IP
        #[arg(long)]
        host: String,
        /// SSH user (default: the current user)
        #[arg(long)]
        user: Option<String>,
        /// SSH port (default: 22)
        #[arg(long)]
        port: Option<u16>,
        /// Path to the SSH private key
        #[arg(long)]
        private_key: Option<std::path::PathBuf>,
        /// Script to run on the server after `vps connect` logs in
        #[arg(long)]
        post_connect_script: Option<String>,
        /// ProxyJump spec (user@host[:port]) to reach this server through
        #[arg(long)]
        jump_host: Option<String>,
    },
    /// List VPS profiles
    List,
    /// Connect to a VPS profile
    Connect {
        /// VPS ID or name (omit for interactive selection)
        #[arg(long)]
        id: Option<String>,
        /// Seconds to wait for the connection before giving up
        #[arg(long)]
        timeout: Option<u32>,
    },
    /// Update a VPS profile
    Update {
        /// VPS ID or name (omit for interactive selection)
        #[arg(long)]
        id: Option<String>,
        /// New unique one-word id
        #[arg(long, alias = "rename-to")]
        new_id: Option<String>,
        /// New friendly name
        #[arg(long)]
        name: Option<String>,
        /// New hostname or IP
        #[arg(long)]
        host: Option<String>,
        /// New SSH user
        #[arg(long)]
        user: Option<String>,
        /// New SSH port
        #[arg(long)]
        port: Option<u16>,
        /// New private key path
        #[arg(long)]
        private_key: Option<String>,
        /// New post-connect script (empty string removes it)
        #[arg(long)]
        post_connect_script: Option<String>,
        /// New ProxyJump spec (empty string removes it)
        #[arg(long)]
        jump_host: Option<String>,
    },
    /// Delete a VPS profile
    Delete {
        /// VPS ID or name (omit for interactive selection)
        #[arg(long)]
        id: Option<String>,
    },
    /// Run a script on a VPS profile
    Script {
        /// VPS ID or name (omit for interactive selection; required with --all off)
        #[arg(long)]
        id: Option<String>,
        /// Inline script to run
        #[arg(long)]
        inline: Option<String>,
        /// Script file path
        #[arg(long)]
        file: Option<std::path::PathBuf>,
        /// Run on every VPS profile instead of one
        #[arg(long, conflicts_with = "id")]
        all: bool,
        /// Seconds to wait for the connection before giving up
        #[arg(long)]
        timeout: Option<u32>,
    },
    /// Copy files to/from a VPS profile
    Copy {
        /// VPS ID or name (the source profile when --to-id is given)
        #[arg(long)]
        id: Option<String>,
        /// Destination profile id/name for a direct profile-to-profile copy
        #[arg(long, conflicts_with = "all")]
        to_id: Option<String>,
        /// Source path (local or remote)
        #[arg(long)]
        source: String,
        /// Destination path (local or remote)
        #[arg(long)]
        destination: String,
        /// Direction: local→remote (default), remote→local, or remote→remote (same host)
        #[arg(long, value_parser = ["push", "pull", "remote"])]
        direction: Option<String>,
        /// Run on every VPS profile instead of one
        #[arg(long, conflicts_with_all = ["id", "to_id"])]
        all: bool,
        /// Seconds to wait for the connection before giving up
        #[arg(long)]
        timeout: Option<u32>,
    },
    /// Test SSH connectivity to a VPS profile without running anything
    Ping {
        /// VPS ID or name (omit for interactive selection)
        #[arg(long)]
        id: Option<String>,
        /// Test every VPS profile instead of one
        #[arg(long, conflicts_with = "id")]
        all: bool,
        /// Seconds to wait for the connection before giving up (default: 5)
        #[arg(long)]
        timeout: Option<u32>,
    },
    /// Export all VPS profiles (to stdout, or a file)
    Export {
        /// Write to this file instead of stdout
        #[arg(long)]
        output: Option<std::path::PathBuf>,
    },
    /// Import VPS profiles from a file
    Import {
        /// File to import profiles from
        #[arg(long)]
        file: std::path::PathBuf,
        /// Add to the existing profiles, skipping duplicate ids, instead of replacing them
        #[arg(long)]
        merge: bool,
    },
    /// Show VPS config JSON Schema, with an example
    Schema,
}

pub async fn run(ctx: &Context, args: Args) -> CliResult<()> {
    match args.subcommand {
        Some(VpsSubcommand::Add {
            name,
            id,
            host,
            user,
            port,
            private_key,
            post_connect_script,
            jump_host,
        }) => {
            vps::add(
                ctx,
                id,
                name,
                host,
                user,
                port,
                private_key,
                post_connect_script,
                jump_host,
            )
            .await
        }
        Some(VpsSubcommand::List) => vps::list(ctx).await,
        Some(VpsSubcommand::Connect { id, timeout }) => vps::connect(ctx, id, timeout).await,
        Some(VpsSubcommand::Update {
            id,
            new_id,
            name,
            host,
            user,
            port,
            private_key,
            post_connect_script,
            jump_host,
        }) => {
            vps::update(
                ctx,
                id,
                new_id,
                name,
                host,
                user,
                port,
                private_key,
                post_connect_script,
                jump_host,
            )
            .await
        }
        Some(VpsSubcommand::Delete { id }) => vps::delete(ctx, id).await,
        Some(VpsSubcommand::Script {
            id,
            inline,
            file,
            all,
            timeout,
        }) => vps::script(ctx, id, inline, file, all, timeout).await,
        Some(VpsSubcommand::Copy {
            id,
            to_id,
            source,
            destination,
            direction,
            all,
            timeout,
        }) => vps::copy(ctx, id, to_id, source, destination, direction, all, timeout).await,
        Some(VpsSubcommand::Ping { id, all, timeout }) => vps::ping(ctx, id, all, timeout).await,
        Some(VpsSubcommand::Export { output }) => vps::export(ctx, output).await,
        Some(VpsSubcommand::Import { file, merge }) => vps::import(ctx, file, merge).await,
        Some(VpsSubcommand::Schema) => vps::schema(ctx).await,
        None => {
            print_subcommand_help("vps");
            Ok(())
        }
    }
}
