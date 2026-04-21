use crate::{
    cli::CliResult,
    core::{context::Context, usecases::logs},
};
use clap::Args as ClapArgs;

#[derive(Debug, Clone, ClapArgs)]
#[command(about = "View application logs")]
pub struct Args {
    /// Only show JSON lines
    #[arg(long)]
    pub json_only: bool,

    /// Show only the log file path
    #[arg(long)]
    pub path: bool,

    /// Show last N log lines
    #[arg(long, default_value_t = 50)]
    pub tail: usize,

    /// Filter logs by date (YYYY-MM-DD)
    #[arg(long)]
    pub date: Option<String>,

    /// Filter logs by level (info, warn, error)
    #[arg(long, default_value = "info")]
    pub level: String,
}

pub async fn run(ctx: &Context, args: Args) -> CliResult<()> {
    logs::show(
        ctx,
        args.tail,
        args.level,
        args.json_only,
        args.date,
        args.path,
    )
    .await
    .map_err(|e| e.into())
}
