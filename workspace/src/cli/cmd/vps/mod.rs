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
        /// Profile name (default: the host)
        name: Option<String>,
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
    },
    /// List VPS profiles
    List,
    /// Connect to a VPS profile
    Connect {
        /// VPS ID or name (omit for interactive selection)
        #[arg(long)]
        id: Option<String>,
    },
    /// Update a VPS profile
    Update {
        /// VPS ID or name
        #[arg(long, required = true)]
        id: String,
        /// New name
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
    },
    /// Delete a VPS profile
    Delete {
        /// VPS ID or name (omit for interactive selection)
        #[arg(long)]
        id: Option<String>,
    },
    /// Run a script on a VPS profile
    Script {
        /// VPS ID or name (omit for interactive selection)
        #[arg(long)]
        id: Option<String>,
        /// Inline script to run
        #[arg(long)]
        inline: Option<String>,
        /// Script file path
        #[arg(long)]
        file: Option<std::path::PathBuf>,
    },
    /// Copy files to/from a VPS profile
    Copy {
        /// VPS ID or name
        #[arg(long)]
        id: Option<String>,
        /// Source path (local or remote)
        #[arg(long)]
        source: String,
        /// Destination path (local or remote)
        #[arg(long)]
        destination: String,
        /// Direction: local→remote (default), remote→local, or remote→remote
        #[arg(long, value_parser = ["push", "pull", "remote"])]
        direction: Option<String>,
    },
    /// Show VPS config schema example
    Schema,
}

pub async fn run(ctx: &Context, args: Args) -> CliResult<()> {
    match args.subcommand {
        Some(VpsSubcommand::Add {
            name,
            host,
            user,
            port,
            private_key,
        }) => vps::add(ctx, name, host, user, port, private_key).await,
        Some(VpsSubcommand::List) => vps::list(ctx).await,
        Some(VpsSubcommand::Connect { id }) => vps::connect(ctx, id).await,
        Some(VpsSubcommand::Update {
            id,
            name,
            host,
            user,
            port,
            private_key,
        }) => vps::update(ctx, id, name, host, user, port, private_key).await,
        Some(VpsSubcommand::Delete { id }) => vps::delete(ctx, id).await,
        Some(VpsSubcommand::Script { id, inline, file }) => {
            vps::script(ctx, id, inline, file).await
        }
        Some(VpsSubcommand::Copy {
            id,
            source,
            destination,
            direction,
        }) => vps::copy(ctx, id, source, destination, direction).await,
        Some(VpsSubcommand::Schema) => vps::schema(ctx).await,
        None => {
            print_subcommand_help("vps");
            Ok(())
        }
    }
}
