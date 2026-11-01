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
    #[arg(long, value_parser = parse_date)]
    pub date: Option<String>,

    /// Filter logs by level (info, warn, error)
    #[arg(long, default_value = "info", value_parser = ["info", "warn", "error"])]
    pub level: String,
}

fn parse_date(value: &str) -> Result<String, String> {
    chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map(|d| d.format("%Y-%m-%d").to_string())
        .map_err(|_| format!("`{value}` is not a valid date, expected YYYY-MM-DD"))
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
}
