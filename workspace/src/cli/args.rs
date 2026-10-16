use std::path::PathBuf;

use crate::{cli::cmd, core::bootstrap::ContextArgs};
use clap::{ArgAction, Args, CommandFactory, Parser, Subcommand, ValueEnum};

/// Output format enumeration for results
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum FormatArg {
    Auto,
    Text,
    Markdown,
    Json,
    Jsonl,
    Plain,
}

impl FormatArg {
    pub fn to_output_format_string(&self, use_json: bool) -> String {
        match (use_json, self) {
            (_, FormatArg::Text) => "text".to_string(),
            (_, FormatArg::Markdown) => "markdown".to_string(),
            (_, FormatArg::Json) => "json".to_string(),
            (_, FormatArg::Jsonl) => "jsonl".to_string(),
            (_, FormatArg::Plain) => "plain".to_string(),
            (true, FormatArg::Auto) => "json".to_string(),
            (false, FormatArg::Auto) => "text".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ColorMode {
    Auto,
    Always,
    Never,
}

impl std::fmt::Display for ColorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorMode::Auto => write!(f, "auto"),
            ColorMode::Always => write!(f, "always"),
            ColorMode::Never => write!(f, "never"),
        }
    }
}

/// Tranquility
#[derive(Parser, Debug)]
#[command(
    name = "tquil",
    version,
    author,
    about = "Tranquility",
    propagate_version = true
)]
pub struct Cli {
    /// Global flags (apply to all subcommands)
    #[command(flatten)]
    pub global: GlobalArgs,
    /// Subcommands
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    /// Show system information and status
    Info,
    App(cmd::app::Args),
    Font(cmd::font::Args),
    Logs(cmd::logs::Args),
    List(cmd::list::Args),
    Config(cmd::config::Args),
    Doctor(cmd::doctor::Args),
    Vps(cmd::vps::Args),
}

#[derive(Debug, Clone, Args)]
pub struct GlobalArgs {
    /// Increase verbosity (-v, -vv, -vvv); combine with -q to reduce
    #[arg(short = 'v', long = "verbose", action = ArgAction::Count, global = true)]
    pub verbose: u8,
    /// Decrease verbosity (-q, -qq)
    #[arg(short = 'q', long = "quiet", action = ArgAction::Count, global = true)]
    pub quiet: u8,
    /// Output Envelop as JSON instead of human-readable text
    #[arg(long, global = true)]
    pub json: bool,
    /// Output Payload format (json, jsonl, markdown, text)
    #[arg(long, global = true, default_value_t = FormatArg::Auto, value_enum, conflicts_with = "plain")]
    pub format: FormatArg,
    /// Output plain text (equivalent to --format plain)
    #[arg(long, short = 'p', global = true, conflicts_with = "format")]
    pub plain: bool,
    /// Simulate actions without changes
    #[arg(long, global = true)]
    pub dry_run: bool,
    /// Color policy for output
    #[arg(long, value_enum, default_value_t = ColorMode::Auto, global = true)]
    pub color: ColorMode,
    /// Run against another directory (default: current working directory)
    #[arg(short = 'C', long = "cwd", global = true, default_value = ".")]
    pub cwd: PathBuf,
    /// Strict, non-interactive mode (assume yes, no prompts, CI-friendly)
    #[arg(long, global = true, conflicts_with = "non_interactive")]
    pub ci: bool,
    /// Strict, non-interactive mode (assume no, no prompts, CI-friendly)
    #[arg(long, global = true, conflicts_with = "ci")]
    pub non_interactive: bool,
    /// Accept defaults automatically (assume yes, no prompts)
    #[arg(long, short = 'y', global = true)]
    pub yes: bool,
    /// Force actions that would normally be prevented (e.g. pushing to protected branches)
    #[arg(long, global = true)]
    pub force: bool,
    /// Use a specific cwizard config not in the default location
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,
}

impl From<GlobalArgs> for ContextArgs {
    fn from(args: GlobalArgs) -> Self {
        Self {
            verbose: args.verbose,
            quiet: args.quiet,
            json: args.json,
            format: if args.plain {
                "plain".to_string()
            } else {
                args.format.to_output_format_string(args.json)
            },
            dry_run: args.dry_run,
            output_color: args.color.to_string(),
            cwd: args.cwd,
            ci: args.ci,
            non_interactive: args.non_interactive,
            auto_yes: args.yes,
            force: args.force,
            config_path: args.config,
        }
    }
}

pub fn print_subcommand_help(subcommand: &str) {
    let mut cmd = Cli::command();
    if let Some(sub) = cmd.find_subcommand_mut(subcommand) {
        sub.print_help().unwrap();
        println!();
    } else {
        tracing::error!("Subcommand `{}` not found.", subcommand);
    }
}
